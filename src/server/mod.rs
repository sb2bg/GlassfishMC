use std::time::Instant;

use log::{debug, info, trace};

use data_manager::config_manager::Config;

use crate::server::plugin_manager::PluginManager;
use crate::server::tick_loop::TickLoop;

mod state;
pub mod packet;
pub mod plugin_manager;
pub mod data_manager;
mod tick_loop;
mod exit_with_err;

const VERSION: &str = "1.17.1";
const PROTOCOL_VERSION: u32 = 756;
const DATA_VERSION: u32 = 2730;

pub struct Server {
    config: Config,
    start_time: Instant,
    plugin_manager: PluginManager,
    tps: f32,
}

impl Server {
    pub fn new(config: Config) -> Self {
        Self { config, tps: 0.0, start_time: Instant::now(), plugin_manager: PluginManager::new() }
    }

    pub fn start(&mut self) {
        let host = self.config.get_host();
        let port = self.config.get_port();
        let cores = if self.config.get_multi_threaded() { self.config.get_max_threads() } else { 1 };

        info!("Version {}, Protocol {}, Data {}", VERSION, PROTOCOL_VERSION, DATA_VERSION);
        info!("Starting server on {}:{} with {} core(s) and {} tick rate", host, port, cores, self.config.get_tick_rate());

        self.plugin_manager.load_plugins();

        self.tick_loop();
    }

    fn tick_loop(&mut self) {
        // may have to use channels to update server tps
        let mut tick_loop = TickLoop::new(self);

        tick_loop.run();
    }

    pub fn get_tps(&self) -> f32 {
        self.tps
    }

    pub fn set_tps(&mut self, tps: f32) {
        self.tps = tps;
    }

    pub fn get_plugin_manager(&self) -> &PluginManager {
        &self.plugin_manager
    }

    pub fn get_config(&self) -> &Config {
        &self.config
    }
}