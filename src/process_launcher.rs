use std::path::Path;
use std::process::Command;
use anyhow::{Result, anyhow};

pub fn launch_app(path: &Path) -> Result<u32> {
    if !path.exists() {
        return Err(anyhow!("Archivo no encontrado: {}", path.display()));
    }

    let child = Command::new(path)
        .spawn()
        .map_err(|e| anyhow!("Error lanzando la app: {}", e))?;

    Ok(child.id())
}
{\rtf1}