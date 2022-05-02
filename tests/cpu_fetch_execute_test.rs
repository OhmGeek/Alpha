use alpha::Cpu;

#[test]
fn it_increments_program_counter() {
    let mut cpu = Cpu {
        pc: 0,
        memory: [0; 4096],
        r: [0;32],
        f: [0;32]
    };
    cpu.execute_cycle();
    assert_eq!(cpu.pc, 1);
}