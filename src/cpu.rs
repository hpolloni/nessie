use std::{cell::RefCell, rc::Rc};

use assert_matches::debug_assert_matches;

use bitflags::bitflags;

use crate::{
    bus::Bus,
    opcodes::{Address, AddressingMode, OPCODE_TABLE},
};

bitflags! {
    #[derive(Copy, Clone, Debug)]
    struct StatusFlags: u8 {
        const C = 1;
        const Z = 1 << 1;
        const I = 1 << 2;
        const D = 1 << 3;
        const B = 1 << 4;
        const X = 1 << 5;
        const O = 1 << 6;
        const N = 1 << 7;
    }
}

pub struct CPU {
    accumulator: u8,
    x_register: u8,
    y_register: u8,
    program_counter: u16,
    remaining_cycles: u8,
    bus: Rc<RefCell<dyn Bus>>,
    status: StatusFlags,
    total_cycles: u64,
    stack_pointer: u8,
}

impl CPU {
    pub fn new(pc: u16, bus: Rc<RefCell<dyn Bus>>) -> Self {
        Self {
            accumulator: 0x00,
            x_register: 0x00,
            y_register: 0x00,
            program_counter: pc,
            remaining_cycles: 0,
            total_cycles: 0,
            stack_pointer: 0xfd,
            bus,
            status: StatusFlags::from_bits_truncate(0x24),
        }
    }

    fn cycle(&mut self) {
        if self.remaining_cycles == 0 {
            let opcode = self.bus.read(self.program_counter);

            self.program_counter += 1;

            let op = OPCODE_TABLE[opcode as usize];

            let address = self.resolve_address(op.addressing());

            self.program_counter += op.len() - 1;

            op.execute(self, address);

            self.remaining_cycles += op.cycles();
        }
        self.total_cycles += 1;
        self.remaining_cycles -= 1;
    }

    pub fn step(&mut self) {
        self.cycle();
        while self.remaining_cycles != 0 {
            self.cycle();
        }
    }

    pub fn run_until_brk(&mut self) {
        while !self.status.contains(StatusFlags::B) {
            self.step()
        }
    }

    fn set_zero_or_neg_flags(&mut self, value: u8) {
        self.status.set(StatusFlags::Z, value == 0);
        self.status
            .set(StatusFlags::N, value & StatusFlags::N.bits() != 0);
    }

    pub fn trace(&self) -> String {
        let opcode = self.bus.read(self.program_counter);

        let op = OPCODE_TABLE[opcode as usize];

        let hexdump = self.hexdump(self.program_counter, self.program_counter + op.len());

        let asm = format!("{}{:28}", op.name(), " ");
        let ppu = " ".repeat(11);
        format!(
            "{:04X}  {:9} {} A:{:02X} X:{:02X} Y:{:02X} P:{:02X} SP:{:02X} {} CYC:{}",
            self.program_counter,
            hexdump,
            asm,
            self.accumulator,
            self.x_register,
            self.y_register,
            self.status.bits(),
            self.stack_pointer,
            ppu,
            self.total_cycles + 7
        ) // TODO figure this out
    }

    // TODO: consider if this should be in the Bus trait instead
    fn hexdump(&self, start: u16, end: u16) -> String {
        let mut hexdump = String::new();
        for addr in start..end {
            hexdump.push_str(&format!("{:02X} ", self.bus.read(addr)));
        }
        hexdump
    }
}

fn s8_to_u16(value: u8) -> u16 {
    let mut value = u16::from(value);
    if value & 0x80 > 0 {
        value |= 0xff00;
    }
    return value;
}

const STACK_PAGE: u16 = 0x0100;

