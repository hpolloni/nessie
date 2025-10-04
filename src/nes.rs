use crate::{bus::Bus, cartridge::Cartridge, ppu::PPU};
use log::warn;

pub struct NesBus {
    cpu_vram: [u8; 2048],
    cartridge: Cartridge,
    ppu: PPU,
}

impl NesBus {
    pub fn new(cartridge: Cartridge) -> Self {
        Self {
            cpu_vram: [0x00; 2048],
            cartridge,
            ppu: PPU::new(),
        }
    }

    pub fn step_ppu(&mut self) {
        self.ppu.clock();
    }

    pub fn get_ppu_scanline(&self) -> u16 {
        self.ppu.scanline
    }

    pub fn get_ppu_cycle(&self) -> u16 {
        self.ppu.cycle
    }

    pub fn should_generate_nmi(&self) -> bool {
        self.ppu.nmi_occurred()
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
                // PPU registers mirror every 8 bytes
                let ppu_reg = 0x2000 + (address & 0x0007);
                // We need to cast away the const to call cpu_read
                // This is a temporary workaround - ideally we'd refactor the Bus trait
                let ppu_ptr = &self.ppu as *const PPU as *mut PPU;
                unsafe { (*ppu_ptr).cpu_read(ppu_reg) }
            }
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
            0x2000..=0x3FFF => {
                // PPU registers mirror every 8 bytes
                let ppu_reg = 0x2000 + (address & 0x0007);
                self.ppu.cpu_write(ppu_reg, value);
            }
            0x6000..=0xFFFF => self.cartridge.write(address, value),
            _ => {
                warn!("Access to unmapped address: {:4X}", address);
            }
        }
    }
}
