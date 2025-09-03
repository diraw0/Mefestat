use std::path::PathBuf;

pub fn tor_path() -> PathBuf {
    PathBuf::from("assets/Tor/tor.exe")
}

pub fn torrc_path() -> PathBuf {
    PathBuf::from("torrc/tor_transparent.conf")
}
