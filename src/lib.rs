pub mod config;
pub mod polling;
pub mod util;
pub mod gui;
pub mod plugins;

use std::{env, path::PathBuf, sync::LazyLock};

/// Path to PowerToys Run Plugins directory.
pub static PLUGIN_PATH: LazyLock<PathBuf> = LazyLock::new(|| {
    PathBuf::from(&env::var("LOCALAPPDATA").unwrap()).join(r"Microsoft\PowerToys\PowerToys Run\Plugins")
});

/// Path to the config file.
pub static CONFIG_PATH: LazyLock<PathBuf> = LazyLock::new(|| {
    PathBuf::from(&env::var("LOCALAPPDATA").unwrap())
        .join(r"Microsoft\PowerToys\PowerToys Run\Plugins\version.toml")
});
