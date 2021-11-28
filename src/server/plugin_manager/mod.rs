use std::fs;
use std::fs::{read, ReadDir};
use std::path::{Path, PathBuf};

use log::{debug, error, info};

use crate::Server;
use crate::server::exit_with_err::exit_with_err;

pub struct PluginManager {}

impl PluginManager {
    pub fn new() -> Self {
        Self {}
    }

    pub fn load_plugins(&self) {
        let paths = Self::find_plugins();
    }

    fn find_plugins() -> Vec<PathBuf> {
        let entries = match fs::read_dir(Self::init_plugin_dir()) {
            Ok(entries) => { entries }
            Err(e) => {
                exit_with_err(format!("Failed to read plugin folder: {}", e).as_str());
                panic!() // hack to avoid incompatible arm error, this will never be reached
            }
        };

        for entry in entries {
            if let Ok(entry) = entry {
                if entry.path().extension().unwrap() == "dll" {
                    info!("Found plugin: {}", entry.path().display());
                }
            }
        }

        vec![]
    }

    fn init_plugin_dir<'a>() -> &'a Path {
        let plugin_dir = Path::new("plugins");

        if !plugin_dir.exists() {
            info!("Plugin folder not found, creating one");

            if let Err(e) = fs::create_dir(plugin_dir) {
                exit_with_err(format!("Failed to create plugin directory: {}", e).as_str());
            }
        } else {
            info!("Plugin folder found");
        };

        plugin_dir
    }
}

pub trait Plugin {
    fn name(&self) -> &str;
    fn description(&self) -> &str;
    fn author(&self) -> &str;
    fn version(&self) -> &str;
    fn on_plugin_load(&self, server: Server) {}
    fn on_plugin_unload(&self) {}
}