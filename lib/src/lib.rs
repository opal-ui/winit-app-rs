//!
//! This library helps get started with the `winit` library , the rust windowing toolkit to get started.
//!
//! ## Usage
//!
//! Create a new winit application using this library to get started.
//!  
//! ```
//! use winit::{event::WindowEvent, window::WindowAttributes};
//! use winit_app::{app_listener::AppWindowEvent, application::Application};
//!
//! fn launch_app() -> Result<(), Box<dyn std::error::Error>> {
//!     let winit_app = Application::new();
//!
//!     winit_app.run(
//!        WindowAttributes::default().with_title("Sample"),
//!        |app_window_event| match app_window_event {
//!            AppWindowEvent::NewWindow(_window) => {
//!                // TODO: Do something with this window
//!                
//!            }
//!            AppWindowEvent::OnWindowEvent(event, event_loop) => match event {
//!                WindowEvent::CloseRequested => {
//!                   // Just a default handler to exit the event_loop when window is being closed  
//!                    event_loop.exit();
//!                }
//!                _ => {
//!                    // Handle all other window events
//!                }
//!            },
//!        },
//!    )?;
//!    Ok(())
//! }
//! ```
//!
//!
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
