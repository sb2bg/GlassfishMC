use log::error;

pub fn exit_with_err(err: &str) {
    error!("{}", err);
    std::process::exit(1);
}