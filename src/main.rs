use log::info;

const PKG_NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");
const AUTHORS: &str = env!("CARGO_PKG_AUTHORS");

fn main() {
    // env_logger config
    env_logger::Builder::from_default_env()
        .format_timestamp_millis()
        .init();

    info!("{} {}", PKG_NAME, VERSION);
}