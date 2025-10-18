use eframe::{egui, Frame, App, NativeOptions};
use crate::config::Config;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use image::io::Reader as ImageReader;
// anyhow::Error not used directly here

pub fn run_gui() {
    // Try to load an icon from assets/extension.ico or assets/extension-256.png
    let options = NativeOptions::default();
    // If user provided a source image but no generated icons, create them here.
    let _ = std::fs::create_dir_all("assets");
    if !std::path::Path::new("assets/extension-256.png").exists() {
        if std::path::Path::new("assets/source.png").exists() {
            // generate multiple sizes from source.png
            match ImageReader::open("assets/source.png") {
                Ok(reader) => match reader.decode() {
                    Ok(src) => {
                    use image::imageops::FilterType;
                    let sizes = [16, 24, 32, 48, 64, 128, 256];
                    for &s in &sizes {
                        let img = src.resize_exact(s, s, FilterType::Lanczos3).to_rgba8();
                        let _ = img.save(format!("assets/extension-{}.png", s));
                    }
                    }
                    Err(e) => eprintln!("failed to decode assets/source.png: {}", e),
                },
                Err(e) => eprintln!("failed to open assets/source.png: {}", e),
            }
        }
    }
    // Note: we no longer set a custom window icon via NativeOptions here because
    // the eframe API for icons may differ between platforms. If desired we can
    // set the icon later with platform-specific APIs.

    let _ = eframe::run_native(
        "Third-Party Extension Installer",
        options,
        Box::new(|cc| {
            // Query system theme and accent color and apply to egui visuals
            if let Ok(dark) = crate::util::system_theme_dark() {
                if dark {
                    cc.egui_ctx.set_visuals(egui::Visuals::dark());
                } else {
                    cc.egui_ctx.set_visuals(egui::Visuals::light());
                }
            }
            if let Ok((r,g,b)) = crate::util::system_accent_color() {
                let accent = egui::Color32::from_rgb(r,g,b);
                let mut visuals = cc.egui_ctx.style().visuals.clone();
                // set selection and hyper-link colors to accent where applicable
                visuals.widgets.active.bg_fill = accent;
                visuals.selection.bg_fill = accent;
                cc.egui_ctx.set_visuals(visuals);
            }
            Box::new(GuiApp::new())
        }),
    );
}

#[derive(Clone)]
struct GuiApp {
    state: Arc<Mutex<State>>,
}

struct State {
    config_path: String,
    plugins: Vec<(String, String)>, // (name, version)
    message: String,
    new_name: String,
    new_repo: String,
    busy: bool,
}

impl GuiApp {
    fn new() -> Self {
        let mut app = Self {
            state: Arc::new(Mutex::new(State {
                config_path: String::new(),
                plugins: vec![],
                message: String::new(),
                new_name: String::new(),
                new_repo: String::new(),
                busy: false,
            })),
        };
        app.refresh_plugins();
        app
    }

    fn open_url(url: &str) {
        // Try to open in default browser; fallback prints to console
        if webbrowser::open(url).is_err() {
            eprintln!("Open URL failed: {}", url);
        }
    }

    fn refresh_plugins(&mut self) {
        let state = self.state.clone();
        thread::spawn(move || {
            let mut s = state.lock().unwrap();
            s.busy = true;
            s.plugins.clear();
            if let Ok(cfg) = Config::new() {
                // Use public getter to avoid accessing private fields
                s.config_path = format!("{}", crate::CONFIG_PATH.display());
                s.plugins = cfg.plugin_list();
            }
            s.busy = false;
        });
    }

    fn run_add(&self, name: String, repo: String, on_complete: impl FnOnce(Result<(), anyhow::Error>) + Send + 'static) {
        thread::spawn(move || {
            let mut cfg = Config::new().unwrap_or_else(|_| Config::init().unwrap_or_else(|_| Config::new().unwrap()));
            // Normalize repo string: accept full URL, owner/repo, or owner/repo.git
            let repo_clean = {
                let r = repo.trim();
                let r = r.strip_prefix("https://github.com/").unwrap_or(r);
                let r = r.strip_suffix(".git").unwrap_or(r);
                r.to_string()
            };
            let r = cfg.try_add(&name, repo_clean, None, None, false);
            if r.is_ok() {
                cfg.save().unwrap_or(());
            }
            on_complete(r.map(|_| ()).map_err(|e| e.into()));
        });
    }

