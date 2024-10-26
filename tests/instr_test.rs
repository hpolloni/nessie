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
            cpu_vram: [0; 2048],
            cartridge_ram: [0; 0x2000],
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
            0x2000..=0x3FFF => {
                println!("Ignoring PPU address: {:4X}", address);
                0
            }
            0x6000..=0x7FFF => {
                let mut address = address - 0x6000;
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
            0x2000..=0x3FFF => {
                println!("Ignoring PPU address: {:4X}", address);
            }
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
#[test]
fn test_basics_rom() -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::open("roms/instr_test-v5/01-basics.nes")?;

    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    let skip_trainer = buffer[6] & 0b100 != 0;

    let prg_rom_start = 16 + if skip_trainer { 512 } else { 0 };
    let prg_rom_end = prg_rom_start + buffer[4] as usize * 16384;

    let bus = NesBus::new(buffer[prg_rom_start..prg_rom_end].to_vec());

    let pc = bus.read16(0xFFFC);

    let mut cpu = CPU::new(pc, Box::new(bus));

    for _ in 0..100 {
        println!("{}", cpu.trace());

        cpu.step();

        println!("{:2X}", cpu.bus.read(0x6000));
    }
    todo!();
    Ok(())
}
