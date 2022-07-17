#[derive(Debug)]
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