// Operations
impl CPU {
    pub(crate) fn adc(&mut self, address: Address) {
        debug_assert_matches!(address, Address::Absolute(address) => {
            let value = self.bus.read(address);
            let carry = self.status.contains(StatusFlags::C) as u16;
            let result: u16 = u16::from(self.accumulator) + u16::from(value) + carry;
            let result_u8 = result as u8;

            self.status.set(StatusFlags::C, result > u16::from(u8::max_value()));
            self.status.set(
                StatusFlags::O,
                (!(self.accumulator ^ value)
                    & (self.accumulator ^ result_u8)
                    & StatusFlags::N.bits())
                    > 0,
            );
            self.set_zero_or_neg_flags(result_u8);

            self.accumulator = result_u8;
        });
    }

    pub(crate) fn ahx(&mut self, _address: Address) {
        todo!("ahx Not Implemented")
    }

    pub(crate) fn alr(&mut self, address: Address) {
        self.and(address);
        self.lsr(Address::Implied);
    }

    pub(crate) fn anc(&mut self, address: Address) {
        self.and(address);
        self.status.set(StatusFlags::C, self.accumulator >> 7 == 1);
    }

    pub(crate) fn and(&mut self, address: Address) {
        debug_assert_matches!(address, Address::Absolute(address) => {
            let value = self.bus.read(address);
            self.accumulator &= value;
            self.set_zero_or_neg_flags(self.accumulator);
        });
    }

    pub(crate) fn arr(&mut self, _address: Address) {
        todo!("arr Not Implemented")
    }

    // TODO: find a way to refactor asl, ror and lsr
    pub(crate) fn asl(&mut self, address: Address) {
        let mut inner = |value: u8| -> u8 {
            self.status.set(StatusFlags::C, value >> 7 == 1);
            let value = value << 1;
            self.status.set(StatusFlags::Z, value == 0);
            self.status
                .set(StatusFlags::N, value & StatusFlags::N.bits() != 0);
            value
        };

        match address {
            Address::Implied => self.accumulator = inner(self.accumulator),
            Address::Absolute(address) => {
                let value = inner(self.bus.read(address));
                self.bus.write(address, value);
            }
            _ => panic!("ASL opcode with relative addressing"),
        }
    }

    pub(crate) fn axs(&mut self, _address: Address) {
        todo!("axs Not Implemented")
    }

    fn branch(&mut self, address: Address, cond: bool) {
        debug_assert_matches!(address,
        Address::Relative(address) => {
            let address = s8_to_u16(address).wrapping_add(self.program_counter);

            if cond {
                if address & 0xff00 != self.program_counter & 0xff00 {
                    self.remaining_cycles += 2;
                } else {
                    self.remaining_cycles += 1;
                }
                self.program_counter = address;
            }
        });
    }

    pub(crate) fn bcc(&mut self, address: Address) {
        self.branch(address, !self.status.contains(StatusFlags::C));
    }

    pub(crate) fn bcs(&mut self, address: Address) {
        self.branch(address, self.status.contains(StatusFlags::C));
    }

    pub(crate) fn beq(&mut self, address: Address) {
        self.branch(address, self.status.contains(StatusFlags::Z));
    }

    pub(crate) fn bit(&mut self, address: Address) {
        debug_assert_matches!(address, Address::Absolute(address) => {
            let value = self.bus.read(address);
            let mask = StatusFlags::from_bits_truncate(value);

            self.status.set(StatusFlags::Z, self.accumulator & value == 0);
            self.status.set(StatusFlags::O, mask.contains(StatusFlags::O));
            self.status.set(StatusFlags::N, mask.contains(StatusFlags::N));
        });
    }

    pub(crate) fn bmi(&mut self, address: Address) {
        self.branch(address, self.status.contains(StatusFlags::N));
    }

    pub(crate) fn bne(&mut self, address: Address) {
        self.branch(address, !self.status.contains(StatusFlags::Z));
    }

    pub(crate) fn bpl(&mut self, address: Address) {
        self.branch(address, !self.status.contains(StatusFlags::N));
    }

    pub(crate) fn brk(&mut self, address: Address) {
        debug_assert_matches!(address, Address::Implied);

        self.status |= StatusFlags::B;
        // TODO: stack manipulation
    }

