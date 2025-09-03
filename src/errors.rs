use thiserror::Error;

#[derive(Error, Debug)]
pub enum LauncherError {
    #[error("Tor no encontrado")]
    TorNotFound,
    #[error("Error lanzando app")]
    AppLaunchError,
    #[error("WinDivert error")]
    DivertError,
}
