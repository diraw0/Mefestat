use windivert::{WinDivert, WinDivertLayer, WinDivertFlags};
use anyhow::{Result, anyhow};
use std::thread;

pub fn start_divert(pid: u32) -> Result<()> {
    let filter = format!("outbound and ip and tcp and processid {}", pid);

    let mut handle = WinDivert::open(&filter, WinDivertLayer::Network, 0, WinDivertFlags::DEFAULT)
        .map_err(|e| anyhow!("Error abriendo WinDivert: {}", e))?;

    thread::spawn(move || {
        let mut packet = [0u8; 2000];
        while let Ok(size) = handle.recv(&mut packet) {
            let _ = handle.send(&packet[..size]);
        }
    });

    Ok(())
}
