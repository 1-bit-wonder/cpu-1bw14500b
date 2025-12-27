use self::opcode::Opcode;
use logicunit::LU;
mod logicunit;
pub mod opcode;
pub mod program_loader;

#[derive(Debug, Default)]
pub struct Cpu {
    pub pc: u8,
    pub program: Vec<Opcode>,
    pub lu: LU,
    pub jmp_flag: bool,
    pub rtn_flag: bool,
}

impl Cpu {
    pub fn new(program: Vec<Opcode>) -> Self {
        Self {
            program,
            ..Self::default()
        }
    }

    pub fn step(&mut self, data_bus: bool) -> Option<bool> {
        let instruction = &self.program[self.pc as usize];

        self.lu.execute(*instruction, data_bus);

        let mut output = None;
        match instruction {
            // Non-LU Operations
            Opcode::STO => {
                if self.lu.oen {
                    output = Some(self.lu.result_reg);
                }
            }
            Opcode::STOC => {
                if self.lu.oen {
                    output = Some(!self.lu.result_reg);
                }
            }
            Opcode::JMP => {
                self.jmp_flag = true;
            }
            Opcode::RTN => {
                self.rtn_flag = true;
            }
            Opcode::SKZ => {
                if !self.lu.result_reg {
                    self.pc += 1; // Skip next instruction
                }
            }
            // NOPO, NOPF, and all logical ops are handled by LU or are no-ops here
            _ => {}
        };

        self.pc = (self.pc + 1) % self.program.len() as u8; // Simple loop for now
        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test helper to run a program and collect outputs.
    fn run_program(program: Vec<Opcode>, num_cycles: u32) -> Vec<bool> {
        let mut cpu = Cpu::new(program);
        // Manually enable IEN and OEN for testing purposes.
        cpu.lu.ien = true;
        cpu.lu.oen = true;

        let mut outputs = Vec::new();

        for _ in 0..num_cycles {
            // In this simple model, the data bus is always false (0)
            let data_bus = false;
            if let Some(output) = cpu.step(data_bus) {
                outputs.push(output);
            }
        }
        outputs
    }

    #[test]
    fn test_gating_instructions() {
        let mut cpu = Cpu::new(vec![Opcode::IEN, Opcode::OEN]);

        // Can't enable IEN/OEN if RR is 0
        cpu.step(false); // Execute IEN
        assert_eq!(cpu.lu.ien, false);
        cpu.lu.result_reg = true;
        cpu.step(false); // Execute OEN
        assert_eq!(cpu.lu.oen, true);
    }

    #[test]
    fn test_jmp_and_rtn_flags() {
        let mut cpu = Cpu::new(vec![Opcode::JMP, Opcode::RTN]);
        assert_eq!(cpu.jmp_flag, false);
        cpu.step(false); // Execute JMP
        assert_eq!(cpu.jmp_flag, true);

        assert_eq!(cpu.rtn_flag, false);
        cpu.step(false); // Execute RTN
        assert_eq!(cpu.rtn_flag, true);
    }

    #[test]
    fn test_skz_instruction() {
        let mut cpu = Cpu::new(vec![Opcode::SKZ, Opcode::JMP, Opcode::RTN]);

        // Skips JMP if RR is 0
        cpu.lu.result_reg = false;
        cpu.step(false); // Execute SKZ, pc should increment twice
        assert_eq!(cpu.pc, 2);
        assert_eq!(cpu.jmp_flag, false); // JMP was skipped

        // Does not skip if RR is 1
        cpu.pc = 0; // Reset pc
        cpu.lu.result_reg = true;
        cpu.step(false); // Execute SKZ
        assert_eq!(cpu.pc, 1);
    }

    #[test]
    fn test_store_instructions() {
        let mut cpu = Cpu::new(vec![Opcode::STO, Opcode::STOC]);
        cpu.lu.oen = true;
        cpu.lu.result_reg = true;

        let output1 = cpu.step(false); // Execute STO
        assert_eq!(output1, Some(true));

        let output2 = cpu.step(false); // Execute STOC
        assert_eq!(output2, Some(false));
    }

    // Integration tests for programs
    #[test]
    fn test_run_blink_program() {
        let program = program_loader::load_program("programs/blink.txt").unwrap();
        // Program is 4 cycles. LDC, STO, LD, STO. Output: [true, false]
        let outputs = run_program(program, 4);
        assert_eq!(outputs, vec![true, false]);
    }

    #[test]
    fn test_run_skip_store_program() {
        let program = program_loader::load_program("programs/skip_store.txt").unwrap();
        // Program is 8 cycles. Output: [false, true, true]
        let outputs = run_program(program, 8);
        assert_eq!(outputs, vec![false, true, true]);
    }

    #[test]
    fn test_run_logic_demo_program() {
        let program = program_loader::load_program("programs/logic_demo.txt").unwrap();
        // Program is 8 cycles. Output: [true, false, true, false]
        let outputs = run_program(program, 8);
        assert_eq!(outputs, vec![true, false, true, false]);
    }
}
