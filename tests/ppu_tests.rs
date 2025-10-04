use std::{cell::RefCell, rc::Rc};
use nessie::{bus::Bus, cartridge::Cartridge, cpu::CPU, nes::NesBus};

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

#[test]
fn test_vbl_clear_time_rom() {
    // Test the vbl_clear_time.nes ROM to validate VBlank timing behavior
    // This ROM tests the timing of when the VBlank flag is cleared

    use std::fs;

    // Load the ROM file
    let rom_path = "roms/ppu_tests/blargg_ppu_tests_2005.09.15b/vbl_clear_time.nes";
    let rom_data = match fs::read(rom_path) {
        Ok(data) => data,
        Err(_) => {
            // Skip test if ROM file is not available
            println!("Skipping test: {} not found", rom_path);
            return;
        }
    };

    let cartridge = Cartridge::from_rom(&rom_data);
    let bus = Rc::new(RefCell::new(NesBus::new(cartridge)));
    let mut cpu = CPU::new(0x8000, bus.clone());

    // Run the test ROM for a limited number of instructions
    // Blargg's test ROMs write result codes to $6000
    // $80 = test running, $00-$7F = test complete with result code
    for _ in 0..100_000 {
        cpu.step();

        // Check test completion
        let result = bus.borrow().read(0x6000);
        if result < 0x80 {
            // Test completed
            assert_eq!(result, 0x00,
                "vbl_clear_time test failed with code: {}. Check PPU VBlank timing implementation.",
                result);
            return;
        }
    }

    panic!("vbl_clear_time test timed out - ROM may not be running correctly");
}

#[test]
fn test_palette_ram_rom() {
    // Test the palette_ram.nes ROM to validate palette memory access

    use std::fs;

    let rom_path = "roms/ppu_tests/blargg_ppu_tests_2005.09.15b/palette_ram.nes";
    let rom_data = match fs::read(rom_path) {
        Ok(data) => data,
        Err(_) => {
            println!("Skipping test: {} not found", rom_path);
            return;
        }
    };

    let cartridge = Cartridge::from_rom(&rom_data);
    let bus = Rc::new(RefCell::new(NesBus::new(cartridge)));
    let mut cpu = CPU::new(0x8000, bus.clone());

    for _ in 0..100_000 {
        cpu.step();

        let result = bus.borrow().read(0x6000);
        if result < 0x80 {
            assert_eq!(result, 0x00,
                "palette_ram test failed with code: {}. Check PPU palette memory implementation.",
                result);
            return;
        }
    }

    panic!("palette_ram test timed out");
}