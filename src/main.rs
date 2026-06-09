mod runtime;

use anyhow::Result;
use tracing::info;

fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    info!("Nebulis starting...");

    let config_path = dirs::config_dir()
        .unwrap_or_else(|| std::path::PathBuf::from("."))
        .join("nebulis/init.lua");

    runtime::start(&config_path)?;

    Ok(())
}