    pub(crate) fn bvc(&mut self, address: Address) {
        self.branch(address, !self.status.contains(StatusFlags::O));
    }

    pub(crate) fn bvs(&mut self, address: Address) {
        self.branch(address, self.status.contains(StatusFlags::O));
    }

    pub(crate) fn clc(&mut self, address: Address) {
        debug_assert_matches!(address, Address::Implied);
        self.status -= StatusFlags::C;
    }

    pub(crate) fn cld(&mut self, address: Address) {
        debug_assert_matches!(address, Address::Implied);

        self.status -= StatusFlags::D;
    }

    pub(crate) fn cli(&mut self, address: Address) {
        debug_assert_matches!(address, Address::Implied);

        self.status -= StatusFlags::I;
    }

    pub(crate) fn clv(&mut self, address: Address) {
        debug_assert_matches!(address, Address::Implied);

        self.status -= StatusFlags::O;
    }

    fn compare(&mut self, address: Address, register_value: u8) {
        debug_assert_matches!(address, Address::Absolute(address) => {
            let value = self.bus.read(address);

            self.status.set(StatusFlags::C, register_value >= value);

            let cmp = register_value.wrapping_sub(value);
            self.set_zero_or_neg_flags(cmp);
        });
    }

    pub(crate) fn cmp(&mut self, address: Address) {
        self.compare(address, self.accumulator);
    }

    pub(crate) fn cpx(&mut self, address: Address) {
        self.compare(address, self.x_register);
    }

    pub(crate) fn cpy(&mut self, address: Address) {
        self.compare(address, self.y_register);
    }

    pub(crate) fn dcp(&mut self, address: Address) {
        self.dec(address);
        self.cmp(address)
    }

    pub(crate) fn dec(&mut self, address: Address) {
        debug_assert_matches!(address, Address::Absolute(address) => {
            let value = self.bus.read(address).wrapping_sub(1);
            self.set_zero_or_neg_flags(value);
            self.bus.write(address, value);
        });
    }

    pub(crate) fn dex(&mut self, address: Address) {
        debug_assert_matches!(address, Address::Implied);

        self.x_register = self.x_register.wrapping_sub(1);
        self.set_zero_or_neg_flags(self.x_register);
    }

    pub(crate) fn dey(&mut self, address: Address) {
        debug_assert_matches!(address, Address::Implied);

        self.y_register = self.y_register.wrapping_sub(1);
        self.set_zero_or_neg_flags(self.y_register);
    }

    pub(crate) fn eor(&mut self, address: Address) {
        debug_assert_matches!(address, Address::Absolute(address) => {
            let value = self.bus.read(address);
            self.accumulator ^= value;
            self.set_zero_or_neg_flags(self.accumulator);
        });
    }

    pub(crate) fn inc(&mut self, address: Address) {
        debug_assert_matches!(address, Address::Absolute(address) => {
            let value = self.bus.read(address).wrapping_add(1);
            self.set_zero_or_neg_flags(value);
            self.bus.write(address, value);
        });
    }

    pub(crate) fn inx(&mut self, address: Address) {
        debug_assert_matches!(address, Address::Implied);

        self.x_register = self.x_register.wrapping_add(1);
        self.set_zero_or_neg_flags(self.x_register);
    }

    pub(crate) fn iny(&mut self, address: Address) {
        debug_assert_matches!(address, Address::Implied);

        self.y_register = self.y_register.wrapping_add(1);
        self.set_zero_or_neg_flags(self.y_register);
    }

    pub(crate) fn isc(&mut self, address: Address) {
        self.inc(address);
        self.sbc(address);
    }

    pub(crate) fn jmp(&mut self, address: Address) {
        debug_assert_matches!(address, Address::Absolute(address) => self.program_counter = address);
    }

