use std::str::FromStr;

/// Represents the 16 instructions of the Motorola MC14500B Industrial Control Unit (ICU).
///
/// Each variant corresponds to a 4-bit instruction code as specified in the datasheet.
/// This enum can be parsed from a string representation of the opcode (e.g., "LDC"),
/// which is case-insensitive.
#[derive(Debug, Clone, Copy, PartialEq)]
#[allow(clippy::upper_case_acronyms)]
pub enum Opcode {
    NOPO = 0b0000, // No change in registers
    LD = 0b0001,   // Load result register
    LDC = 0b0010,  // Load complement
    AND = 0b0011,  // Logical AND
    ANDC = 0b0100, // Logical AND complement
    OR = 0b0101,   // Logical OR
    ORC = 0b0110,  // Logical OR complement
    XNOR = 0b0111, // Exclusive NOR
    STO = 0b1000,  // Store
    STOC = 0b1001, // Store complement
    IEN = 0b1010,  // Input enable
    OEN = 0b1011,  // Output enable
    JMP = 0b1100,  // Set jump flag
    RTN = 0b1101,  // Set return flag
    SKZ = 0b1110,  // Skip next instruction if RR=0
    NOPF = 0b1111, // No change in registers
}

impl FromStr for Opcode {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "NOPO" => Ok(Opcode::NOPO),
            "LD" => Ok(Opcode::LD),
            "LDC" => Ok(Opcode::LDC),
            "AND" => Ok(Opcode::AND),
            "ANDC" => Ok(Opcode::ANDC),
            "OR" => Ok(Opcode::OR),
            "ORC" => Ok(Opcode::ORC),
            "XNOR" => Ok(Opcode::XNOR),
            "STO" => Ok(Opcode::STO),
            "STOC" => Ok(Opcode::STOC),
            "IEN" => Ok(Opcode::IEN),
            "OEN" => Ok(Opcode::OEN),
            "JMP" => Ok(Opcode::JMP),
            "RTN" => Ok(Opcode::RTN),
            "SKZ" => Ok(Opcode::SKZ),
            "NOPF" => Ok(Opcode::NOPF),
            _ => Err(format!("'{}' is not a valid opcode", s)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_opcode_from_str_ok() {
        assert_eq!(Opcode::from_str("LD").unwrap(), Opcode::LD);
        assert_eq!(Opcode::from_str("ldc").unwrap(), Opcode::LDC);
        assert_eq!(Opcode::from_str("XNOR").unwrap(), Opcode::XNOR);
    }

    #[test]
    fn test_opcode_from_str_err() {
        assert!(Opcode::from_str("INVALID").is_err());
        assert!(Opcode::from_str("").is_err());
    }
}
