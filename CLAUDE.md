# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Nessie is a Nintendo Entertainment System (NES) emulator written in Rust, focusing on accurate 6502 CPU emulation and NES hardware components. The project is in early development with a complete CPU implementation and basic memory bus architecture.

## Development Commands

### Build and Testing
- `cargo build` - Build the project
- `cargo test` - Run all tests including CPU validation tests
- `cargo check` - Fast compilation check without producing binaries
- `cargo test nestest` - Run the comprehensive CPU test using nestest ROM
- `cargo test instr_test` - Run individual instruction category tests

### Running Specific Tests
Individual instruction tests are available in the test suite:
- Tests are located in `tests/` directory
- ROM files for testing are in `roms/` directory
- Use `cargo test <test_name>` to run specific tests

## Architecture Overview

### Core Components
- **CPU (`src/cpu.rs`)** - Complete 6502 processor implementation with cycle-accurate timing
- **Bus (`src/bus.rs`)** - Memory bus abstraction using trait-based design
- **Opcodes (`src/opcodes.rs`)** - Full 6502 instruction set and addressing modes
- **Cartridge (`src/cartridge.rs`)** - ROM/RAM handling for game cartridges
- **NES (`src/nes.rs`)** - System bus implementation connecting components

### Design Patterns
- **Trait-based Bus Architecture**: `Bus` trait abstracts memory access, allowing different implementations
- **Shared Ownership**: Uses `Rc<RefCell<>>` for shared mutable access to the bus between CPU and other components
- **Bitflags for CPU Status**: CPU status register implemented using bitflags crate for clean flag manipulation
- **Golden Master Testing**: CPU behavior validated against known-good execution traces from nestest ROM

### Memory Layout
The CPU operates on a 64KB address space with the bus handling memory mapping. The current implementation supports basic cartridge ROM access through the bus interface.

## Testing Infrastructure

### Test ROMs
- **nestest** - Comprehensive CPU behavior validation ROM
- **instr_test-v5** - 16 individual instruction category test ROMs
- Test ROMs are stored in `roms/` directory and used for integration testing

### Validation Approach
CPU accuracy is validated by comparing execution traces against reference implementations. Tests check for proper instruction execution, flag handling, and cycle counting.

## Current Implementation Status

### Completed
- Full 6502 CPU instruction set
- Memory bus architecture
- Basic cartridge ROM loading (mapper 0/NROM)
- Comprehensive CPU testing infrastructure

### Pending Implementation
- PPU (Picture Processing Unit) for graphics
- APU (Audio Processing Unit) for sound
- Advanced cartridge mappers beyond NROM
- User interface/frontend
- Controller input handling

## Code Generation

The `opcode_table_generator.py` script generates parts of the opcodes module from Visual6502 wiki data. When modifying opcode handling, consider whether the generator script needs updates.

## Dependencies

The project uses minimal dependencies focused on low-level functionality:
- `bitflags` - CPU status register flags
- `log` + `env_logger` - Logging infrastructure
- `assert_matches` - Testing utilities (dev dependency)