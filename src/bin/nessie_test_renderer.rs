// Test renderer frontend
//
// This is a simple test application that demonstrates the rendering system
// by showing a test pattern and responding to keyboard input.

use pixels::{Pixels, SurfaceTexture};
use winit::{
    dpi::LogicalSize,
    event::{ElementState, Event, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

use nessie::{
    ppu::PPU,
    rendering::{framebuffer_to_rgba8888, NES_HEIGHT, NES_WIDTH},
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    println!("Nessie Test Renderer");
    println!("Controls:");
    println!("  Up Arrow: White screen");
    println!("  Down Arrow: Black screen");
    println!("  Left Arrow: Red screen");
    println!("  Right Arrow: Green screen");
    println!("  Space: Change test pattern");
    println!("  Escape: Exit");
    println!();

    // Create PPU
    let mut ppu = PPU::new();
    ppu.render_test_pattern();

    let mut pattern_mode = 0;

    // Create window and event loop
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Nessie Test Renderer")
        .with_inner_size(LogicalSize::new(
            (NES_WIDTH * 3) as f64,  // 3x scale
            (NES_HEIGHT * 3) as f64,
        ))
        .build(&event_loop)?;

    // Create pixels context
    let window_size = window.inner_size();
    let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
    let mut pixels = Pixels::new(NES_WIDTH as u32, NES_HEIGHT as u32, surface_texture)?;

    println!("Window created - press keys to interact");

    // Run event loop
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => {
                    println!("Window closed");
                    *control_flow = ControlFlow::Exit;
                }
                WindowEvent::KeyboardInput { input, .. } => {
                    if let Some(keycode) = input.virtual_keycode {
                        if input.state == ElementState::Pressed {
                            match keycode {
                                VirtualKeyCode::Escape => {
                                    println!("Escape pressed - exiting");
                                    *control_flow = ControlFlow::Exit;
                                }
                                VirtualKeyCode::Up => {
                                    println!("Up arrow pressed - White screen");
                                    ppu.render_solid_color(0x30); // White
                                    window.request_redraw();
                                }
                                VirtualKeyCode::Down => {
                                    println!("Down arrow pressed - Black screen");
                                    ppu.render_solid_color(0x0F); // Black
                                    window.request_redraw();
                                }
                                VirtualKeyCode::Left => {
                                    println!("Left arrow pressed - Red screen");
                                    ppu.render_solid_color(0x16); // Red
                                    window.request_redraw();
                                }
                                VirtualKeyCode::Right => {
                                    println!("Right arrow pressed - Green screen");
                                    ppu.render_solid_color(0x1A); // Green
                                    window.request_redraw();
                                }
                                VirtualKeyCode::Space => {
                                    println!("Space pressed - changing pattern");
                                    pattern_mode = (pattern_mode + 1) % 3;
                                    match pattern_mode {
                                        0 => {
                                            println!("  -> Checkerboard pattern");
                                            ppu.render_test_pattern();
                                        }
                                        1 => {
                                            println!("  -> Blue screen");
                                            ppu.render_solid_color(0x12);
                                        }
                                        2 => {
                                            println!("  -> Yellow screen");
                                            ppu.render_solid_color(0x38);
                                        }
                                        _ => unreachable!(),
                                    }
                                    window.request_redraw();
                                }
                                _ => {}
                            }
                        }
                    }
                }
                WindowEvent::Resized(new_size) => {
                    println!("Window resized to {}x{}", new_size.width, new_size.height);
                    if let Err(err) = pixels.resize_surface(new_size.width, new_size.height) {
                        eprintln!("Failed to resize: {}", err);
                        *control_flow = ControlFlow::Exit;
                    }
                }
                _ => {}
            },
            Event::RedrawRequested(_) => {
                // Convert framebuffer and render
                let rgba_data = framebuffer_to_rgba8888(ppu.framebuffer());
                let frame = pixels.frame_mut();
                frame.copy_from_slice(&rgba_data);

                if let Err(err) = pixels.render() {
                    eprintln!("Render error: {}", err);
                    *control_flow = ControlFlow::Exit;
                }
            }
            Event::MainEventsCleared => {
                // Request redraw on each frame to keep the window responsive
                window.request_redraw();
            }
            _ => {}
        }
    });
}
