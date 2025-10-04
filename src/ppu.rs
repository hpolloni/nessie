use bitflags::bitflags;

bitflags! {
    #[derive(Copy, Clone, Debug)]
    pub struct PpuStatus: u8 {
        const SPRITE_OVERFLOW = 1 << 5;
        const SPRITE_ZERO_HIT = 1 << 6;
        const VBLANK = 1 << 7;
    }
}

bitflags! {
    #[derive(Copy, Clone, Debug)]
    pub struct PpuCtrl: u8 {
        const NAMETABLE_X = 1;
        const NAMETABLE_Y = 1 << 1;
        const VRAM_INCREMENT = 1 << 2;
        const SPRITE_PATTERN = 1 << 3;
        const BACKGROUND_PATTERN = 1 << 4;
        const SPRITE_SIZE = 1 << 5;
        const MASTER_SLAVE = 1 << 6;
        const NMI_ENABLE = 1 << 7;
    }
}

bitflags! {
    #[derive(Copy, Clone, Debug)]
    pub struct PpuMask: u8 {
        const GREYSCALE = 1;
        const SHOW_BACKGROUND_LEFT = 1 << 1;
        const SHOW_SPRITES_LEFT = 1 << 2;
        const SHOW_BACKGROUND = 1 << 3;
        const SHOW_SPRITES = 1 << 4;
        const EMPHASIZE_RED = 1 << 5;
        const EMPHASIZE_GREEN = 1 << 6;
        const EMPHASIZE_BLUE = 1 << 7;
    }
}

pub struct PPU {
    // PPU Registers
    pub ctrl: PpuCtrl,           // $2000 PPUCTRL
    pub mask: PpuMask,           // $2001 PPUMASK
    pub status: PpuStatus,       // $2002 PPUSTATUS
    pub oam_addr: u8,            // $2003 OAMADDR

    // Internal registers
    pub scroll_x: u8,            // $2005 PPUSCROLL (first write)
    pub scroll_y: u8,            // $2005 PPUSCROLL (second write)
    pub addr_hi: u8,             // $2006 PPUADDR (first write)
    pub addr_lo: u8,             // $2006 PPUADDR (second write)
    pub data_buffer: u8,         // Internal read buffer for $2007

    // State tracking
    pub write_toggle: bool,      // Toggle for 2005/2006 double writes
    pub vram_addr: u16,          // Current VRAM address

    // Timing
    pub cycle: u16,              // Current cycle in scanline (0-340)
    pub scanline: u16,           // Current scanline (0-261)
    pub frame: u64,              // Frame counter

    // Memory
    pub vram: [u8; 0x800],       // Name tables (2KB internal)
    pub palette_ram: [u8; 32],   // Palette memory
    pub oam: [u8; 256],          // Object Attribute Memory (sprites)
}

impl PPU {
    pub fn new() -> Self {
        Self {
            ctrl: PpuCtrl::empty(),
            mask: PpuMask::empty(),
            status: PpuStatus::VBLANK, // Start with VBlank set for our test
            oam_addr: 0,

            scroll_x: 0,
            scroll_y: 0,
            addr_hi: 0,
            addr_lo: 0,
            data_buffer: 0,

            write_toggle: false,
            vram_addr: 0,

            cycle: 0,
            scanline: 241, // Start in VBlank period
            frame: 0,

            vram: [0; 0x800],
            palette_ram: [0; 32],
            oam: [0; 256],
        }
    }

    pub fn cpu_read(&mut self, address: u16) -> u8 {
        match address {
            0x2000 => {
                // PPUCTRL is write-only
                0
            }
            0x2001 => {
                // PPUMASK is write-only
                0
            }
            0x2002 => {
                // PPUSTATUS - reading clears VBlank flag
                let status = self.status.bits();
                self.status.remove(PpuStatus::VBLANK);
                self.write_toggle = false; // Also resets write toggle
                status
            }
            0x2003 => {
                // OAMADDR is write-only
                0
            }
            0x2004 => {
                // OAMDATA - read from OAM
                self.oam[self.oam_addr as usize]
            }
            0x2005 => {
                // PPUSCROLL is write-only
                0
            }
            0x2006 => {
                // PPUADDR is write-only
                0
            }
            0x2007 => {
                // PPUDATA - read from VRAM with buffering
                let data = self.data_buffer;
                self.data_buffer = self.read_vram(self.vram_addr);

                // Palette reads are not buffered
                if self.vram_addr >= 0x3F00 {
                    self.data_buffer
                } else {
                    // Increment VRAM address
                    self.vram_addr += if self.ctrl.contains(PpuCtrl::VRAM_INCREMENT) { 32 } else { 1 };
                    data
                }
            }
            _ => 0
        }
    }