    pub(crate) fn jsr(&mut self, address: Address) {
        debug_assert_matches!(address, Address::Absolute(address) => {
            self.push_stack_16(self.program_counter - 1);
            self.program_counter = address;
        });
    }

    pub(crate) fn las(&mut self, _address: Address) {
        todo!("las Not Implemented")
    }

    pub(crate) fn lax(&mut self, address: Address) {
        self.lda(address);
        self.ldx(address);
    }
    pub(crate) fn lda(&mut self, address: Address) {
        debug_assert_matches!(address, Address::Absolute(address) => {
            self.accumulator = self.bus.read(address);
            self.set_zero_or_neg_flags(self.accumulator);
        });
    }

    pub(crate) fn ldx(&mut self, address: Address) {
        debug_assert_matches!(address, Address::Absolute(address) => {
            self.x_register = self.bus.read(address);
            self.set_zero_or_neg_flags(self.x_register);
        });
    }

    pub(crate) fn ldy(&mut self, address: Address) {
        debug_assert_matches!(address, Address::Absolute(address) => {
            self.y_register = self.bus.read(address);
            self.set_zero_or_neg_flags(self.y_register);
        });
    }

    pub(crate) fn lsr(&mut self, address: Address) {
        let mut inner = |value: u8| -> u8 {
            self.status.set(StatusFlags::C, value & 1 == 1);
            let shifted_value = value >> 1;
            self.status.set(StatusFlags::Z, shifted_value == 0);
            self.status.set(StatusFlags::N, false);
            return shifted_value;
        };

        match address {
            Address::Implied => self.accumulator = inner(self.accumulator),
            Address::Absolute(address) => {
                let value = inner(self.bus.read(address));
                self.bus.write(address, value);
            }
            _ => panic!("LSR opcode with relative addressing"),
        }
    }

    pub(crate) fn nop(&mut self, _address: Address) {
        // Do nothing (NOP)
    }

    pub(crate) fn ora(&mut self, address: Address) {
        debug_assert_matches!(address, Address::Absolute(address) => {
            let value = self.bus.read(address);
            self.accumulator |= value;
            self.set_zero_or_neg_flags(self.accumulator);
        });
    }

    pub(crate) fn pha(&mut self, address: Address) {
        debug_assert_matches!(address, Address::Implied);

        self.push_stack(self.accumulator);
    }

    pub(crate) fn php(&mut self, address: Address) {
        debug_assert_matches!(address, Address::Implied);

        self.push_stack((self.status | StatusFlags::B).bits());
    }

    pub(crate) fn pla(&mut self, address: Address) {
        debug_assert_matches!(address, Address::Implied);

        self.accumulator = self.pop_stack();
        self.set_zero_or_neg_flags(self.accumulator);
    }

    pub(crate) fn plp(&mut self, address: Address) {
        debug_assert_matches!(address, Address::Implied);

        let old_status = self.status;
        let mut new_status = StatusFlags::from_bits_truncate(self.pop_stack());

        new_status.set(StatusFlags::B, old_status.contains(StatusFlags::B));
        new_status.set(StatusFlags::X, old_status.contains(StatusFlags::X));

        self.status = new_status;
    }

    pub(crate) fn rla(&mut self, address: Address) {
        self.rol(address);
        self.and(address);
    }

    pub(crate) fn rol(&mut self, address: Address) {
        let mut inner = |value: u8| -> u8 {
            // Save carry flag
            let carry = if self.status.contains(StatusFlags::C) {
                1
            } else {
                0
            };

            self.status.set(StatusFlags::C, value >> 7 == 1);

            let value = value << 1 | carry;

            self.status.set(StatusFlags::Z, value == 0);
            self.status
                .set(StatusFlags::N, value & StatusFlags::N.bits() != 0);
            value
        };

        match address {
            Address::Implied => self.accumulator = inner(self.accumulator),
            Address::Absolute(address) => {
                let value = inner(self.bus.read(address));
                self.bus.write(address, value);
            }
            _ => panic!("ROR opcode with relative addressing"),
        }
    }

