pub mod app_listener;
pub mod application;

use thiserror::Error;
use winit::error::EventLoopError;

/// WinitAppError indicates the possible errors from the winit framework
#[derive(Error, Debug)]
pub enum WinitAppError {
    #[error(transparent)]
    EventLoopError(#[from] EventLoopError),
    #[error(transparent)]
    WinitOSError(#[from] winit::error::OsError),
    #[error(transparent)]
    WinitRequestError(#[from] winit::error::RequestError),
}

pub type WinitAppResult<T> = Result<T, WinitAppError>;
