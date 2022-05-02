use alpha::Cpu;

#[test]
fn it_increments_program_counter() {
    // Given we read an opcode of zero (and pc is 0)
    let mut cpu = Cpu {
        pc: 0,
        memory: [0; 4096],
        r: [0;32],
        fr: [0;32]
    };
    // When we execute
    cpu.execute_cycle();
    // Then we should increase the program counter
    assert_eq!(cpu.pc, 1);
}

#[test]
#[should_panic]
fn it_handles_unimplemented_opcode() {
    // Given we read an opcode of 1 which isn't implemented
    let mut cpu = Cpu {
        pc: 0,
        memory: [1; 4096],
        r: [0;32],
        fr: [0;32]
    };
    // When we execute, expect the test to panic
    cpu.execute_cycle();
}