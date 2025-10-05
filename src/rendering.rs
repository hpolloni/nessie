// Rendering subsystem for the NES emulator
//
// This module provides a trait-based abstraction for rendering that allows
// easy switching between different graphics backends (pixels, SDL2, etc.)

pub mod pixels_renderer;

use std::error::Error;

/// NES display constants
pub const NES_WIDTH: usize = 256;
pub const NES_HEIGHT: usize = 240;

/// Standard NES color palette
/// These are the 64 colors that the NES PPU can display
pub const NES_PALETTE: [u32; 64] = [
    0x666666, 0x002A88, 0x1412A7, 0x3B00A4, 0x5C007E, 0x6E0040, 0x6C0600, 0x561D00,
    0x333500, 0x0B4800, 0x005200, 0x004F08, 0x00404D, 0x000000, 0x000000, 0x000000,
    0xADADAD, 0x155FD9, 0x4240FF, 0x7527FE, 0xA01ACC, 0xB71E7B, 0xB53120, 0x994E00,
    0x6B6D00, 0x388700, 0x0C9300, 0x008F32, 0x007C8D, 0x000000, 0x000000, 0x000000,
    0xFFFEFF, 0x64B0FF, 0x9290FF, 0xC676FF, 0xF36AFF, 0xFE6ECC, 0xFE8170, 0xEA9E22,
    0xBCBE00, 0x88D800, 0x5CE430, 0x45E082, 0x48CDDE, 0x4F4F4F, 0x000000, 0x000000,
    0xFFFEFF, 0xC0DFFF, 0xD3D2FF, 0xE8C8FF, 0xFBC2FF, 0xFEC4EA, 0xFECCC5, 0xF7D8A5,
    0xE4E594, 0xCFEF96, 0xBDF4AB, 0xB3F3CC, 0xB5EBF2, 0xB8B8B8, 0x000000, 0x000000,
];

/// Framebuffer type for NES display
/// Each pixel is represented as a palette index (0-63)
pub type NESFramebuffer = [u8; NES_WIDTH * NES_HEIGHT];

/// Trait for rendering backends
///
/// This abstraction allows us to support different rendering systems
/// while keeping the same interface for the emulator core.
pub trait Renderer {
    type Error: Error + Send + Sync + 'static;

    /// Initialize the renderer with a window title
    fn new(title: &str) -> Result<Self, Self::Error>
    where
        Self: Sized;

    /// Render a frame from NES palette indices to the screen
    ///
    /// The framebuffer contains palette indices (0-63) that should be
    /// converted to RGB colors using the NES_PALETTE.
    fn render_frame(&mut self, framebuffer: &NESFramebuffer) -> Result<(), Self::Error>;

    /// Check if the window should close (user clicked X, pressed ESC, etc.)
    fn should_close(&self) -> bool;

    /// Poll for events (input, window events, etc.)
    /// Returns any key presses or other input events
    fn poll_events(&mut self) -> Vec<InputEvent>;

    /// Get the current window size (for scaling calculations)
    fn window_size(&self) -> (u32, u32);
}

/// Input events from the rendering system
#[derive(Debug, Clone, PartialEq)]
pub enum InputEvent {
    /// Window close requested
    Close,
    /// Key pressed
    KeyDown(Key),
    /// Key released
    KeyUp(Key),
    /// Window resized
    Resize(u32, u32),
}

/// Keyboard keys relevant to NES emulation
#[derive(Debug, Clone, PartialEq)]
pub enum Key {
    // NES controller buttons (mapped to keyboard)
    Up,
    Down,
    Left,
    Right,
    A,      // Typically mapped to 'Z' or 'X'
    B,      // Typically mapped to 'A' or 'S'
    Start,  // Typically mapped to Enter
    Select, // Typically mapped to Space

    // Emulator controls
    Escape,
    Reset,  // Typically F1 or R
    Pause,  // Typically P

    // Other keys
    Other(String),
}

/// Convert a palette index to RGB color
#[inline]
pub fn palette_to_rgb(palette_index: u8) -> (u8, u8, u8) {
    let color = NES_PALETTE[palette_index as usize & 0x3F]; // Ensure valid index
    (
        ((color >> 16) & 0xFF) as u8, // Red
        ((color >> 8) & 0xFF) as u8,  // Green
        (color & 0xFF) as u8,         // Blue
    )
}

/// Convert a NES framebuffer to RGB888 format
///
/// This is a utility function that converts the palette-indexed framebuffer
/// to a format suitable for most rendering backends.
pub fn framebuffer_to_rgb888(framebuffer: &NESFramebuffer) -> Vec<u8> {
    let mut rgb_buffer = Vec::with_capacity(NES_WIDTH * NES_HEIGHT * 3);

    for &palette_index in framebuffer.iter() {
        let (r, g, b) = palette_to_rgb(palette_index);
        rgb_buffer.push(r);
        rgb_buffer.push(g);
        rgb_buffer.push(b);
    }

    rgb_buffer
}

/// Convert a NES framebuffer to RGBA8888 format
///
/// Similar to rgb888 but includes an alpha channel (always 255 for opaque)
pub fn framebuffer_to_rgba8888(framebuffer: &NESFramebuffer) -> Vec<u8> {
    let mut rgba_buffer = Vec::with_capacity(NES_WIDTH * NES_HEIGHT * 4);

    for &palette_index in framebuffer.iter() {
        let (r, g, b) = palette_to_rgb(palette_index);
        rgba_buffer.push(r);
        rgba_buffer.push(g);
        rgba_buffer.push(b);
        rgba_buffer.push(255); // Alpha
    }

    rgba_buffer
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_palette_to_rgb() {
        // Test first color (dark gray)
        let (r, g, b) = palette_to_rgb(0x00);
        assert_eq!((r, g, b), (0x66, 0x66, 0x66));

        // Test a known color (bright white)
        let (r, g, b) = palette_to_rgb(0x30);
        assert_eq!((r, g, b), (0xFF, 0xFE, 0xFF));
    }

    #[test]
    fn test_framebuffer_conversion() {
        let mut framebuffer = [0u8; NES_WIDTH * NES_HEIGHT];
        framebuffer[0] = 0x00; // First pixel dark gray
        framebuffer[1] = 0x30; // Second pixel white

        let rgb = framebuffer_to_rgb888(&framebuffer);
        assert_eq!(rgb[0..3], [0x66, 0x66, 0x66]); // First pixel
        assert_eq!(rgb[3..6], [0xFF, 0xFE, 0xFF]); // Second pixel

        let rgba = framebuffer_to_rgba8888(&framebuffer);
        assert_eq!(rgba[0..4], [0x66, 0x66, 0x66, 255]); // First pixel with alpha
        assert_eq!(rgba[4..8], [0xFF, 0xFE, 0xFF, 255]); // Second pixel with alpha
    }
}