    fn run_update(&self, name: String, on_complete: impl FnOnce(Result<(), anyhow::Error>) + Send + 'static) {
        thread::spawn(move || {
            let mut cfg = Config::new().unwrap();
            let r = cfg.try_update(vec![name.clone()], None, false);
            if r.is_ok() {
                cfg.save().unwrap_or(());
                on_complete(Ok(()));
            } else {
                on_complete(r.map(|_| ()).map_err(|e| e.into()));
            }
        });
    }

    fn run_remove(&self, name: String, on_complete: impl FnOnce(Result<(), anyhow::Error>) + Send + 'static) {
        thread::spawn(move || {
            let mut cfg = Config::new().unwrap();
            let r = cfg.try_remove(vec![name.clone()], false);
            if r.is_ok() {
                cfg.save().unwrap_or(());
                on_complete(Ok(()));
            } else {
                on_complete(r.map(|_| ()).map_err(|e| e.into()));
            }
        });
    }
}

impl App for GuiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        // Snapshot state into local variables so we don't hold the mutex while
        // calling UI closure code (egui may call closures that borrow self).
        let (mut new_name, mut new_repo, mut message, busy, plugins) = {
            let s = self.state.lock().unwrap();
            (
                s.new_name.clone(),
                s.new_repo.clone(),
                s.message.clone(),
                s.busy,
                s.plugins.clone(),
            )
        };

        // Actions requested by the UI this frame
        let mut requested_add: Option<(String, String)> = None;
        let mut requested_refresh = false;
        let mut requested_update: Option<String> = None;
        let mut requested_remove: Option<String> = None;

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading("PowerToys Run Plugin Manager (GUI)");
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Min), |ui| {
                    if ui.button("Plugins List").clicked() {
                        // toggle a simple state value by updating message
                        // we store 'plugins' view state in message for small change
                        let mut s = self.state.lock().unwrap();
                        if s.message == "__show_plugins" { s.message = String::new(); } else { s.message = "__show_plugins".to_string(); }
                    }
                });
            });

            ui.horizontal(|ui| {
                ui.label("Name:");
                ui.text_edit_singleline(&mut new_name);
                ui.label("Repo:");
                ui.text_edit_singleline(&mut new_repo);
                if ui.add_enabled(!busy, egui::Button::new("Add")).clicked() {
                    if new_name.trim().is_empty() || new_repo.trim().is_empty() {
                        message = "Name and repo required".to_string();
                    } else {
                        requested_add = Some((new_name.trim().to_string(), new_repo.trim().to_string()));
                    }
                }
                if ui.add_enabled(!busy, egui::Button::new("Refresh")).clicked() {
                    requested_refresh = true;
                }
            });

            ui.separator();

            if busy {
                ui.label("Busy...");
            }

            // If message equals '__show_plugins' render our static plugin lists
            if message == "__show_plugins" {
                ui.label("Available community plugins (click Download or Install)");
                ui.separator();
                egui::CollapsingHeader::new("General plugins").show(ui, |ui| {
                    egui::ScrollArea::vertical().max_height(300.0).show(ui, |ui| {
                        for p in crate::plugins::GENERAL_PLUGINS {
                            ui.horizontal(|ui| {
                                ui.label(p.name);
                                ui.label(p.author);
                                ui.add(egui::Label::new(p.description).wrap(true).sense(egui::Sense::hover()));
                                if ui.button("Download ZIP").clicked() {
                                    // attempt main zip; user can fallback to repo page if needed
                                    let main_zip = format!("{}/archive/refs/heads/main.zip", p.repo);
                                    Self::open_url(&main_zip);
                                }
                                if ui.button("Install").clicked() {
                                    // Normalize repo to owner/repo
                                    let r = p.repo.trim().strip_prefix("https://github.com/").unwrap_or(p.repo).strip_suffix(".git").unwrap_or(p.repo).to_string();
                                    // call run_add with name and repo shorthand
                                    let nm = p.name.to_string();
                                    let self_clone = self.clone();
                                    self.run_add(nm.clone(), r, move |res| {
                                        let mut st = self_clone.state.lock().unwrap();
                                        match res {
                                            Ok(_) => st.message = format!("Installed {}", nm),
                                            Err(e) => st.message = format!("Failed to install {}: {}", nm, e),
                                        }
                                        st.busy = false;
                                    });
                                }
                            });
                        }
                    });
                });
                egui::CollapsingHeader::new("Extending software plugins").show(ui, |ui| {
                    egui::ScrollArea::vertical().max_height(300.0).show(ui, |ui| {
                        for p in crate::plugins::EXTENDING_PLUGINS {
                            ui.horizontal(|ui| {
                                ui.label(p.name);
                                ui.label(p.author);
                                ui.add(egui::Label::new(p.description).wrap(true));
                                if ui.button("Download ZIP").clicked() {
                                    let main_zip = format!("{}/archive/refs/heads/main.zip", p.repo);
                                    Self::open_url(&main_zip);
                                }
                                if ui.button("Install").clicked() {
                                    let r = p.repo.trim().strip_prefix("https://github.com/").unwrap_or(p.repo).strip_suffix(".git").unwrap_or(p.repo).to_string();
                                    let nm = p.name.to_string();
                                    let self_clone = self.clone();
                                    self.run_add(nm.clone(), r, move |res| {
                                        let mut st = self_clone.state.lock().unwrap();
                                        match res {
                                            Ok(_) => st.message = format!("Installed {}", nm),
                                            Err(e) => st.message = format!("Failed to install {}: {}", nm, e),
                                        }
                                        st.busy = false;
                                    });
                                }
                            });
                        }
                    });
                });
            } else {
                ui.label("Plugins:");
                egui::ScrollArea::vertical().show(ui, |ui| {
                    for (name, version) in &plugins {
                        ui.horizontal(|ui| {
                            ui.label(name);
                            ui.label(version);
                            if ui.button("Update").clicked() {
                                requested_update = Some(name.clone());
                            }
                            if ui.button("Remove").clicked() {
                                requested_remove = Some(name.clone());
                            }
                        });
                        if requested_update.is_some() || requested_remove.is_some() {
                            break;
                        }
                    }
                });
            }

            ui.separator();
            ui.label(&message);
        });

        // Commit local edits back to shared state and trigger actions.
        if requested_add.is_some() {
            let (nm, rb) = requested_add.take().unwrap();
            {
                let mut s = self.state.lock().unwrap();
                s.new_name = new_name.clone();
                s.new_repo = new_repo.clone();
                s.message = format!("Adding {}...", nm);
                s.busy = true;
            }
            let state_clone = self.state.clone();
            self.run_add(nm.clone(), rb.clone(), move |res| {
                let mut st = state_clone.lock().unwrap();
                match res {
                    Ok(_) => st.message = format!("Added {}", nm),
                    Err(e) => st.message = format!("Failed to add {}: {}", nm, e),
                }
                st.busy = false;
            });
        } else if requested_refresh {
            {
                let mut s = self.state.lock().unwrap();
                s.new_name = new_name.clone();
                s.new_repo = new_repo.clone();
                s.message = "Refreshing...".to_string();
                s.busy = true;
            }
            // refresh_plugins spawns its own thread
            self.refresh_plugins();
        } else if let Some(nm) = requested_update {
            {
                let mut s = self.state.lock().unwrap();
                s.new_name = new_name.clone();
                s.new_repo = new_repo.clone();
                s.message = format!("Updating {}...", nm);
                s.busy = true;
            }
            let state_clone = self.state.clone();
            self.run_update(nm.clone(), move |res| {
                let mut st = state_clone.lock().unwrap();
                match res {
                    Ok(_) => st.message = format!("Updated {}", nm),
                    Err(e) => st.message = format!("Failed to update {}: {}", nm, e),
                }
                st.busy = false;
            });
        } else if let Some(nm) = requested_remove {
            {
                let mut s = self.state.lock().unwrap();
                s.new_name = new_name.clone();
                s.new_repo = new_repo.clone();
                s.message = format!("Removing {}...", nm);
                s.busy = true;
            }
            let state_clone = self.state.clone();
            self.run_remove(nm.clone(), move |res| {
                let mut st = state_clone.lock().unwrap();
                match res {
                    Ok(_) => st.message = format!("Removed {}", nm),
                    Err(e) => st.message = format!("Failed to remove {}: {}", nm, e),
                }
                st.busy = false;
            });
        } else {
            // No action; just commit edited fields back to state
            let mut s = self.state.lock().unwrap();
            s.new_name = new_name;
            s.new_repo = new_repo;
            s.message = message;
        }

        // small tick to refresh UI
        ctx.request_repaint_after(Duration::from_millis(100));
    }
}
