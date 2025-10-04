# PPU Implementation Design

This document outlines the design and implementation strategy for adding the Picture Processing Unit (PPU) to the Nessie NES emulator.

## Design Goals

1. **Test-Driven Development**: Use authentic NES test ROMs to validate PPU behavior
2. **Accuracy-Focused**: Implement cycle-accurate timing where needed for compatibility
3. **Modular Architecture**: PPU as separate struct, similar to CPU design
4. **Incremental Development**: Build functionality progressively using test ROMs

## Architecture Overview

### PPU as Separate Module

Following the existing CPU pattern:

```rust
// src/ppu.rs
pub struct PPU {
    // Internal PPU state
    cycle: u16,           // Current cycle in scanline (0-340)
    scanline: u16,        // Current scanline (0-261)
    frame: u64,           // Frame counter

    // PPU Registers ($2000-$2007)
    ctrl: u8,             // $2000 PPUCTRL
    mask: u8,             // $2001 PPUMASK
    status: u8,           // $2002 PPUSTATUS
    oam_addr: u8,         // $2003 OAMADDR
    // ... other registers

    // Internal memory
    vram: [u8; 0x800],    // Name tables (2KB internal)
    palette_ram: [u8; 32], // Palette memory
    oam: [u8; 256],       // Object Attribute Memory (sprites)

    // Rendering state
    background_shifters: BackgroundShifters,
    sprite_shifters: [SpriteShifter; 8],

    // Output buffer
    frame_buffer: [u8; 256 * 240 * 3], // RGB pixels
}

impl PPU {
    pub fn new() -> Self { /* ... */ }
    pub fn clock(&mut self) { /* ... */ }
    pub fn cpu_read(&mut self, address: u16) -> u8 { /* ... */ }
    pub fn cpu_write(&mut self, address: u16, value: u8) { /* ... */ }
    pub fn nmi_occurred(&self) -> bool { /* ... */ }
}
```

### Integration with NES Bus

Modify `src/nes.rs` to include PPU:

```rust
pub struct NesBus {
    cpu_vram: [u8; 2048],
    cartridge: Cartridge,
    ppu: PPU,
}

impl Bus for NesBus {
    fn read(&self, address: u16) -> u8 {
        match address {
            0x2000..=0x3FFF => {
                // Mirror PPU registers every 8 bytes
                let ppu_reg = 0x2000 + (address & 0x0007);
                self.ppu.cpu_read(ppu_reg)
            }
            // ... other ranges
        }
    }
}
```

## Test-Driven Implementation Strategy

### Phase 1: Basic PPU Registers (Week 1)

**Target Test ROMs:**
- `blargg_ppu_tests_2005.09.15b/palette_ram.nes`
- `ppu_open_bus`

**Implementation Focus:**
- PPU register read/write behavior
- Register mirroring (every 8 bytes in $2000-$3FFF)
- Basic VRAM access via $2006/$2007

**Success Criteria:**
- palette_ram test passes
- Basic register writes don't crash

### Phase 2: VBlank and NMI Timing (Week 2)

**Target Test ROMs:**
- `ppu_vbl_nmi`
- `blargg_ppu_tests_2005.09.15b/vbl_clear_time.nes`

**Implementation Focus:**
- Scanline/cycle counting (262 scanlines, 341 cycles each)
- VBlank flag setting/clearing
- NMI generation timing
- Basic PPU clock stepping

**Success Criteria:**
- VBlank occurs at correct timing
- NMI fires when enabled
- vbl_clear_time test passes

### Phase 3: VRAM Access and Buffering (Week 3)

**Target Test ROMs:**
- `ppu_read_buffer`
- `blargg_ppu_tests_2005.09.15b/vram_access.nes`

**Implementation Focus:**
- PPU internal read buffer
- Address increment behavior ($2006/$2007)
- VRAM mirroring

**Success Criteria:**
- VRAM reads return correct buffered values
- Address increments work properly

### Phase 4: Sprite Memory (Week 4)

**Target Test ROMs:**
- `blargg_ppu_tests_2005.09.15b/sprite_ram.nes`
- `oam_stress`

**Implementation Focus:**
- OAM (Object Attribute Memory) access
- $2003/$2004 sprite memory registers
- $4014 DMA transfer (CPU bus interaction)

**Success Criteria:**
- Sprite RAM tests pass
- DMA timing is approximately correct

### Phase 5: Basic Rendering (Week 5-6)

**Target Test ROMs:**
- `sprite_hit_tests_2005.10.05`
- `full_palette` (visual test)

**Implementation Focus:**
- Scanline rendering (no mid-frame effects)
- Background tile rendering
- Sprite rendering
- Sprite 0 hit detection
- Color palette lookup

**Success Criteria:**
- Simple games show recognizable graphics
- Sprite 0 hit timing roughly correct
- Color palette displays correctly

### Phase 6: Advanced Timing (Week 7+)

**Target Test ROMs:**
- `sprite_overflow_tests`
- Advanced timing tests

**Implementation Focus:**
- Pixel-accurate rendering
- Mid-scanline register changes
- Complex sprite overflow behavior

## Rendering Approach Decision

### Initial: Scanline Rendering

For the first implementation, we'll use **scanline rendering**:

- Render one complete scanline at a time
- Update registers only between scanlines
- Good compatibility with ~90% of NES games
- Simpler to implement and debug

**Advantages:**
- Faster development
- Easier debugging
- Works for most games
- Clear separation of concerns

**Limitations:**
- Won't handle mid-scanline register changes
- Some advanced games may glitch

### Future: Pixel-Accurate Rendering

Later phases can add **cycle-accurate** rendering:

- Render pixel-by-pixel
- Handle mid-scanline register changes
- Required for games with advanced effects

## Test ROM Integration

### Test ROM Repository Setup

```bash
# Add PPU test ROMs to project
mkdir -p roms/ppu_tests
cd roms/ppu_tests

# Download key test ROMs
wget https://github.com/christopherpow/nes-test-roms/raw/master/blargg_ppu_tests_2005.09.15b.zip
wget https://github.com/christopherpow/nes-test-roms/raw/master/ppu_vbl_nmi.zip
wget https://github.com/christopherpow/nes-test-roms/raw/master/sprite_hit_tests_2005.10.05.zip
# ... other ROMs
```

### Automated Testing Framework

```rust
// tests/ppu_tests.rs
#[test]
fn test_palette_ram() {
    let rom = load_test_rom("roms/ppu_tests/palette_ram.nes");
    let mut nes = NES::new(rom);

    // Run until test completion
    for _ in 0..1_000_000 {
        nes.step();

        // Check test result at $6000
        if nes.cpu_read(0x6000) < 0x80 {
            let result = nes.cpu_read(0x6000);
            assert_eq!(result, 0x00, "palette_ram test failed with code {}", result);
            return;
        }
    }

    panic!("Test timed out");
}
```

## Rendering Output Options

### Option 1: Headless Testing (Initial)
- RGB buffer in memory
- No graphics output initially
- Focus on test ROM validation
- Save frames to files for debugging

### Option 2: Terminal Output (Debug)
- ASCII art representation
- Character-based "graphics"
- Good for debugging register behavior
- Cross-platform compatibility

### Option 3: GUI Integration (Future)
- SDL2 or similar graphics library
- Real-time rendering
- User interaction (loading ROMs)
- Save states, debugging tools

### Recommendation: Start Headless

Begin with **headless rendering** to focus on accuracy:

```rust
pub struct PPU {
    // ...
    frame_buffer: [RGB; 256 * 240], // RGB pixel buffer
    frame_complete: bool,
}

impl PPU {
    pub fn get_frame(&self) -> &[RGB; 256 * 240] {
        &self.frame_buffer
    }

    pub fn frame_ready(&self) -> bool {
        self.frame_complete
    }
}
```

## Memory Layout

### PPU Address Space ($0000-$3FFF)
```
$0000-$0FFF: Pattern Table 0 (tiles/sprites)
$1000-$1FFF: Pattern Table 1 (tiles/sprites)
$2000-$23FF: Nametable 0
$2400-$27FF: Nametable 1
$2800-$2BFF: Nametable 2
$2C00-$2FFF: Nametable 3
$3000-$3EFF: Mirrors of $2000-$2EFF
$3F00-$3F1F: Palette RAM
$3F20-$3FFF: Mirrors of $3F00-$3F1F
```

### CPU Address Space ($2000-$3FFF)
```
$2000: PPUCTRL
$2001: PPUMASK
$2002: PPUSTATUS
$2003: OAMADDR
$2004: OAMDATA
$2005: PPUSCROLL
$2006: PPUADDR
$2007: PPUDATA
$2008-$3FFF: Mirrors of $2000-$2007
```

## Development Timeline

| Week | Phase | Goal | Test ROMs |
|------|-------|------|-----------|
| 1 | Basic Registers | PPU register access | palette_ram |
| 2 | VBlank/NMI | Timing basics | ppu_vbl_nmi |
| 3 | VRAM Access | Memory interface | vram_access |
| 4 | Sprite Memory | OAM handling | sprite_ram |
| 5-6 | Basic Rendering | Scanline graphics | sprite_hit_tests |
| 7+ | Advanced Timing | Pixel accuracy | sprite_overflow |

## Success Metrics

### Phase 1-4: Register Accuracy
- All blargg PPU register tests pass
- No crashes on PPU register access
- Correct timing for VBlank/NMI

### Phase 5: Basic Graphics
- Simple games (e.g., Donkey Kong) show recognizable graphics
- Backgrounds and sprites render
- Color palettes display correctly

### Phase 6+: Advanced Compatibility
- Complex games work correctly
- Mid-frame effects supported
- Sprite overflow behavior accurate

## Tools and Resources

### Development Tools
- `cargo test` - Run PPU test suite
- Frame capture tools for visual debugging
- Cycle/timing analysis utilities

### Reference Materials
- [NESDev Wiki PPU pages](https://www.nesdev.org/wiki/PPU)
- [Visual NES simulator](http://visual6502.org/)
- Existing emulator source code (FCEUX, Nestopia)

### Test ROM Collections
- Blargg's PPU test suite (2005.09.15b)
- Individual PPU test ROMs
- Visual test ROMs for debugging

---

This design provides a clear roadmap for implementing a test-driven, accurate PPU that builds incrementally from basic register handling to full rendering capabilities.