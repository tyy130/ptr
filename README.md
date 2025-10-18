
# PowerToys Run Plugin Manager

![preview](https://github.com/user-attachments/assets/94489f6f-0301-4427-8c44-2f801201c64f)

Install and manage any PowerToys Run plugin released on GitHub with single command line interface.

## Installation

### Download

Download binary from [releases](https://github.com/8LWXpg/ptr/releases) page.

### Using `cargo-binstall`

# PowerToys Run Plugin Manager

![preview](https://github.com/user-attachments/assets/94489f6f-0301-4427-8c44-2f801201c64f)

Install and manage any PowerToys Run plugin released on GitHub with a simple command line interface. A lightweight GUI front-end is included that shells out to the CLI.

## Installation

### Download (recommended for end users)

Download the prebuilt ZIP from the Releases page and extract. The release package should include:

- `ptr.exe` (CLI)
- `ptr-gui.exe` (GUI)

Run `ptr-gui.exe` to open the GUI, or run `ptr.exe` from PowerShell / command prompt.

### Using `cargo-binstall` (developer / power users)

```powershell
cargo binstall --git https://github.com/8LWXpg/ptr ptr
```

### Build from source

You need Rust toolchain installed (rustup). Then:

```powershell
Set-Location 'C:\dev\ptr\ptr-repo'
cargo install --git https://github.com/8LWXpg/ptr.git
```

## GUI (Graphical User Interface)

The GUI is a thin front-end built with `eframe`/`egui`. It shells out to the `ptr` CLI for actions and reads the configuration at `%LOCALAPPDATA%\\Microsoft\\PowerToys\\PowerToys Run\\Plugins\\version.toml` to list plugins.

How to use the GUI (Windows):

1. Download the release ZIP and extract.
2. Double-click `ptr-gui.exe` or run from PowerShell:

```powershell
.\\ptr-gui.exe
```

If `ptr` (the CLI) is not on PATH the GUI will try to call `ptr` in the same folder. Place `ptr.exe` next to `ptr-gui.exe` if necessary.

### Developer build (GUI)

To build the GUI locally (requires Rust):

```powershell
Set-Location 'C:\dev\ptr\ptr-repo'
cargo build --release --bin ptr     # build CLI
cargo build --release --bin gui     # build GUI (produces ptr-gui.exe)
```

## Packaging and distribution recommendations

For easy user installs on Windows:

- Build both `ptr.exe` and `ptr-gui.exe` on CI per target (e.g., windows-x86_64). Create a zip that contains both files and attach it to a GitHub Release.
- Optionally publish a Chocolatey or Scoop package for single-command install on Windows.
- You can also provide a signed installer (MSI) if you want an installer-style UX.

For simple Rust-first distribution, publish the CLI to `cargo-binstall` and include the GUI if you prefer. Note that `cargo-binstall` focuses on CLI binaries and not Windows installers.

## How the GUI is implemented

- The GUI uses `eframe`/`egui`.
- It reads the existing `version.toml` to list plugins and shells out to `ptr add/update/remove` for operations. This keeps business logic in the CLI (`src/config.rs` and `src/util.rs`) and makes the GUI a thin control surface.

If you prefer, we can refactor the project to expose the plugin management functionality as a library crate (e.g., `ptr-lib`) and then call it directly from the GUI; that avoids the need for an external `ptr` executable and enables tighter integration.

## Quick Start (CLI)

Install a plugin with `add`:

 # PowerToys Run Plugin Manager

![preview](https://github.com/user-attachments/assets/94489f6f-0301-4427-8c44-2f801201c64f)

Manage PowerToys Run plugins from GitHub. This repository provides a single `ptr` executable that works as both CLI and GUI:

- CLI: `ptr <command>` (existing commands like `add`, `update`, `remove`, etc.)
- GUI: `ptr gui` launches a graphical front-end built with eframe/egui (runs in the same executable).

## Quick start

Download the Windows release ZIP from the Releases page, extract, and run:

```powershell
# Runs the GUI
.\\ptr.exe gui

# Or use CLI
.\\ptr.exe add MyPlugin 8LWXpg/PowerToysRun-MyPlugin
```

If you prefer to build locally (developers):

```powershell
Set-Location 'C:\dev\ptr\ptr-repo'
cargo build --release --bin ptr
# PowerToys Run — Third‑Party Plugins Installer

PowerToys Run Third‑Party Plugins Installer ("PTR Installer") makes it simple to discover and install third‑party PowerToys Run plugins without manually cloning repositories or fiddling with files.

This project provides a single executable that works as both a CLI and a GUI. Use the CLI for automation and scripting, or run the GUI for a simple install experience that respects your Windows theme and accent.

Built by tacticdev.com — help@tacticdev.com

---

Key features
- Install, update and remove third‑party PowerToys Run plugins from GitHub releases or repository references.
- Single executable that exposes both CLI and GUI entrypoints (`ptr` with a `gui` subcommand).
- GUI adapts to Windows dark mode and accent color for native look-and-feel.
- Supports creating properly sized icons for the GUI and installer.
- Packaging helpers and a Windows release workflow included for CI-based builds and signing.

Quick links
- Releases: See GitHub Releases for prebuilt Windows zips
- Issues & Support: Open an issue on this repository or email help@tacticdev.com

---

Getting started — end users

1. Download the latest Windows release ZIP from the Releases page and extract it.
2. Run the included executable and open the GUI:

```powershell
# runs the GUI from the single executable
.\ptr.exe gui
```

3. In the GUI, choose a third‑party provider from the dropdown or paste a GitHub repo (owner/name). Click Install.

That's it — PTR Installer handles downloading, installing, and registering the plugin with PowerToys Run.

CLI examples (power users)

Install a plugin by shorthand (owner/repo):

```powershell
.\ptr.exe add MyPluginOwner/PowerToysRun-MyPlugin
```

Install a plugin and accept defaults interactively:

```powershell
.\ptr.exe add
# follow prompts
```

List installed plugins:

```powershell
.\ptr.exe list
```

Update or remove:

```powershell
.\ptr.exe update MyPlugin
.\ptr.exe remove MyPlugin
```

Developer: build from source

Requirements
- Rust toolchain (rustup)
- Windows (development and runtime tested on recent Windows 10 / 11)

Build steps:

```powershell
Set-Location 'C:\dev\ptr\ptr-repo'
cargo build --release --bin ptr
# Launch GUI from the built binary
.\target\release\ptr.exe gui
```

If you prefer separate binaries for distribution, CI packaging currently builds a single `ptr.exe` that contains both CLI and GUI modes.

Packaging and CI

- A GitHub Actions workflow is included to build Windows release artifacts and create a ZIP for distribution. See `.github/workflows/windows-release.yml`.
- The workflow supports optional code signing using a PFX certificate supplied by repository secrets or via Azure Key Vault.

Icons and assets

- The repository includes a runtime mechanism to prefer an ICO or PNG icon in `assets/` and can also generate sized PNGs at runtime.
- Generated icons have been added to `assets/` for convenience; the large `assets/source.png` is ignored in the repository (`.gitignore`) to keep the repo small.

Contributing

- Pull requests are welcome. For larger changes, open an issue first to discuss the design.
- Tests: adding unit tests around the core plugin management in `src/config.rs` and library exports in `src/lib.rs` is encouraged.

Support & contact

- Built by tacticdev.com — for help email help@tacticdev.com
- If you find a bug, please open an issue and include the following:
	- Windows version
	- `ptr --version` output
	- Steps to reproduce

License

This project is provided under the MIT license (see `LICENSE` file if present).

---

If you want a tailored installer (MSI/NSIS) or automatic publishing to Chocolatey/Scoop, I can add those CI steps and sign the builds — tell me which distribution channel you prefer.

Plugins catalog
---------------

We provide a curated plugin index in PLUGINS.md. The GUI includes a Catalog page that renders this list and offers oneclick Download and Install actions for each plugin. See PLUGINS.md in this repository for details and links.