    pub(crate) fn ror(&mut self, address: Address) {
        let mut inner = |value: u8| -> u8 {
            // Save carry flag
            let carry = if self.status.contains(StatusFlags::C) {
                1
            } else {
                0
            };

            self.status.set(StatusFlags::C, value & 1 == 1);

            let value = value >> 1 | carry << 7;

            self.status.set(StatusFlags::Z, value == 0);
            self.status
                .set(StatusFlags::N, value & StatusFlags::N.bits() != 0);
            value
        };

        match address {
            Address::Implied => self.accumulator = inner(self.accumulator),
            Address::Absolute(address) => {
                let value = inner(self.bus.read(address));
                self.bus.write(address, value);
            }
            _ => panic!("ROR opcode with relative addressing"),
        }
    }

    pub(crate) fn rra(&mut self, address: Address) {
        self.ror(address);
        self.adc(address);
    }

    pub(crate) fn rti(&mut self, address: Address) {
        self.plp(address);
        self.program_counter = self.pop_stack_16();
    }

    pub(crate) fn rts(&mut self, address: Address) {
        debug_assert_matches!(address, Address::Implied);

        self.program_counter = self.pop_stack_16() + 1;
    }

    pub(crate) fn sax(&mut self, address: Address) {
        debug_assert_matches!(address, Address::Absolute(address) => self.bus.write(address, self.accumulator & self.x_register));
    }

    pub(crate) fn sbc(&mut self, address: Address) {
        debug_assert_matches!(address, Address::Absolute(address) => {
            let value = self.bus.read(address);
            let carry = self.status.contains(StatusFlags::C) as u16;

            let result = u16::from(self.accumulator) + u16::from(!value) + carry;

            let result_u8 = result as u8;

            self.status.set(StatusFlags::C, result > u16::from(u8::max_value()));
            self.status.set(StatusFlags::Z, result_u8 == 0);
            self.status.set(
                StatusFlags::O,
                ((self.accumulator ^ value)
                    & (self.accumulator ^ result_u8)
                    & StatusFlags::N.bits())
                    > 0,
            );

            self.status.set(StatusFlags::N, result_u8 & StatusFlags::N.bits() > 0);

            self.accumulator = result_u8;
        });
    }

    pub(crate) fn sec(&mut self, address: Address) {
        debug_assert_matches!(address, Address::Implied);

        self.status |= StatusFlags::C;
    }

    pub(crate) fn sed(&mut self, address: Address) {
        debug_assert_matches!(address, Address::Implied);

        self.status |= StatusFlags::D;
    }

    pub(crate) fn sei(&mut self, address: Address) {
        debug_assert_matches!(address, Address::Implied);

        self.status |= StatusFlags::I;
    }

    pub(crate) fn shx(&mut self, _address: Address) {
        todo!("shx Not Implemented")
    }

    pub(crate) fn shy(&mut self, _address: Address) {
        todo!("shy Not Implemented")
    }

    pub(crate) fn slo(&mut self, address: Address) {
        self.asl(address);
        self.ora(address);
    }

    pub(crate) fn sre(&mut self, address: Address) {
        self.lsr(address);
        self.eor(address);
    }

    pub(crate) fn sta(&mut self, address: Address) {
        debug_assert_matches!(address, Address::Absolute(address) => self.bus.write(address, self.accumulator));
    }

    pub(crate) fn stx(&mut self, address: Address) {
        debug_assert_matches!(address, Address::Absolute(address) => self.bus.write(address, self.x_register));
    }

    pub(crate) fn sty(&mut self, address: Address) {
        debug_assert_matches!(address, Address::Absolute(address) => self.bus.write(address, self.y_register));
    }

    pub(crate) fn tas(&mut self, _address: Address) {
        todo!("tas Not Implemented")
    }

    pub(crate) fn tax(&mut self, address: Address) {
        debug_assert_matches!(address, Address::Implied);

        self.x_register = self.accumulator;

        self.set_zero_or_neg_flags(self.x_register);
    }

