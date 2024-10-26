pub trait Bus {
    fn read(&self, address: u16) -> u8;
    fn write(&mut self, address: u16, value: u8);

    fn read16(&self, address: u16) -> u16 {
        let lo = u16::from(self.read(address));
        let hi = u16::from(self.read(address + 1));
        return (hi << 8) | lo;
    }
}

impl Bus for [u8; 65536] {
    fn read(&self, address: u16) -> u8 {
        self[address as usize]
    }

    fn write(&mut self, address: u16, value: u8) {
        self[address as usize] = value;
    }
}
