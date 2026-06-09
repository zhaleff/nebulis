pub mod lua_bridge;

use anyhow::Result;
use std::path::Path;

pub fn start(config_path: &Path) -> Result<()> {
    lua_bridge::load(config_path)
}
