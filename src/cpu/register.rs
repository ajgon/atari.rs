/*
Status register format (P):
+---+---+---+---+---+---+---+---+
| N | V | 1 | B | D | I | Z | C |
+---+---+---+---+---+---+---+---+
*/

const NEGATIVE_MASK: u8  = 0b1000_0000;
const OVERFLOW_MASK: u8  = 0b0100_0000;
const HARDWIRED_MASK: u8 = 0b0010_0000;
const BREAK_MASK: u8     = 0b0001_0000;
const DECIMAL_MASK: u8   = 0b0000_1000;
const INTERRUPT_MASK: u8 = 0b0000_0100;
const ZERO_MASK: u8      = 0b0000_0010;
const CARRY_MASK: u8     = 0b0000_0001;

#[derive(Debug)]
pub struct Register {
    pc: u16, // Program Counter
    s: u8, // Stack Pointer
    pub a: u8, // Accumulator
    pub x: u8, // Index Register X
    pub y: u8, // Index Register Y
    p: u8 // Status Register
}

impl Register {
    pub fn new() -> Register {
        return Register { pc: 0x0600, s: 0xFF, a: 0x00, x: 0x00, y: 0x00, p: 0b0010_0000 };
    }

    pub fn pc(&self) -> u16 {
        self.pc
    }

    pub fn increment_pc(&mut self) {
        self.pc = self.pc.overflowing_add(1).0;
    }

    pub fn set_pc(&mut self, value: u16) {
        self.pc = value;
    }

    pub fn s(&self) -> u8 {
        self.s
    }

    pub fn set_s(&mut self, value: u8) {
        self.s = value;
    }

    pub fn push_s(&mut self) {
        self.s = self.s.overflowing_sub(1).0;
    }

    pub fn pull_s(&mut self) {
        self.s = self.s.overflowing_add(1).0;
    }

    pub fn p(&self) -> u8 {
        self.p
    }

    pub fn set_p(&mut self, value: u8) {
        self.p = value | HARDWIRED_MASK;
    }

    pub fn negative_bit(&self) -> bool {
        self.p & NEGATIVE_MASK != 0
    }

    pub fn overflow_bit(&self) -> bool {
        self.p & OVERFLOW_MASK != 0
    }

    pub fn break_bit(&self) -> bool {
        self.p & BREAK_MASK != 0
    }

    pub fn decimal_bit(&self) -> bool {
        self.p & DECIMAL_MASK != 0
    }

    pub fn interrupt_bit(&self) -> bool {
        self.p & INTERRUPT_MASK != 0
    }

    pub fn zero_bit(&self) -> bool {
        self.p & ZERO_MASK != 0
    }

    pub fn carry_bit(&self) -> bool {
        self.p & CARRY_MASK != 0
    }

    pub fn set_negative_bit(&mut self, value: bool) {
        if value {
            self.p |= NEGATIVE_MASK
        } else {
            self.p &= !NEGATIVE_MASK
        }
    }

    pub fn set_overflow_bit(&mut self, value: bool) {
        if value {
            self.p |= OVERFLOW_MASK
        } else {
            self.p &= !OVERFLOW_MASK
        }
    }

    pub fn set_break_bit(&mut self, value: bool) {
        if value {
            self.p |= BREAK_MASK
        } else {
            self.p &= !BREAK_MASK
        }
    }

    pub fn set_decimal_bit(&mut self, value: bool) {
        if value {
            self.p |= DECIMAL_MASK
        } else {
            self.p &= !DECIMAL_MASK
        }
    }

    pub fn set_interrupt_bit(&mut self, value: bool) {
        if value {
            self.p |= INTERRUPT_MASK
        } else {
            self.p &= !INTERRUPT_MASK
        }
    }

    pub fn set_zero_bit(&mut self, value: bool) {
        if value {
            self.p |= ZERO_MASK
        } else {
            self.p &= !ZERO_MASK
        }
    }

    pub fn set_carry_bit(&mut self, value: bool) {
        if value {
            self.p |= CARRY_MASK
        } else {
            self.p &= !CARRY_MASK
        }
    }
}

