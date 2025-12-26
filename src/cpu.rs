//! The core of the MC14500B CPU simulation.
//!
//! This module contains the `Cpu` struct, which represents the state of the 1-bit processor,
//! and the `run_program` function, which executes a given program.

use self::opcode::Opcode;
use logicunit::LU;
mod logicunit;
pub mod opcode;
pub mod program_loader;

#[derive(Debug)]
struct Cpu {
    pc: u8,
    program: Vec<Opcode>,
    result_reg: bool,
    ien_reg: bool,
    oen_reg: bool,
    data: bool,
    jmp_flag: bool,
    rtn_flag: bool,
}

impl Cpu {
    fn new(program: Vec<Opcode>) -> Self {
        Self {
            pc: 0,
            program,
            result_reg: false,
            ien_reg: false,
            oen_reg: false,
            data: false,
            jmp_flag: false,
            rtn_flag: false,
        }
    }

    fn step(&mut self, data_bus: bool) -> Option<bool> {
        let instruction = &self.program[self.pc as usize];
        self.data = data_bus; // Update data from the bus

        let lu = LU {
            result_reg: self.result_reg,
            data: self.data,
        };
        // By default, the result register is unchanged
        let mut next_result_reg = self.result_reg;
        let mut output = None;

        match instruction {
            // Logical Operations
            Opcode::LD
            | Opcode::LDC
            | Opcode::AND
            | Opcode::ANDC
            | Opcode::OR
            | Opcode::ORC
            | Opcode::XNOR => {
                next_result_reg = lu.operation(*instruction);
            }
            // Non-Logical Operations
            Opcode::STO => {
                if self.oen_reg {
                    output = Some(self.result_reg);
                }
            }
            Opcode::STOC => {
                if self.oen_reg {
                    output = Some(!self.result_reg);
                }
            }
            Opcode::IEN => {
                self.ien_reg = self.result_reg;
            }
            Opcode::OEN => {
                self.oen_reg = self.result_reg;
            }
            Opcode::JMP => {
                self.jmp_flag = true;
            }
            Opcode::RTN => {
                self.rtn_flag = true;
            }
            Opcode::SKZ => {
                if !self.result_reg {
                    self.pc += 1; // Skip next instruction
                }
            }
            // NOPO and NOPF are no-ops concerning the CPU state change in this model.
            Opcode::NOPO | Opcode::NOPF => {
                // No operation
            }
        };

        self.result_reg = next_result_reg;
        self.pc = (self.pc + 1) % self.program.len() as u8; // Simple loop for now
        output
    }
}

/// Runs a program on the CPU simulator for a specified number of cycles.
///
/// This function initializes a new CPU instance, enables its output register,
/// and then executes the program step-by-step. It collects any output generated
/// by `STO` or `STOC` instructions into a vector.
///
/// # Arguments
///
/// * `program` - A `Vec<Opcode>` representing the program to be executed.
/// * `num_cycles` - The number of simulation cycles to run.
///
/// # Returns
///
/// A `Vec<bool>` containing the values that were sent to the output during the simulation.
///
/// # Example
///
/// ```
/// use mc14500b_cpu::cpu::{run_program, opcode::Opcode};
///
/// let simple_program = vec![Opcode::LDC, Opcode::STO];
/// let outputs = run_program(simple_program, 2);
/// assert_eq!(outputs, vec![true]);
/// ```
pub fn run_program(program: Vec<Opcode>, num_cycles: u32) -> Vec<bool> {
    let mut cpu = Cpu::new(program);
    // Let's enable output for the STO instructions to work
    cpu.oen_reg = true;

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ien_instruction() {
        let mut cpu = Cpu::new(vec![Opcode::IEN]);
        cpu.result_reg = true;
        cpu.step(false);
        assert_eq!(cpu.ien_reg, true);

        let mut cpu = Cpu::new(vec![Opcode::IEN]);
        cpu.result_reg = false;
        cpu.step(false);
        assert_eq!(cpu.ien_reg, false);
    }

    #[test]
    fn test_oen_instruction() {
        let mut cpu = Cpu::new(vec![Opcode::OEN]);
        cpu.result_reg = true;
        cpu.step(false);
        assert_eq!(cpu.oen_reg, true);

        let mut cpu = Cpu::new(vec![Opcode::OEN]);
        cpu.result_reg = false;
        cpu.step(false);
        assert_eq!(cpu.oen_reg, false);
    }

    #[test]
    fn test_jmp_instruction() {
        let mut cpu = Cpu::new(vec![Opcode::JMP]);
        cpu.step(false);
        assert_eq!(cpu.jmp_flag, true);
    }

    #[test]
    fn test_rtn_instruction() {
        let mut cpu = Cpu::new(vec![Opcode::RTN]);
        cpu.step(false);
        assert_eq!(cpu.rtn_flag, true);
    }

    #[test]
    fn test_skz_instruction_skips() {
        // SKZ should skip the next instruction if RR is 0
        let mut cpu = Cpu::new(vec![Opcode::SKZ, Opcode::NOPF, Opcode::NOPF]);
        cpu.result_reg = false; // RR is 0
        assert_eq!(cpu.pc, 0);
        cpu.step(false);
        assert_eq!(cpu.pc, 2); // 0(SKZ) -> pc=1(skip), end of step pc=(1+1)=2
    }

    #[test]
    fn test_skz_instruction_no_skip() {
        // SKZ should NOT skip if RR is 1
        let mut cpu = Cpu::new(vec![Opcode::SKZ, Opcode::NOPF]);
        cpu.result_reg = true; // RR is 1
        assert_eq!(cpu.pc, 0);
        cpu.step(false);
        assert_eq!(cpu.pc, 1); // 0(SKZ) -> no skip, end of step pc=(0+1)=1
    }

    #[test]
    fn test_run_blink_program() {
        let program = program_loader::load_program("programs/blink.txt").unwrap();
        let outputs = run_program(program, 4);
        assert_eq!(outputs, vec![true, false]);
    }

    #[test]
    fn test_store_instructions() {
        // Test STO and STOC with OEN enabled
        let mut cpu_sto = Cpu::new(vec![Opcode::STO]);
        cpu_sto.oen_reg = true;
        cpu_sto.result_reg = true;
        assert_eq!(cpu_sto.step(false), Some(true));

        let mut cpu_stoc = Cpu::new(vec![Opcode::STOC]);
        cpu_stoc.oen_reg = true;
        cpu_stoc.result_reg = true;
        assert_eq!(cpu_stoc.step(false), Some(false));

        // Test STO and STOC with OEN disabled
        let mut cpu_sto_disabled = Cpu::new(vec![Opcode::STO]);
        cpu_sto_disabled.result_reg = true;
        assert_eq!(cpu_sto_disabled.step(false), None);

        let mut cpu_stoc_disabled = Cpu::new(vec![Opcode::STOC]);
        cpu_stoc_disabled.result_reg = true;
        assert_eq!(cpu_stoc_disabled.step(false), None);
    }
}
