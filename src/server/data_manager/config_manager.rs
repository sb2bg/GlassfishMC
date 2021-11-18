use log::{error, warn};
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::Write;

#[derive(Deserialize, Serialize)]
pub struct Config {
    max_player_count: u32,
    multi_threaded: bool,
    max_threads: u16,
    host: String,
    port: u16,
    tick_rate: u32,
    auto_save_interval: u32,
    debug: bool,
}

impl Config {
    pub fn load() -> Self {
        match fs::read_to_string("config.toml") {
            Ok(s) => match toml::from_str(s.as_str()) {
                Ok(c) => c,
                Err(e) => {
                    error!("Error parsing config_manager file: {}", e);
                    warn!("Using default file, config_manager file was not saved");
                    Config::create_default_config()
                }
            },
            Err(_) => {
                warn!("No config_manager file found, using default config_manager");
                Config::create_default_config()
            }
        }
    }

    fn create_default_config() -> Config {
        let config = Config {
            max_player_count: 10,
            multi_threaded: false,
            max_threads: 1,
            host: String::from("localhost"),
            port: 25565,
            tick_rate: 20,
            auto_save_interval: 30,
            debug: false,
        };

        match fs::File::create("config.toml") {
            Ok(mut f) => {
                f.write_all(toml::to_string(&config).unwrap().as_bytes()).unwrap();
            }
            Err(e) => {
                error!("Error creating config_manager file: {}", e);
                warn!("Using default file, config_manager file was not saved");
            }
        };

        config
    }

    pub fn get_max_player_count(&self) -> u32 {
        self.max_player_count
    }

    pub fn get_multi_threaded(&self) -> bool {
        self.multi_threaded
    }

    pub fn get_max_threads(&self) -> u16 {
        self.max_threads
    }

    pub fn get_host(&self) -> String {
        self.host.clone()
    }

    pub fn get_port(&self) -> u16 {
        self.port
    }

    pub fn get_tick_rate(&self) -> u32 {
        self.tick_rate
    }

    pub fn get_auto_save_interval(&self) -> u32 {
        self.auto_save_interval
    }

    pub fn get_debug(&self) -> bool {
        self.debug
    }
}