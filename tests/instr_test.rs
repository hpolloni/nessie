use core::str;
use std::{fs::File, io::Read};

use nessie::{bus::Bus, cpu::CPU};

struct NesBus {
    cpu_vram: [u8; 2048],
    prg_rom: Vec<u8>,
    cartridge_ram: [u8; 0x2000],
}

impl NesBus {
    fn new(prg_rom: Vec<u8>) -> Self {
        Self {
            cpu_vram: [0xDD; 2048],
            cartridge_ram: [0xCF; 0x2000],
            prg_rom,
        }
    }
}

impl Bus for NesBus {
    fn read(&self, address: u16) -> u8 {
        match address {
            0x0000..=0x1FFF => {
                let mirror_addr = address & 0b00000111_11111111;
                self.cpu_vram[mirror_addr as usize]
            }
            0x2000..=0x3FFF => 0,
            0x6000..=0x7FFF => {
                let address = address - 0x6000;
                self.cartridge_ram[address as usize]
            }
            0x8000..=0xFFFF => {
                let mut address = address - 0x8000;
                if self.prg_rom.len() == 0x4000 && address >= 0x4000 {
                    address = address % 0x4000;
                }
                self.prg_rom[address as usize]
            }
            _ => {
                println!("Access to unmapped address: {:4X}", address);
                0
            }
        }
    }

    fn write(&mut self, address: u16, value: u8) {
        match address {
            0x0000..=0x1FFF => {
                let mirror_addr = address & 0b00000111_11111111;
                self.cpu_vram[mirror_addr as usize] = value;
            }
            0x2000..=0x3FFF => {}
            0x6000..=0x7FFF => {
                let address = address - 0x6000;
                self.cartridge_ram[address as usize] = value;
            }
            0x8000..=0xFFFF => {
                panic!("Cant write to ROM address: {:4X}", address);
            }
            _ => {
                println!("Access to unmapped address: {:4X}", address);
            }
        }
    }
}

fn run_instr_test_rom(rom: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::open(rom)?;

    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    let skip_trainer = buffer[6] & 0b100 != 0;

    let prg_rom_start = 16 + if skip_trainer { 512 } else { 0 };
    let prg_rom_end = prg_rom_start + buffer[4] as usize * 16384;

    let bus = NesBus::new(buffer[prg_rom_start..prg_rom_end].to_vec());

    let pc = bus.read16(0xFFFC);

    let mut cpu = CPU::new(pc, Box::new(bus));

    let mut test_is_running = false;
    // Make sure that the test is running
    for _ in 0..100000 {
        cpu.step();

        if cpu.bus.read(0x6000) == 0x80
            && cpu.bus.read(0x6001) == 0xDE
            && cpu.bus.read(0x6002) == 0xB0
            && cpu.bus.read(0x6003) == 0x61
        {
            test_is_running = true;
            break;
        }
    }

    assert!(test_is_running, "Test is not running after 100,000 steps");

    while cpu.bus.read(0x6000) == 0x80 {
        cpu.step();
    }

    assert_eq!(0x00, cpu.bus.read(0x6000));

    // TODO: this should be in the Bus trait
    let mut status = vec![];
    let mut idx = 0;
    while cpu.bus.read(0x6004 + idx) != 0 {
        status.push(cpu.bus.read(0x6004 + idx));
        idx += 1;
    }
    println!("{}", str::from_utf8(&status)?);
    Ok(())
}

macro_rules! instr_test {
    ($func_name:ident, $file: expr) => {
        #[test]
        fn $func_name() -> Result<(), Box<dyn std::error::Error>> {
            run_instr_test_rom(&format!("roms/instr_test-v5/{}.nes", $file))
        }
    };
}

instr_test!(test_basics, "01-basics");
instr_test!(test_implied, "02-implied");

// ARR not implemented
// instr_test!(test_immediate, "03-immediate");

instr_test!(test_zero_page, "04-zero_page");
instr_test!(test_zp_xy, "05-zp_xy");
instr_test!(test_absolute, "06-absolute");

// SHY not implemented
// instr_test!(test_abs_xy, "07-abs_xy");

instr_test!(test_ind_x, "08-ind_x");
instr_test!(test_ind_y, "09-ind_y");
instr_test!(test_branches, "10-branches");
instr_test!(test_stack, "11-stack");
instr_test!(test_jmp_jsr, "12-jmp_jsr");
instr_test!(test_rts, "13-rts");

instr_test!(test_rti, "14-rti");

// BRK is really not implemented
// instr_test!(test_brk, "15-brk");

// instr_test!(test_special, "16-special");
