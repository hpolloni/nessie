use std::{cell::RefCell, rc::Rc};
use nessie::{bus::Bus, cartridge::Cartridge, cpu::CPU, nes::NesBus};

// PPU (Picture Processing Unit) Tests
//
// This module contains tests for the NES PPU implementation, including:
// - PPU register behavior and timing tests
// - VBlank and NMI functionality tests
// - ROM-based validation tests using authentic NES test ROMs

// Helper function to run a ROM test and check for completion
fn run_rom_test(rom_path: &str, test_description: &str) -> Result<(), Box<dyn std::error::Error>> {
    use std::fs;

    let rom_data = match fs::read(rom_path) {
        Ok(data) => data,
        Err(_) => {
            println!("Skipping test: {} not found", rom_path);
            return Ok(());
        }
    };

    let cartridge = Cartridge::from_rom(&rom_data);
    let bus = Rc::new(RefCell::new(NesBus::new(cartridge)));
    let mut cpu = CPU::new(0x8000, bus.clone());

    for _ in 0..100_000 {
        cpu.step();

        let result = bus.borrow().read(0x6000);
        if result < 0x80 {
            if result == 0x00 {
                return Ok(()); // Test passed
            } else {
                return Err(format!(
                    "{} test failed with code: {}. Check {}.",
                    test_description, result, test_description
                ).into());
            }
        }
    }

    Err(format!("{} test timed out", test_description).into())
}

// Macro to create ROM tests with minimal boilerplate
macro_rules! ppu_rom_test {
    ($test_name:ident, $rom_path:expr, $description:expr) => {
        #[test]
        fn $test_name() -> Result<(), Box<dyn std::error::Error>> {
            run_rom_test($rom_path, $description)
        }
    };
}

// =============================================================================
// PPU Register Behavior Tests
// =============================================================================

#[test]
fn test_ppustatus_read_behavior() {
    // PPUSTATUS ($2002) should have specific behavior:
    // - Bit 7: VBlank flag
    // - Reading PPUSTATUS should clear the VBlank flag
    // - Writing to PPUSTATUS should be ignored (read-only register)

    let cartridge_data = vec![0x00; 0x8000];
    let cartridge = Cartridge::from_rom(&cartridge_data);
    let bus = Rc::new(RefCell::new(NesBus::new(cartridge)));

    // Write to PPUSTATUS should be ignored (it's read-only)
    bus.borrow_mut().write(0x2002, 0xFF);

    // Read should return actual PPU status, not what we tried to write
    let status = bus.borrow().read(0x2002);

    // Currently NesBus returns 0 for PPU reads, but a real PPU would have
    // proper status register behavior. This test will fail until we implement a real PPU.
    // The VBlank flag (bit 7) should be properly managed by the PPU
    assert_eq!(status & 0x80, 0x80, "VBlank flag should be set initially or after proper timing");
}

#[test]
fn test_ppu_register_mirroring() {
    // Test that PPU registers mirror every 8 bytes in $2000-$3FFF range
    let cartridge_data = vec![0x00; 0x8000];
    let cartridge = Cartridge::from_rom(&cartridge_data);
    let bus = Rc::new(RefCell::new(NesBus::new(cartridge)));

    // Write to base register
    bus.borrow_mut().write(0x2000, 0xAA);

    // Check mirroring at various offsets
    // These should all access the same register due to mirroring
    bus.borrow_mut().write(0x2008, 0xBB); // +8
    bus.borrow_mut().write(0x2010, 0xCC); // +16
    bus.borrow_mut().write(0x3FF8, 0xDD); // Near end of range

    // This test will verify proper mirroring once implemented
}

// =============================================================================
// VBlank and Timing Tests
// =============================================================================

#[test]
fn test_vblank_flag_set_on_scanline_241() {
    // Simple test: VBlank flag should be set when we reach scanline 241

    let cartridge_data = vec![0x00; 0x8000];
    let cartridge = Cartridge::from_rom(&cartridge_data);
    let mut bus = NesBus::new(cartridge);

    // Our PPU starts at scanline 241, so VBlank should already be set
    assert_eq!(bus.get_ppu_scanline(), 241, "PPU starts at scanline 241");
    let status = bus.read(0x2002);
    assert_eq!(status & 0x80, 0x80, "VBlank should be set at scanline 241");

    // Clear VBlank by reading status
    let _status = bus.read(0x2002);
    let status = bus.read(0x2002);
    assert_eq!(status & 0x80, 0x00, "VBlank should be cleared after read");

    // Step to end of current scanline
    for _ in 0..341 {
        bus.step_ppu();
    }

    // Should be at scanline 242, and VBlank should still be cleared (no new transition)
    assert_eq!(bus.get_ppu_scanline(), 242, "Should be at scanline 242");
    let status = bus.read(0x2002);
    assert_eq!(status & 0x80, 0x00, "VBlank should remain cleared at scanline 242");
}

