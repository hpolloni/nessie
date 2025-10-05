// Pixels-based renderer implementation
//
// This renderer uses the `pixels` and `winit` crates to provide
// a simple, pure-Rust rendering solution for the NES emulator.

use pixels::{Pixels, SurfaceTexture};
use winit::{
    dpi::LogicalSize,
    event::{ElementState, Event, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};

use super::{
    framebuffer_to_rgba8888, InputEvent, Key, NESFramebuffer, Renderer, NES_HEIGHT, NES_WIDTH,
};

/// Pixels-based renderer for NES emulation
/// Note: This is a simplified implementation for demonstration
pub struct PixelsRenderer {
    window: Option<Window>,
    pixels: Option<Pixels>,
    event_loop: Option<EventLoop<()>>,
    events: Vec<InputEvent>,
    should_close: bool,
}

#[derive(Debug, thiserror::Error)]
pub enum PixelsRendererError {
    #[error("Event loop error: {0}")]
    EventLoop(String),
    #[error("Pixels error: {0}")]
    Pixels(#[from] pixels::Error),
    #[error("Window creation failed")]
    WindowCreation,
    #[error("Renderer not initialized")]
    NotInitialized,
}

impl Renderer for PixelsRenderer {
    type Error = PixelsRendererError;

    fn new(title: &str) -> Result<Self, Self::Error> {
        Ok(Self {
            window: None,
            pixels: None,
            event_loop: None,
            events: Vec::new(),
            should_close: false,
        })
    }

    fn render_frame(&mut self, framebuffer: &NESFramebuffer) -> Result<(), Self::Error> {
        // Initialize if needed
        if self.pixels.is_none() {
            self.initialize()?;
        }

        if let Some(ref mut pixels) = self.pixels {
            // Convert NES framebuffer to RGBA
            let rgba_data = framebuffer_to_rgba8888(framebuffer);

            // Copy to pixels framebuffer
            let frame = pixels.frame_mut();
            frame.copy_from_slice(&rgba_data);

            // Render
            pixels.render()?;

            // Request redraw
            if let Some(ref window) = self.window {
                window.request_redraw();
            }
        }
        Ok(())
    }

    fn should_close(&self) -> bool {
        self.should_close
    }

    fn poll_events(&mut self) -> Vec<InputEvent> {
        let mut events = Vec::new();
        std::mem::swap(&mut events, &mut self.events);
        events
    }

    fn window_size(&self) -> (u32, u32) {
        self.window
            .as_ref()
            .map(|window| {
                let size = window.inner_size();
                (size.width, size.height)
            })
            .unwrap_or((NES_WIDTH as u32 * 2, NES_HEIGHT as u32 * 2))
    }
}

impl PixelsRenderer {
    fn initialize(&mut self) -> Result<(), PixelsRendererError> {
        let event_loop = EventLoop::new();

        let window = WindowBuilder::new()
            .with_title("Nessie NES Emulator")
            .with_inner_size(LogicalSize::new(
                (NES_WIDTH * 2) as f64,  // 2x scale by default
                (NES_HEIGHT * 2) as f64,
            ))
            .build(&event_loop)
            .map_err(|_| PixelsRendererError::WindowCreation)?;

        // Create pixels context
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);

        let pixels = Pixels::new(NES_WIDTH as u32, NES_HEIGHT as u32, surface_texture)?;

        self.window = Some(window);
        self.pixels = Some(pixels);
        self.event_loop = Some(event_loop);

        Ok(())
    }

    /// Run the event loop - this should be called from main to start rendering
    pub fn run_event_loop(mut self) -> Result<(), PixelsRendererError> {
        let event_loop = self.event_loop.take()
            .ok_or(PixelsRendererError::NotInitialized)?;

        event_loop.run(move |event, _, control_flow| {
            *control_flow = ControlFlow::Wait;

            match event {
                Event::WindowEvent { event, .. } => {
                    match event {
                        WindowEvent::CloseRequested => {
                            self.should_close = true;
                            self.events.push(InputEvent::Close);
                            *control_flow = ControlFlow::Exit;
                        }
                        WindowEvent::Resized(new_size) => {
                            if let Some(ref mut pixels) = self.pixels {
                                let _ = pixels.resize_surface(new_size.width, new_size.height);
                            }
                            self.events.push(InputEvent::Resize(new_size.width, new_size.height));
                        }
                        WindowEvent::KeyboardInput {
                            input,
                            ..
                        } => {
                            if let Some(virtual_keycode) = input.virtual_keycode {
                                if let Some(key) = keycode_to_key(virtual_keycode) {
                                    match input.state {
                                        ElementState::Pressed => self.events.push(InputEvent::KeyDown(key)),
                                        ElementState::Released => self.events.push(InputEvent::KeyUp(key)),
                                    }
                                }
                            }
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        })
    }
}

/// Convert winit VirtualKeyCode to our Key enum
fn keycode_to_key(keycode: VirtualKeyCode) -> Option<Key> {
    match keycode {
        VirtualKeyCode::Up => Some(Key::Up),
        VirtualKeyCode::Down => Some(Key::Down),
        VirtualKeyCode::Left => Some(Key::Left),
        VirtualKeyCode::Right => Some(Key::Right),
        VirtualKeyCode::Z | VirtualKeyCode::X => Some(Key::A),
        VirtualKeyCode::A | VirtualKeyCode::S => Some(Key::B),
        VirtualKeyCode::Return => Some(Key::Start),
        VirtualKeyCode::Space => Some(Key::Select),
        VirtualKeyCode::Escape => Some(Key::Escape),
        VirtualKeyCode::R | VirtualKeyCode::F1 => Some(Key::Reset),
        VirtualKeyCode::P => Some(Key::Pause),
        _ => None,
    }
}

// Note: This renderer implementation is designed to work with an external event loop
// For a complete implementation, you would typically run the event loop like this:
//
// ```rust
// let mut renderer = PixelsRenderer::new("Nessie")?;
// let event_loop = renderer.event_loop.take().unwrap();
// let mut app_state = renderer.app_state.take().unwrap();
//
// event_loop.run_app(&mut app_state)?;
// ```