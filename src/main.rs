use std::env;
use std::process::exit;

use fern::colors::Color;
use git_version::git_version;
use log::{debug, error, info};

use crate::server::data_manager::config_manager::Config;
use crate::server::Server;

mod server;

const PKG_NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");
const AUTHORS: &str = env!("CARGO_PKG_AUTHORS");

fn main() {
    let config = Config::load();

    let level: log::LevelFilter = match env::var("GLASSFISH_LOG_LEVEL") {
        Ok(level) => match level.as_str() {
            "trace" => log::LevelFilter::Trace,
            "debug" => log::LevelFilter::Debug,
            "info" => log::LevelFilter::Info,
            "warn" => log::LevelFilter::Warn,
            "error" => log::LevelFilter::Error,
            _ => {
                eprintln!("Fatal Error: Invalid log level: {}", level);
                exit(1);
            }
        },
        Err(_) => if config.get_debug() { log::LevelFilter::Debug } else { log::LevelFilter::Info },
    };

    match setup_logger(level) {
        Ok(_) => {}
        Err(err) => {
            println!("Error initializing fern {}", err);
            exit(1);
        }
    };

    info!("{} {} by {}", PKG_NAME, VERSION, AUTHORS);
    debug!("Debugging enabled. If you don't want to see debug messages, disable it in config_manager.toml");
    info!("Config loaded and logging enabled with level {}", level);

    std::panic::set_hook(Box::new(|panic_info| {
        error!("Unknown Error, please report as an issue on GitHub: '{}:v_{},gh={}'", panic_info.to_string(), VERSION, git_version!());
    }));

    let mut server = Server::new(config);
    server.start();
}

fn setup_logger(level: log::LevelFilter) -> Result<(), fern::InitError> {
    let colors = fern::colors::ColoredLevelConfig::new()
        .error(Color::Red)
        .warn(Color::Yellow)
        .info(Color::White)
        .debug(Color::BrightBlack)
        .trace(Color::Magenta);


    fern::Dispatch::new()
        .chain(fern::Dispatch::new()
            .format(move |out, message, record| {
                out.finish(format_args!(
                    "[{} {}] {}{}\x1B[0m",
                    record.level(),
                    chrono::Local::now().format("%H:%M:%S"),
                    format_args!(
                        "\x1B[{}m",
                        colors.get_color(&record.level()).to_fg_str()
                    ),
                    message
                ))
            })
            .level(level)
            .chain(std::io::stdout()))
        .chain(fern::Dispatch::new()
            .format(|out, message, record| {
                out.finish(format_args!(
                    "[{} {}] {}",
                    record.level(),
                    chrono::Local::now().format("%H:%M:%S"),
                    message
                ))
            })
            .level(log::LevelFilter::Debug)
            .chain(fern::log_file("output.log")?))
        .apply()?;
    Ok(())
}