use super::opcode::Opcode;

/// Represents the stateful 1-bit Logic Unit (LU) of the MC14500B.
///
/// The LU is responsible for all logical operations and holds key state,
/// including the result register (RR) and the Input/Output Enable flags.
#[derive(Debug, Default)]
pub struct LU {
    pub result_reg: bool,
    pub ien: bool,
    pub oen: bool,
}

impl LU {
    /// Executes a single instruction, modifying the LU's internal state.
    ///
    /// The `data` parameter represents the state of the external data bus.
    /// Logical operations are gated by the `ien` flag.
    pub fn execute(&mut self, code: Opcode, data: bool) {
        let logical_result = match code {
            Opcode::LD => data,
            Opcode::LDC => !data,
            Opcode::AND => self.result_reg & data,
            Opcode::ANDC => self.result_reg & !data,
            Opcode::OR => self.result_reg | data,
            Opcode::ORC => self.result_reg | !data,
            Opcode::XNOR => !(self.result_reg ^ data),
            // Not a logical opcode, handle below
            _ => self.result_reg,
        };

        // Hardware Gating: Only update the result register if Input Enable is active.
        if self.ien {
            self.result_reg = logical_result;
        }

        // Handle non-gated instructions
        match code {
            Opcode::IEN => self.ien = self.result_reg,
            Opcode::OEN => self.oen = self.result_reg,
            // Other non-LU opcodes are handled by the main CPU
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_logical_ops_when_gated_open() {
        let mut lu = LU::default();
        lu.ien = true; // Open the gate

        // Test LD
        lu.execute(Opcode::LD, true);
        assert_eq!(lu.result_reg, true);
        lu.execute(Opcode::LD, false);
        assert_eq!(lu.result_reg, false);

        // Test AND (RR is now false)
        lu.result_reg = true; // reset for test
        lu.execute(Opcode::AND, true);
        assert_eq!(lu.result_reg, true);
        lu.execute(Opcode::AND, false);
        assert_eq!(lu.result_reg, false);
    }

    #[test]
    fn test_gated_input_when_closed() {
        let mut lu = LU::default();
        lu.ien = false; // Close the gate
        lu.result_reg = false;

        // Try to load true, it should fail
        lu.execute(Opcode::LD, true);
        assert_eq!(
            lu.result_reg, false,
            "LD should not change RR when IEN is false"
        );

        // Try another operation
        lu.result_reg = true;
        lu.execute(Opcode::AND, false);
        assert_eq!(
            lu.result_reg, true,
            "AND should not change RR when IEN is false"
        );
    }

    #[test]
    fn test_gate_toggle_instructions() {
        let mut lu = LU::default();
        assert_eq!(lu.ien, false);
        assert_eq!(lu.oen, false);

        // Enable IEN
        lu.result_reg = true;
        lu.execute(Opcode::IEN, false);
        assert_eq!(lu.ien, true, "IEN should be set to true");

        // Disable IEN
        lu.result_reg = false;
        lu.execute(Opcode::IEN, true); // data bus shouldn't matter
        assert_eq!(lu.ien, false, "IEN should be set to false");

        // Enable OEN
        lu.result_reg = true;
        lu.execute(Opcode::OEN, false);
        assert_eq!(lu.oen, true, "OEN should be set to true");

        // Disable OEN
        lu.result_reg = false;
        lu.execute(Opcode::OEN, true);
        assert_eq!(lu.oen, false, "OEN should be set to false");
    }
}