    pub(crate) fn tay(&mut self, address: Address) {
        debug_assert_matches!(address, Address::Implied);

        self.y_register = self.accumulator;

        self.set_zero_or_neg_flags(self.y_register);
    }

    pub(crate) fn tsx(&mut self, address: Address) {
        debug_assert_matches!(address, Address::Implied);

        self.x_register = self.stack_pointer;
        self.set_zero_or_neg_flags(self.x_register);
    }

    pub(crate) fn txa(&mut self, address: Address) {
        debug_assert_matches!(address, Address::Implied);

        self.accumulator = self.x_register;
        self.set_zero_or_neg_flags(self.x_register);
    }

    pub(crate) fn txs(&mut self, address: Address) {
        debug_assert_matches!(address, Address::Implied);

        self.stack_pointer = self.x_register;
    }

    pub(crate) fn tya(&mut self, address: Address) {
        debug_assert_matches!(address, Address::Implied);

        self.accumulator = self.y_register;
        self.set_zero_or_neg_flags(self.y_register);
    }

    pub(crate) fn xaa(&mut self, _address: Address) {
        todo!("xaa Not Implemented")
    }
}

// Stack manipulation functions
impl CPU {
    fn pop_stack(&mut self) -> u8 {
        self.stack_pointer = self.stack_pointer.wrapping_add(1);
        self.bus.read(STACK_PAGE + u16::from(self.stack_pointer))
    }

    fn pop_stack_16(&mut self) -> u16 {
        let lo = u16::from(self.pop_stack());
        let hi = u16::from(self.pop_stack());
        return (hi << 8) | lo;
    }

    fn push_stack_16(&mut self, data: u16) {
        self.push_stack((data >> 8) as u8);
        self.push_stack(data as u8);
    }

    fn push_stack(&mut self, data: u8) {
        self.bus
            .write(STACK_PAGE + u16::from(self.stack_pointer), data);
        self.stack_pointer = self.stack_pointer.wrapping_sub(1);
    }
}

impl CPU {
    fn resolve_address(&self, addressing: AddressingMode) -> Address {
        match addressing {
            AddressingMode::Absolute => self.absolute(0),
            AddressingMode::AbsoluteX => self.absolute(self.x_register),
            AddressingMode::AbsoluteY => self.absolute(self.y_register),
            AddressingMode::Immediate => Address::Absolute(self.program_counter),
            AddressingMode::Implied => Address::Implied,
            AddressingMode::Indirect => self.indirect(),
            AddressingMode::IndirectX => self.indirect_x(),
            AddressingMode::IndirectY => self.indirect_y(),
            AddressingMode::Relative => self.relative(),
            AddressingMode::ZeroPage => self.zero_page(0),
            AddressingMode::ZeroPageX => self.zero_page(self.x_register),
            AddressingMode::ZeroPageY => self.zero_page(self.y_register),
        }
    }

    fn relative(&self) -> Address {
        let relative_address = self.bus.read(self.program_counter);
        Address::Relative(relative_address)
    }

    fn zero_page(&self, offset: u8) -> Address {
        let address = self.bus.read(self.program_counter).wrapping_add(offset);
        Address::Absolute(address as u16)
    }

    fn absolute(&self, offset: u8) -> Address {
        let address = self.bus.read16(self.program_counter);
        let offset_address: u16 = address.wrapping_add(offset as u16);
        Address::Absolute(offset_address)
    }

    fn indirect(&self) -> Address {
        let indirect_address = self.bus.read16(self.program_counter);

        let page = indirect_address & 0xff00;

        let address_hi = u16::from(self.bus.read(page | ((indirect_address + 1) & 0xff))) << 8;
        let address_lo = u16::from(self.bus.read(indirect_address));

        let address = address_hi | address_lo;

        Address::Absolute(address)
    }