#[test]
fn test_vblank_clears_at_scanline_261() {
    // Test that VBlank is cleared when we reach scanline 261

    let cartridge_data = vec![0x00; 0x8000];
    let cartridge = Cartridge::from_rom(&cartridge_data);
    let mut bus = NesBus::new(cartridge);

    // Start at scanline 241 with VBlank set
    assert_eq!(bus.get_ppu_scanline(), 241, "Start at scanline 241");

    // Step to scanline 261
    while bus.get_ppu_scanline() != 261 {
        bus.step_ppu();
    }

    assert_eq!(bus.get_ppu_scanline(), 261, "Should reach scanline 261");
    let status = bus.read(0x2002);
    assert_eq!(status & 0x80, 0x00, "VBlank should be cleared at scanline 261");
}

#[test]
fn test_vblank_flag_clearing_on_read() {
    // Reading PPUSTATUS should clear the VBlank flag

    let cartridge_data = vec![0x00; 0x8000];
    let cartridge = Cartridge::from_rom(&cartridge_data);
    let bus = Rc::new(RefCell::new(NesBus::new(cartridge)));

    // First read should show VBlank set
    let status1 = bus.borrow().read(0x2002);
    assert_eq!(status1 & 0x80, 0x80, "VBlank should be set initially");

    // Second read should show VBlank cleared
    let status2 = bus.borrow().read(0x2002);
    assert_eq!(status2 & 0x80, 0x00, "VBlank should be cleared after first read");
}

#[test]
fn test_nmi_generation_on_vblank() {
    // Test that NMI is generated when VBlank occurs and NMI is enabled

    let cartridge_data = vec![0x00; 0x8000];
    let cartridge = Cartridge::from_rom(&cartridge_data);
    let mut bus = NesBus::new(cartridge);

    // Enable NMI in PPUCTRL (bit 7)
    bus.write(0x2000, 0x80); // Set NMI_ENABLE

    // Check that PPU recognizes NMI should occur (VBlank is set + NMI enabled)
    assert!(bus.should_generate_nmi(), "NMI should be generated when VBlank + NMI_ENABLE");

    // Disable NMI
    bus.write(0x2000, 0x00); // Clear NMI_ENABLE

    // NMI should not occur even though VBlank is still set
    assert!(!bus.should_generate_nmi(), "NMI should not be generated when NMI disabled");
}

// =============================================================================
// ROM-Based Validation Tests
// =============================================================================
// These tests use authentic NES test ROMs to validate PPU behavior against
// real hardware. Test ROMs check for specific error codes at memory location
// $6000, where 0x00 indicates success and any other value indicates failure.

// PPU ROM Tests - using macro for concise test definitions
ppu_rom_test!(
    test_vbl_clear_time_rom,
    "roms/external/blargg_ppu_tests_2005.09.15b/vbl_clear_time.nes",
    "VBlank timing behavior"
);

ppu_rom_test!(
    test_palette_ram_rom,
    "roms/external/blargg_ppu_tests_2005.09.15b/palette_ram.nes",
    "PPU palette memory access"
);

ppu_rom_test!(
    test_ppu_vbl_nmi_rom,
    "roms/external/ppu_vbl_nmi/ppu_vbl_nmi.nes",
    "VBlank NMI timing behavior"
);

ppu_rom_test!(
    test_sprite_ram_rom,
    "roms/external/blargg_ppu_tests_2005.09.15b/sprite_ram.nes",
    "PPU OAM/sprite memory access"
);

ppu_rom_test!(
    test_vram_access_rom,
    "roms/external/blargg_ppu_tests_2005.09.15b/vram_access.nes",
    "PPU VRAM read/write behavior"
);

ppu_rom_test!(
    test_ppu_open_bus_rom,
    "roms/external/ppu_open_bus/ppu_open_bus.nes",
    "PPU open bus behavior"
);

ppu_rom_test!(
    test_ppu_read_buffer_rom,
    "roms/external/ppu_read_buffer/test_ppu_read_buffer.nes",
    "PPU read buffer behavior"
);