use super::opcode::Opcode;

#[derive(Debug)]
pub struct LU {
    pub result_reg: bool,
    pub data: bool,
}

impl LU {
    pub fn new() -> Self {
        return Self {
            result_reg: false,
            data: false,
        };
    }

    pub fn operation(self, code: Opcode) -> bool {
        match code {
            Opcode::LD => self.data,
            Opcode::LDC => !self.data,
            Opcode::AND => self.result_reg & self.data,
            Opcode::ANDC => self.result_reg & !self.data,
            Opcode::OR => self.result_reg | self.data,
            Opcode::ORC => self.result_reg | !self.data,
            Opcode::XNOR => !(self.result_reg ^ self.data),
            _ => self.result_reg, // Not an LU operation, don't change result
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(Opcode::LD,  false; "ld")]
    #[test_case(Opcode::LDC,  true; "ldc")]
    #[test_case(Opcode::AND,  false; "and")]
    #[test_case(Opcode::ANDC, false; "andc")]
    #[test_case(Opcode::OR, false; "or")]
    #[test_case(Opcode::ORC, true; "orc")]
    #[test_case(Opcode::XNOR, true; "xnor")]
    fn r0_d0(opcode: Opcode, expected: bool) {
        let lu = LU {
            result_reg: false,
            data: false,
        };

        assert_eq!(lu.operation(opcode), expected);
    }

    #[test_case(Opcode::LD,  true; "ld")]
    #[test_case(Opcode::LDC,  false; "ldc")]
    #[test_case(Opcode::AND,  false; "and")]
    #[test_case(Opcode::ANDC, false; "andc")]
    #[test_case(Opcode::OR, true; "or")]
    #[test_case(Opcode::ORC, false; "orc")]
    #[test_case(Opcode::XNOR, false; "xnor")]
    fn r0_d1(opcode: Opcode, expected: bool) {
        let lu = LU {
            result_reg: false,
            data: true,
        };

        assert_eq!(lu.operation(opcode), expected);
    }

    #[test_case(Opcode::LD,  false; "ld")]
    #[test_case(Opcode::LDC,  true; "ldc")]
    #[test_case(Opcode::AND,  false; "and")]
    #[test_case(Opcode::ANDC, true; "andc")]
    #[test_case(Opcode::OR, true; "or")]
    #[test_case(Opcode::ORC, true; "orc")]
    #[test_case(Opcode::XNOR, false; "xnor")]
    fn r1_d0(opcode: Opcode, expected: bool) {
        let lu = LU {
            result_reg: true,
            data: false,
        };

        assert_eq!(lu.operation(opcode), expected);
    }

    #[test_case(Opcode::LD,  true; "ld")]
    #[test_case(Opcode::LDC,  false; "ldc")]
    #[test_case(Opcode::AND,  true; "and")]
    #[test_case(Opcode::ANDC, false; "andc")]
    #[test_case(Opcode::OR, true; "or")]
    #[test_case(Opcode::ORC, true; "orc")]
    #[test_case(Opcode::XNOR, true; "xnor")]
    fn r1_d1(opcode: Opcode, expected: bool) {
        let lu = LU {
            result_reg: true,
            data: true,
        };

        assert_eq!(lu.operation(opcode), expected);
    }
}
