use core::str;
use std::{cell::RefCell, fs::File, io::Read, rc::Rc};

use nessie::{bus::Bus, cartridge::Cartridge, cpu::CPU, nes::NesBus};

fn run_instr_test_rom(rom: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::open(rom)?;

    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    let cartridge = Cartridge::from_rom(&buffer);
    let bus = NesBus::new(cartridge);
    let bus = Rc::new(RefCell::new(bus));

    let pc = bus.read16(0xFFFC);
    let mut cpu = CPU::new(pc, bus.clone());

    let mut test_is_running = false;
    // Make sure that the test is running
    for _ in 0..100000 {
        cpu.step();

        if bus.read(0x6000) == 0x80
            && bus.read(0x6001) == 0xDE
            && bus.read(0x6002) == 0xB0
            && bus.read(0x6003) == 0x61
        {
            test_is_running = true;
            break;
        }
    }

    assert!(test_is_running, "Test is not running after 100,000 steps");

    while bus.read(0x6000) == 0x80 {
        cpu.step();
    }

    assert_eq!(0x00, bus.read(0x6000));

    // TODO: this should be in the Bus trait
    let mut status = vec![];
    let mut idx = 0;
    while bus.read(0x6004 + idx) != 0 {
        status.push(bus.read(0x6004 + idx));
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
