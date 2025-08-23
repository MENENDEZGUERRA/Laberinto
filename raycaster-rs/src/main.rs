use anyhow::Result;
use env_logger;
use log::{error, info};

mod app;

fn main() -> Result<()> {
    env_logger::init();
    if let Err(e) = app::run() {
        error!("Fatal error: {e:?}");
        std::process::exit(1);
    }
    Ok(())
}
