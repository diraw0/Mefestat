use std::process::{Command, Child};
use std::path::PathBuf;
use anyhow::{Result, anyhow};

pub struct TorManager {
    pub tor_process: Option<Child>,
}

impl TorManager {
    pub fn new() -> Self {
        Self { tor_process: None }
    }

    pub fn start_tor(&mut self) -> Result<()> {
        if self.tor_process.is_some() {
            return Ok(());
        }

        let tor_path = PathBuf::from("assets/Tor/tor.exe");
        if !tor_path.exists() {
            return Err(anyhow!("tor.exe no encontrado en assets/Tor/"));
        }

        let child = Command::new(tor_path)
            .spawn()
            .map_err(|e| anyhow!("No se pudo iniciar Tor: {}", e))?;

        self.tor_process = Some(child);
        Ok(())
    }

    pub fn stop_tor(&mut self) {
        if let Some(child) = &mut self.tor_process {
            let _ = child.kill();
        }
        self.tor_process = None;
    }
}
