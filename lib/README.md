[`winit_app`](https://crates.io/crates/winit_app) [![crates.io](https://img.shields.io/crates/v/winit_app.svg)](https://crates.io/crates/winit_app)
# Winit App Starter Project

```toml
[dependencies]
winit_app = "0.31.1"
```

This Rust library `winit_app` represents the code to get started with `winit` Rust windowing library.

(To see more details about winit see here  at - https://github.com/rust-windowing/winit )

This project can be used as a start quick launching pad based on the `winit` library.



## Usage

```rust
use winit::{event::WindowEvent, window::WindowAttributes};
use winit_app::{app_listener::AppWindowEvent, application::Application};

fn launch_app() -> Result<(), Box<dyn std::error::Error>> {
    let winit_app = Application::new();

    winit_app.run(
       WindowAttributes::default().with_title("Sample"),
       |app_window_event| match app_window_event {
           AppWindowEvent::NewWindow(_window) => {
               // TODO: Do something with this window
                
           }
           AppWindowEvent::OnWindowEvent(event, event_loop) => match event {
               WindowEvent::CloseRequested => {
                  // Just a default handler to exit the event_loop when window is being closed  
                   event_loop.exit();
               }
               _ => {
                   // Handle all other window events
               }
           },
       },
   )?;
   Ok(())
}


```

## Credits

* Winit Documentation and Examples

https://docs.rs/winit/latest/winit/