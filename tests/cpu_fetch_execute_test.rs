use alpha::Cpu;


#[test]
#[should_panic]
fn it_handles_unimplemented_opcode() {
    // Given we read an opcode of 1 which isn't implemented
    let mut cpu = Cpu {
        pc: 0,
        memory: [0xFFFFFFFF; 4096],
        r: [0;32],
        fr: [0;32]
    };
    // When we execute, expect the test to panic
    cpu.execute_cycle();
}

#[test]
fn it_should_handle_lda_with_zero_address_displacement() {
    //Init the CPU subsystem
    let mut cpu = Cpu {
        pc: 0,
        memory: [0; 4096],
        r: [0;32],
        fr: [0;32]
    };

    // LDA opcode
    cpu.memory[0] = to_inst(0x08, 0x01, 0x02);
    cpu.r[1] = 0x5;
    assert_eq!(0, cpu.r[2]);

    // After we execute, expect the data to have been loaded.
    cpu.execute_cycle();
    assert_eq!(0x5, cpu.r[2]);
    // Finally, we should have moved to the next program counter value.
    assert_eq!(1, cpu.pc)
}

fn to_inst(opcode: u64, ra: u64, rb: u64) -> u64 {
    // First 6 bits are opcode
    let opcode_part = (opcode & 0x3F) << 26;
    let ra_part = (ra & 0x1F) << 21;
    let rb_part = (rb & 0x1F) << 16;

    // OR the bits together, creating the full instruction
    return opcode_part | ra_part | rb_part;
}