    pub fn cpu_write(&mut self, address: u16, value: u8) {
        match address {
            0x2000 => {
                self.ctrl = PpuCtrl::from_bits_truncate(value);
            }
            0x2001 => {
                self.mask = PpuMask::from_bits_truncate(value);
            }
            0x2002 => {
                // PPUSTATUS is read-only, writes are ignored
            }
            0x2003 => {
                self.oam_addr = value;
            }
            0x2004 => {
                // OAMDATA - write to OAM
                self.oam[self.oam_addr as usize] = value;
                self.oam_addr = self.oam_addr.wrapping_add(1);
            }
            0x2005 => {
                // PPUSCROLL - first write is X, second is Y
                if !self.write_toggle {
                    self.scroll_x = value;
                } else {
                    self.scroll_y = value;
                }
                self.write_toggle = !self.write_toggle;
            }
            0x2006 => {
                // PPUADDR - first write is high byte, second is low byte
                if !self.write_toggle {
                    self.addr_hi = value;
                } else {
                    self.addr_lo = value;
                    self.vram_addr = ((self.addr_hi as u16) << 8) | (self.addr_lo as u16);
                }
                self.write_toggle = !self.write_toggle;
            }
            0x2007 => {
                // PPUDATA - write to VRAM
                self.write_vram(self.vram_addr, value);
                self.vram_addr += if self.ctrl.contains(PpuCtrl::VRAM_INCREMENT) { 32 } else { 1 };
            }
            _ => {}
        }
    }

    fn read_vram(&self, address: u16) -> u8 {
        let address = address & 0x3FFF; // Mirror down to 16KB

        match address {
            0x0000..=0x1FFF => {
                // Pattern tables - would come from cartridge CHR ROM
                0
            }
            0x2000..=0x2FFF => {
                // Name tables
                let index = (address - 0x2000) & 0x7FF; // 2KB internal VRAM
                self.vram[index as usize]
            }
            0x3000..=0x3EFF => {
                // Mirror of name tables
                let index = (address - 0x3000) & 0x7FF;
                self.vram[index as usize]
            }
            0x3F00..=0x3FFF => {
                // Palette RAM
                let index = (address - 0x3F00) & 0x1F;
                self.palette_ram[index as usize]
            }
            _ => 0
        }
    }

    fn write_vram(&mut self, address: u16, value: u8) {
        let address = address & 0x3FFF; // Mirror down to 16KB

        match address {
            0x0000..=0x1FFF => {
                // Pattern tables - would go to cartridge CHR ROM if writable
            }
            0x2000..=0x2FFF => {
                // Name tables
                let index = (address - 0x2000) & 0x7FF; // 2KB internal VRAM
                self.vram[index as usize] = value;
            }
            0x3000..=0x3EFF => {
                // Mirror of name tables
                let index = (address - 0x3000) & 0x7FF;
                self.vram[index as usize] = value;
            }
            0x3F00..=0x3FFF => {
                // Palette RAM
                let index = (address - 0x3F00) & 0x1F;
                self.palette_ram[index as usize] = value;
            }
            _ => {}
        }
    }

    pub fn clock(&mut self) {
        // Basic timing - advance cycle and scanline
        self.cycle += 1;

        if self.cycle >= 341 {
            self.cycle = 0;
            self.scanline += 1;

            if self.scanline >= 262 {
                self.scanline = 0;
                self.frame += 1;
            }

            // Set VBlank flag when entering VBlank period (scanline 241)
            if self.scanline == 241 {
                self.status.insert(PpuStatus::VBLANK);
            }

            // Clear VBlank flag when leaving VBlank period (scanline 261/pre-render)
            if self.scanline == 261 {
                self.status.remove(PpuStatus::VBLANK);
            }
        }
    }

    pub fn nmi_occurred(&self) -> bool {
        self.ctrl.contains(PpuCtrl::NMI_ENABLE) && self.status.contains(PpuStatus::VBLANK)
    }
}