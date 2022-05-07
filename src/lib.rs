pub struct Cpu {
    // program counter
    pub pc: u64,
    // memory: should follow configured memory sizes
    pub memory: [u64; 4096],
    // general purpose registers
    // R32 is always zero (we're doing a Python2 here and trusting the user)
    pub r: [u64; 32],
    // floating point registers
    // F32 is always set to 0.0 (again, we are very trusting)
    pub fr: [u64; 32],
}

impl Cpu {
    // Called each clock cycle. This handles the CPU emulation, handling a single instruction.
    // There is no pipelining implemented.
    pub fn execute_cycle(&mut self) {
        let instr = self.memory[self.pc as usize];
        self.handle_instruction(instr);
    }

    fn handle_instruction(&mut self, inst: u64) {
        // Get the instruction from the opcode
        let op = get_opcode(inst);

        // TODO we should group ops into various files, so we can test them individually.
        // TODO don't use the op itself, but break it into <opcode> and <data>
        // Based on this, pattern match only the opcode rather than the full instruction as we're doing now.
        println!("Looking at opcode {}", op);
        match op {
            // 0x00 => self.pc += 1,
            0x08 => self.op_lda(inst),
            0x09 => self.op_ldah(),
            0x0a => self.op_ldbu(),
            _ => panic!("Error, unimplemented opcode {}", op),
        }
        // Increment the program counter
        self.pc += 1;
    }

    fn call_pal(&mut self) {
        // stub
    }

    fn op_lda(&mut self, inst: u64) {
        let ra = get_ra(inst);
        let rb = get_rb(inst);
        self.r[rb as usize] = self.r[ra as usize];
        println!("lda from r{} to r{}", ra, rb)
    }

    fn op_ldah(&mut self) {
        // stub
    }

    fn op_ldbu(&mut self) {
        // stub
    }
}

fn get_ra(inst: u64) -> u64 {
    return (inst >> 21) & 0x1F;
}

fn get_rb(inst: u64) -> u64 {
    return (inst >> 16) & 0x1F;
}

fn get_opcode(inst: u64) -> u64 {
    return (inst >> 26) & 0x3F;
}
