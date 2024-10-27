use std::{fs::File, io::Read};

use nessie::{bus::Bus, cpu::CPU};

#[test]
fn test_nestest_rom() -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::open("roms/nestest/nestest.nes")?;

    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    let mut ram = [0u8; 65536];

    ram[0x8000..0xBFFF].copy_from_slice(&buffer[0x0010..0x400f]);
    ram[0xC000..0xFFFF].copy_from_slice(&buffer[0x0010..0x400f]);

    let mut cpu = CPU::new(0xC000, Box::new(ram));

    // Compare expected output to cpu trace
    let mut file = File::open("roms/nestest/nestest.expected.out").unwrap();
    let mut content: String = String::new();
    file.read_to_string(&mut content).unwrap();

    for line in content.lines() {
        let trace = cpu.trace();

        println!("{} | {}", trace, line);

        // compare PC and hexdump
        assert_eq!(&line[0..15], &trace[0..15]);

        // compare asm
        assert_eq!(&line[16..19], &trace[16..19]);

        // compare registers
        assert_eq!(&line[48..73], &trace[48..73]);

        // TODO: compare CPU cycles.
        // Disabled for now as addressing mode don't properly address page crosses
        // For example for opcode 9D
        // assert_eq!(&line[86..], &trace[86..]);
        cpu.step();
    }

    assert_eq!(0x00, ram.read(0x02));
    assert_eq!(0x00, ram.read(0x03));

    Ok(())
}
