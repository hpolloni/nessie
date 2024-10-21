use std::{fs::File, io::Read};

use cpu::CPU;

mod cpu;

fn main() {
    let mut file = File::open("roms/nestest/nestest.nes").unwrap();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();

    let mut ram = vec![0u8; 65536];

    ram[0x8000..0xBFFF].copy_from_slice(&buffer[0x0010..0x400f]);
    ram[0xC000..0xFFFF].copy_from_slice(&buffer[0x0010..0x400f]);

    let mut cpu = CPU::new(0xC000, Box::new(ram));

    // Execute the first 2 instr
    // This will eventually crash
    cpu.run_until_brk();
    /*
    let mut file = File::open("roms/instr_test-v5/01-basics.nes").unwrap();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();

    let mut ram = [0u8; 65536];

    ram[0x8000..0xFFFF].copy_from_slice(&buffer[0x0010..0x800f]);

    let mut cpu = CPU::new(0xFFFC, Box::new(ram));

    cpu.step();*/
}
