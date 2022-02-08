use env_logger::Env;
use log::{debug, error, info};

fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or("trace")).init();
    info!("Hello, info log is here!");
    debug!("Hello, debug log is here!");
    error!("Hello, error log is here!");
}
