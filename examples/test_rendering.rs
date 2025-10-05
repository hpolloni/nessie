// Test rendering example
//
// This example demonstrates the rendering system by creating a simple
// test pattern and showing how to convert NES framebuffer data to RGB.

use nessie::{
    ppu::PPU,
    rendering::{framebuffer_to_rgba8888, palette_to_rgb, NES_HEIGHT, NES_WIDTH},
};

fn main() {
    println!("Nessie Rendering Test");
    println!("===================");

    // Create a PPU instance
    let mut ppu = PPU::new();

    // Test 1: Solid color rendering
    println!("\nTest 1: Solid color rendering");
    ppu.render_solid_color(0x30); // White
    let framebuffer = ppu.framebuffer();
    println!("First pixel palette index: {}", framebuffer[0]);

    let (r, g, b) = palette_to_rgb(framebuffer[0]);
    println!("First pixel RGB: ({}, {}, {})", r, g, b);

    // Test 2: Test pattern
    println!("\nTest 2: Test pattern");
    ppu.render_test_pattern();
    let framebuffer = ppu.framebuffer();

    // Sample a few pixels from the test pattern
    let samples = [
        (0, 0),      // Top-left
        (128, 120),  // Center
        (255, 239),  // Bottom-right
    ];

    for (x, y) in samples {
        let index = y * NES_WIDTH + x;
        let palette_index = framebuffer[index];
        let (r, g, b) = palette_to_rgb(palette_index);
        println!("Pixel at ({}, {}): palette={}, RGB=({}, {}, {})",
                 x, y, palette_index, r, g, b);
    }

    // Test 3: RGB conversion
    println!("\nTest 3: RGB conversion performance");
    let start = std::time::Instant::now();
    let rgba_data = framebuffer_to_rgba8888(framebuffer);
    let duration = start.elapsed();

    println!("Converted {}x{} framebuffer to RGBA in {:?}",
             NES_WIDTH, NES_HEIGHT, duration);
    println!("RGBA data size: {} bytes", rgba_data.len());
    println!("Expected size: {} bytes", NES_WIDTH * NES_HEIGHT * 4);

    // Test 4: Color palette validation
    println!("\nTest 4: Color palette samples");
    let palette_samples = [0x00, 0x0F, 0x16, 0x1A, 0x12, 0x30, 0x38];
    for &palette_index in &palette_samples {
        let (r, g, b) = palette_to_rgb(palette_index);
        println!("Palette {:#04X}: RGB({}, {}, {})", palette_index, r, g, b);
    }

    println!("\nRendering test completed successfully!");
}