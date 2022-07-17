mod logicunit;
mod opcode;
use logicunit::LU;
use opcode::Opcode;

#[derive(Debug)]
struct CPU {
    instr_reg: [bool; 4],
    result_reg: bool,
    ien_reg: bool,
    oen_reg: bool,
    data: bool,
    jmp_flag: bool,
    rtn_flag: bool,
    flgo_flag: bool,
    flgf_flag: bool,
}

impl CPU {
    fn new() -> Self {
        return Self {
            instr_reg: [false, false, false, false],
            result_reg: false,
            ien_reg: false,
            oen_reg: false,
            data: false,
            jmp_flag: false,
            rtn_flag: false,
            flgo_flag: false,
            flgf_flag: false,
        };
    }

    // TODO: Implement non-logical operations
    // STO
    // NOPO
    // STOC
    // IEN
    // OEN
    // JMP
    // RTN
    // SKZ
    // NOPF
}

pub fn run_program() {
    let mut cpu = CPU::new();
}
