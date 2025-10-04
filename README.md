# Nessie

A Nintendo Entertainment System (NES) emulator written in Rust, focusing on accuracy and clean architecture.

## Overview

Nessie is an educational NES emulator project that aims to accurately recreate the behavior of the Nintendo Entertainment System's hardware components. Currently, the project features a complete and cycle-accurate implementation of the 6502 CPU that powered the NES.

## Current Status

### ✅ Implemented
- **6502 CPU**: Complete instruction set with cycle-accurate timing
- **Memory Bus**: Flexible bus architecture supporting different memory configurations
- **Basic Cartridge Support**: ROM loading with NROM mapper (mapper 0)
- **Test Infrastructure**: Comprehensive CPU validation using authentic NES test ROMs

### 🚧 In Development
- **PPU (Picture Processing Unit)**: Graphics rendering system
- **APU (Audio Processing Unit)**: Sound generation
- **Advanced Mappers**: Support for additional cartridge types
- **User Interface**: Frontend for actually playing games

## Features

- **Accuracy-Focused**: Validated against real NES test ROMs including the famous `nestest`
- **Clean Architecture**: Modular design with trait-based abstractions
- **Comprehensive Testing**: Uses authentic test ROMs to ensure hardware-level accuracy
- **Memory Safety**: Built with Rust's ownership system for safe emulation

## Building and Running

### Prerequisites
- Rust 1.70+ (2021 edition)
- Cargo

### Build
```bash
# Clone the repository
git clone <repository-url>
cd nessie

# Initialize test ROM submodule (required for running tests)
git submodule update --init --recursive

# Build the project
cargo build

# Run tests
cargo test
```

### Testing

The project includes comprehensive test suites using authentic NES test ROMs from the [nes-test-roms](https://github.com/christopherpow/nes-test-roms) collection:

```bash
# Run all tests (23 tests total)
cargo test

# Run specific test categories
cargo test nestest           # CPU validation test
cargo test instr_test        # Individual instruction tests (12 tests)
cargo test ppu_tests         # PPU functionality tests (8 tests)

# Run specific PPU test ROMs
cargo test test_vbl_clear_time_rom    # VBlank timing validation
cargo test test_palette_ram_rom       # Palette RAM access validation
```

#### Test ROM Collection
The project uses a git submodule containing 60+ test ROM suites including:
- **nestest** - Comprehensive CPU behavior validation
- **instr_test-v5** - Individual instruction category tests
- **blargg_ppu_tests** - PPU register and timing validation
- **sprite_hit_tests** - Sprite collision detection
- **apu_tests** - Audio processing unit tests
- And many more for comprehensive NES hardware validation

## Architecture

### Core Components

- **CPU (`src/cpu.rs`)**: 6502 processor implementation with all official opcodes
- **Bus (`src/bus.rs`)**: Memory bus abstraction using Rust traits
- **Opcodes (`src/opcodes.rs`)**: Complete 6502 instruction set and addressing modes
- **Cartridge (`src/cartridge.rs`)**: Game cartridge ROM/RAM handling
- **NES (`src/nes.rs`)**: System bus connecting all components

### Design Principles

1. **Trait-Based Architecture**: The `Bus` trait allows different memory implementations
2. **Shared Ownership**: Uses `Rc<RefCell<>>` for safe shared access between components
3. **Bitflags**: CPU status register implemented with the `bitflags` crate
4. **Golden Master Testing**: CPU behavior validated against known-good execution traces

## CPU Implementation

The 6502 CPU implementation includes:
- All 151 official opcodes
- Proper cycle counting for timing accuracy
- Complete addressing mode support
- Accurate flag handling
- Interrupt processing (NMI, IRQ, BRK)

## Test ROMs

The project uses the comprehensive [nes-test-roms](https://github.com/christopherpow/nes-test-roms) collection via git submodule at `roms/external/`. This provides access to 60+ test ROM suites for validating NES hardware behavior.

**Note**: You must initialize the submodule to run tests:
```bash
git submodule update --init --recursive
```

## Dependencies

- `bitflags`: CPU status register flag handling
- `log` + `env_logger`: Logging infrastructure
- `assert_matches`: Testing utilities

## Development

### Code Generation

The `opcode_table_generator.py` script generates parts of the opcodes module from Visual6502 wiki data. When modifying opcode handling, consider updating the generator script.

### Testing Strategy

1. **Unit Tests**: Individual component testing
2. **Integration Tests**: Full system tests using test ROMs
3. **Golden Master**: Comparing execution traces against reference implementations

### Updating Test ROMs

To update to the latest test ROMs:
```bash
git submodule update --remote
```

To add new test ROM validation, create tests in the appropriate `tests/*.rs` file that load ROMs from `roms/external/`.

## Roadmap

### Near Term
- [ ] PPU implementation for graphics rendering
- [ ] Basic APU for sound generation
- [ ] Simple frontend for loading and running ROMs

### Long Term
- [ ] Advanced mapper support (MMC1, MMC3, etc.)
- [ ] Save state functionality
- [ ] Debugging tools and CPU step-through
- [ ] Performance optimizations
- [ ] Audio/video recording

## Contributing

This is an educational project focused on understanding NES hardware emulation. The codebase prioritizes clarity and accuracy over performance.

### Getting Started
1. Read through the existing CPU implementation to understand the architecture
2. Run the test suite to ensure everything works
3. Check the CLAUDE.md file for development guidance

## Resources

### NES Development
- [NESDev Wiki](https://wiki.nesdev.com/) - Comprehensive NES hardware documentation
- [6502 Reference](http://www.obelisk.me.uk/6502/) - Detailed 6502 CPU documentation
- [Visual6502](http://visual6502.org/) - Visual 6502 simulation and analysis

### Test ROMs
- [nestest](https://wiki.nesdev.com/w/index.php/Emulator_tests) - CPU validation ROM
- [blargg's test ROMs](https://github.com/christopherpow/nes-test-roms) - Comprehensive test suite

## Technical Notes

### Memory Layout
The NES uses a 64KB address space with complex memory mapping handled by the bus system. The current implementation supports basic ROM access through the cartridge interface.

### Accuracy
CPU timing is implemented to match real hardware, with proper cycle counting for each instruction. This ensures games that rely on precise timing will work correctly.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

---

*Nessie is an educational project and is not affiliated with Nintendo or any commercial emulator.*