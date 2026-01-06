use log::warn;
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::ActiveEventLoop;
use winit::window::{Window, WindowAttributes, WindowId};

pub enum AppWindowEvent<'a> {
    NewWindow(Box<dyn Window>),
    OnWindowEvent(WindowEvent, &'a dyn ActiveEventLoop),
}

pub(crate) struct AppListener<F>
where
    F: FnMut(AppWindowEvent),
{
    fn_on_listen_window: F,

    window_attributes: WindowAttributes,
}

impl<F> AppListener<F>
where
    F: FnMut(AppWindowEvent),
{
    pub(crate) fn new(window_attributes: WindowAttributes, fn_on_listen_window: F) -> Self
    where
        F: FnMut(AppWindowEvent),
    {
        Self {
            window_attributes,
            fn_on_listen_window,
        }
    }
}

impl<F> ApplicationHandler for AppListener<F>
where
    F: FnMut(AppWindowEvent),
{
    fn can_create_surfaces(&mut self, event_loop: &dyn ActiveEventLoop) {
        // The event loop has launched, and we can initialize our UI state.

        // Create a simple window with default attributes.
        match event_loop.create_window(self.window_attributes.clone()) {
            Ok(window) => {
                (self.fn_on_listen_window)(AppWindowEvent::NewWindow(window));
            }
            Err(err) => {
                warn!("Error creating a new window {:?}", err);
            }
        }
    }

    fn window_event(
        &mut self,
        event_loop: &dyn ActiveEventLoop,
        _id: WindowId,
        event: WindowEvent,
    ) {
        (self.fn_on_listen_window)(AppWindowEvent::OnWindowEvent(event, event_loop));
    }
}
