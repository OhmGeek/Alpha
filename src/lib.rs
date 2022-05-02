pub struct Cpu {
    // program counter
    pub pc: u64,
    // memory: should follow configured memory sizes
    pub memory: [u64; 4096],
    // general purpose registers
    pub r: [u64; 32],
    // floating point registers
    pub f: [u64; 32]
}

impl Cpu {
    pub fn execute_cycle(&mut self) {
        // TODO fetch the state from memory
        // TODO process the opcode
        self.pc += 1
    }
}