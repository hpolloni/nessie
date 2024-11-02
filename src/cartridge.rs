use crate::bus::Bus;

pub struct Cartridge {
    cartridge_ram: [u8; 0x2000],
    prg_rom: Vec<u8>,
}

impl Cartridge {
    pub fn from_rom(buffer: &[u8]) -> Self {
        // TODO: Check NES header
        // TODO: Check iNes 1.0 format

        let skip_trainer = buffer[6] & 0b100 != 0;

        let prg_rom_start = 16 + if skip_trainer { 512 } else { 0 };
        let prg_rom_end = prg_rom_start + buffer[4] as usize * 0x4000;

        // TODO: read chr rom
        Self {
            cartridge_ram: [0x00; 0x2000],
            prg_rom: buffer[prg_rom_start..prg_rom_end].to_vec(),
        }
    }
}

impl Bus for Cartridge {
    fn read(&self, address: u16) -> u8 {
        match address {
            0x6000..=0x7FFF => {
                let address = address - 0x6000;
                self.cartridge_ram[address as usize]
            }
            0x8000..=0xFFFF => {
                let mut address = address - 0x8000;
                // Roms are usually 1 or 2 banks.
                // If rom is 16KB, address > 16KB are mirrored
                if self.prg_rom.len() == 0x4000 && address >= 0x4000 {
                    address = address % 0x4000;
                }
                self.prg_rom[address as usize]
            }
            _ => panic!("Access to unmapped cartridge address: {:4X}", address),
        }
    }

    fn write(&mut self, address: u16, value: u8) {
        match address {
            0x6000..=0x7FFF => {
                let address = address - 0x6000;
                self.cartridge_ram[address as usize] = value;
            }
            0x8000..=0xFFFF => {
                panic!("Can't write to cartridge rom address: {:4X}", address)
            }
            _ => panic!("Access to unmapped cartridge address: {:4X}", address),
        }
    }
}
