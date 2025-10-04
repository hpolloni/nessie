use std::{cell::RefCell, fs::File, io::Read, rc::Rc};

use nessie::{bus::Bus, cartridge::Cartridge, cpu::CPU, nes::NesBus};

#[test]
fn test_nestest_rom() -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::open("roms/external/other/nestest.nes")?;

    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    let cartridge = Cartridge::from_rom(&buffer);
    let bus = NesBus::new(cartridge);
    let bus = Rc::new(RefCell::new(bus));

    let mut cpu = CPU::new(0xC000, bus.clone());

    // Compare expected output to cpu trace
    let mut file = File::open("roms/nestest/nestest.expected.out")?;
    let mut content: String = String::new();
    file.read_to_string(&mut content).unwrap();

    for line in content.lines() {
        let trace = cpu.trace();

        println!("{} | {}", line, trace);

        // compare PC and hexdump
        assert_eq!(&line[0..15], &trace[0..15]);

        // compare asm
        assert_eq!(&line[16..19], &trace[16..19]);

        // compare registers
        assert_eq!(&line[48..73], &trace[48..73]);

        // TODO: compare CPU cycles.
        // Disabled for now as addressing mode don't properly address page crosses
        // For example for opcode 9D
        assert_eq!(&line[86..], &trace[86..]);
        cpu.step();
    }

    assert_eq!(
        0x00,
        bus.read(0x02),
        "nestest error code: {:4X}",
        bus.read(0x02)
    );
    assert_eq!(0x00, bus.read(0x03));

    Ok(())
}
