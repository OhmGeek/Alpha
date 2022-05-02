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
        let instr = self.memory[self.pc as usize];
        self.handle_instruction(instr);
    }

    fn handle_instruction(&mut self, op: u64) {
        // TODO we should group ops into various files, so we can test them individually.
        // TODO don't use the op itself, but break it into <opcode> and <data>
        // Based on this, pattern match only the opcode rather than the full instruction as we're doing now.
        match op {
            0x00 => self.pc += 1,
            0x08 => self.op_lda(op),
            0x09 => self.op_ldah(),
            0x0a => self.op_ldbu(),
            _ => panic!("Error, unimplemented opcode {}", op),
        }
    }

    fn call_pal(&mut self) {
        // stub
    }

    fn op_lda(&mut self, op: u64) {
        // r[1] = r[2]
        // TODO: take the op, and figure out which two registers to move.
    }

    fn op_ldah(&mut self) {
        // stub
    }

    fn op_ldbu(&mut self) {
        // stub
    }
}
