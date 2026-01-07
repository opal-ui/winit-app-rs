use winit::{
    event_loop::{ControlFlow, EventLoop},
    window::WindowAttributes,
};

use crate::{
    WinitAppResult,
    app_listener::{AppListener, AppWindowEvent},
};

/// Create a new `winit` application.
///
/// # Example
///
/// ```
/// use winit::{event::WindowEvent, window::WindowAttributes};
/// use winit_app::{app_listener::AppWindowEvent, application::Application};
///
/// fn launch_app() -> Result<(), Box<dyn std::error::Error>> {
///     let winit_app = Application::new();
///
///     winit_app.run(
///        WindowAttributes::default().with_title("Sample"),
///        |app_window_event| match app_window_event {
///            _ => {
///                     // TODO: Handle those events
///            }
///        },
///    )?;
///    Ok(())
/// }
/// ```
#[derive(Default)]
pub struct Application {
    control_flow: ControlFlow,
}

impl<'a> Application {
    pub fn new() -> Self {
        Self {
            control_flow: ControlFlow::Wait,
        }
    }

    /// Sets the control flow of this application
    ///
    /// `ControlFlow::Poll` continuously runs the event loop, even if the OS hasn't
    /// dispatched any events. This is ideal for games and similar applications.
    ///
    /// `ControlFlow::Wait` pauses the event loop if no events are available to process.
    /// This is ideal for non-game applications that only update in response to user
    /// input, and uses significantly less power/CPU time than ControlFlow::Poll.
    pub fn with_control_flow(&mut self, control_flow: ControlFlow) -> &Self {
        self.control_flow = control_flow;
        self
    }

    pub fn run<F>(&'a self, window_attributes: WindowAttributes, listener: F) -> WinitAppResult<()>
    where
        F: FnMut(AppWindowEvent) + 'static,
    {
        // Create a new event loop.
        let event_loop = EventLoop::new()?;

        // Configure settings before launching.
        event_loop.set_control_flow(self.control_flow);

        let app_listener = AppListener::new(window_attributes, listener);
        // Launch and begin running the event loop.
        event_loop.run_app(app_listener)?;

        Ok(())
    }
}
