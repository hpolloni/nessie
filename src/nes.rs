use crate::{bus::Bus, cartridge::Cartridge};
use log::warn;

pub struct NesBus {
    cpu_vram: [u8; 2048],
    cartridge: Cartridge,
}

impl NesBus {
    pub fn new(cartridge: Cartridge) -> Self {
        Self {
            cpu_vram: [0x00; 2048],
            cartridge,
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
            0x6000..=0xFFFF => self.cartridge.read(address),
            _ => {
                warn!("Access to unmapped address: {:4X}", address);
                0x00
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
            0x6000..=0xFFFF => self.cartridge.write(address, value),
            _ => {
                warn!("Access to unmapped address: {:4X}", address);
            }
        }
    }
}