    fn indirect_x(&self) -> Address {
        let indirect_address = self
            .bus
            .read(self.program_counter)
            .wrapping_add(self.x_register);
        let indirect_address_plus_one = indirect_address.wrapping_add(1) as u16;

        let address_hi = (self.bus.read(indirect_address_plus_one) as u16) << 8;
        let address_lo = self.bus.read(indirect_address as u16) as u16;

        let address = address_hi | address_lo;

        Address::Absolute(address)
    }

    fn indirect_y(&self) -> Address {
        let indirect_address = self.bus.read(self.program_counter);
        let indirect_address_plus_one = indirect_address.wrapping_add(1) as u16;

        let address_hi = (self.bus.read(indirect_address_plus_one) as u16) << 8;
        let address_lo = self.bus.read(indirect_address as u16) as u16;

        let address = address_hi | address_lo;

        let offset_address = address.wrapping_add(u16::from(self.y_register));

        Address::Absolute(offset_address)
    }
}

#[cfg(test)]
mod tests {

    use std::{cell::RefCell, rc::Rc};

    use crate::bus::Bus;

    use super::CPU;

    #[test]
    fn test_simple_program() {
        let program = [
            0xa9, 0x10, // LDA #$10     -> A = #$10
            0x85, 0x20, // STA $20      -> $20 = #$10
            0xa9, 0x01, // LDA #$1      -> A = #$1
            0x65, 0x20, // ADC $20      -> A = #$11
            0x85, 0x21, // STA $21      -> $21=#$11
            0xe6, 0x21, // INC $21      -> $21=#$12
            0xa4, 0x21, // LDY $21      -> Y=#$12
            0xc8, // INY          -> Y=#$13
            0x00, // BRK
        ];

        let mut ram = [0u8; 65536];
        ram[0x0000..program.len()].copy_from_slice(&program);

        let bus = Rc::new(RefCell::new(ram));

        let mut cpu = CPU::new(0x00, bus.clone());

        // LDA #$10
        cpu.step();

        assert_eq!(cpu.accumulator, 0x10);

        // STA $20
        cpu.step();

        assert_eq!(bus.read(0x20), 0x10);

        // LDA #$1
        cpu.step();
        assert_eq!(cpu.accumulator, 0x01);

        // ADC $20
        cpu.step();
        assert_eq!(cpu.accumulator, 0x11);

        // STA $21
        cpu.step();
        assert_eq!(bus.read(0x21), 0x11);

        // INC $21
        cpu.step();
        assert_eq!(bus.read(0x21), 0x12);

        // LDY $21
        cpu.step();
        assert_eq!(cpu.y_register, 0x12);

        // INY
        cpu.step();
        assert_eq!(cpu.y_register, 0x13);
    }

    #[test]
    fn test_euclid_algo() {
        // From https://github.com/mre/mos6502/blob/master/examples/asm/euclid/euclid.a65
        let program = [
            // .algo
            0xa5, 0x00, // LDA $00
            // .algo_
            0x38, // SEC
            0xe5, 0x01, // SBC $01
            0xf0, 0x07, // BEQ end
            0x30, 0x08, // BMI swap
            0x85, 0x00, // STA $00
            0x4c, 0x12, 0x00, // JMP algo_
            // .end
            0xa5, 0x00, // LDA $00
            0x00, // .swap
            0xa6, 0x00, // LDX $00
            0xa4, 0x01, // LDY $01
            0x86, 0x01, // STX $01
            0x84, 0x00, // STY $00
            0x4c, 0x10, 0x00, // JMP algo
        ];

        let mut ram = [0u8; 65536];
        ram[0x00] = 30;
        ram[0x01] = 20;
        ram[0x10..0x10 + program.len()].copy_from_slice(&program);

        let bus = Rc::new(RefCell::new(ram));

        let mut cpu = CPU::new(0x10, bus);

        cpu.run_until_brk();

        assert_eq!(10, cpu.accumulator);
    }
}
