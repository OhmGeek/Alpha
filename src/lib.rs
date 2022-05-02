pub struct Cpu {
    // program counter
    pub pc: u64,
    // memory: should follow configured memory sizes
    pub memory: [u64; 4096],
    // general purpose registers
    pub r: [u64; 32],
    // floating point registers
    pub fr: [u64; 32],
}

impl Cpu {
    // Called each clock cycle. This handles the CPU emulation, handling a single instruction.
    // There is no pipelining implemented.
    pub fn execute_cycle(&mut self) {
        // TODO fetch the state from memory
        // TODO process the opcode
        let instr = self.memory[self.pc as usize];
        self.handle_op(instr);
    }

    fn handle_op(&mut self, op: u64) {
        match op {
            0 => self.pc += 1,
            _ => panic!("Error, unimplemented opcode {}", op),
        }
    }
}
