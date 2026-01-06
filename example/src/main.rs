use log::LevelFilter;
use simple_logger::SimpleLogger;
use winit::{event::WindowEvent, window::WindowAttributes};
use winit_app::{app_listener::AppWindowEvent, application::Application};

use log::info;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    SimpleLogger::new()
        // Set default global logging level to 'info'
        .with_level(LevelFilter::Info)
        // Override the level for a specific module to 'warn'
        // You can add more module-specific overrides
        .with_module_level("wgpu", LevelFilter::Warn)
        // Optionally, still allow configuration via RUST_LOG env var
        .init()?;

    let winit_app = Application::new();

    winit_app.run(
        WindowAttributes::default().with_title("Sample"),
        |app_window_event| match app_window_event {
            AppWindowEvent::NewWindow(_window) => {
                // TODO: Do something with this window
                info!("Created a new window");
            }
            AppWindowEvent::OnWindowEvent(event, event_loop) => match event {
                WindowEvent::CloseRequested => {
                    event_loop.exit();
                }
                _ => {
                    info!("Received a new window event {:?}", event);
                }
            },
        },
    )?;
    Ok(())
}
