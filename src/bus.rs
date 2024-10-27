use std::{cell::RefCell, rc::Rc};

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

impl<B: Bus> Bus for Rc<RefCell<B>> {
    fn read(&self, address: u16) -> u8 {
        self.borrow().read(address)
    }

    fn write(&mut self, address: u16, value: u8) {
        self.borrow_mut().write(address, value)
    }
}

impl Bus for Rc<RefCell<dyn Bus>> {
    fn read(&self, address: u16) -> u8 {
        self.borrow().read(address)
    }

    fn write(&mut self, address: u16, value: u8) {
        self.borrow_mut().write(address, value)
    }
}
