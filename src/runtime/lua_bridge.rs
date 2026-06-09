use anyhow::{Context, Result};
use mlua::prelude::*;
use std::path::Path;
use tracing::info;

pub fn load(config_path: &Path) -> Result<()> {
    let lua = Lua::new();

    info!("Loading config: {}", config_path.display());

    let source = std::fs::read_to_string(config_path)
        .with_context(|| format!("Cannot read {}", config_path.display()))?;

    lua.load(&source)
        .exec()
        .map_err(|e| anyhow::anyhow!("Lua error: {}", e))?;

    Ok(())
}
