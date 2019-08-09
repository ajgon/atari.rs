/*
Status register format (P):
+---+---+---+---+---+---+---+---+
| N | V | 1 | B | D | I | Z | C |
+---+---+---+---+---+---+---+---+
*/

#[derive(Debug)]
pub struct Register {
    pc: u16, // Program Counter
    s: u8, // Stack Pointer
    a: u8, // Accumulator
    x: u8, // Index Register X
    y: u8, // Index Register Y
    p: u8 // Status Register
}

impl Register {
    pub fn new() -> Register {
        return Register { pc: 0x0600, s: 0xFF, a: 0x00, x: 0x00, y: 0x00, p: 0b0010_0000 };
    }

    pub fn pc(&self) -> u16 {
        return self.pc;
    }

    pub fn increment_pc(&mut self) {
        self.pc = self.pc.overflowing_add(1).0;
    }

    pub fn increment_pc_by(&mut self, amount: u16) {
        self.pc = self.pc.overflowing_add(amount).0;
    }

    pub fn set_pc(&mut self, value: u16) {
        self.pc = value;
    }

    pub fn s(&self) -> u8 {
        return self.s;
    }

    pub fn push_s(&mut self) {
        self.s = self.s.overflowing_sub(1).0;
    }

    pub fn pull_s(&mut self) {
        self.s = self.s.overflowing_add(1).0;
    }

    pub fn set_s(&mut self, value: u8) {
        self.s = value;
    }

    pub fn a(&self) -> u8 {
        return self.a;
    }

    pub fn set_accumulator(&mut self, value: u8) {
        self.a = value;
    }

    pub fn x(&self) -> u8 {
        return self.x;
    }

    pub fn y(&self) -> u8 {
        return self.y;
    }

    pub fn p(&self) -> u8 {
        return self.p;
    }

    pub fn carry_bit(&self) -> bool {
        return self.p & 0b00000001 != 0;
    }

    pub fn zero_bit(&self) -> bool {
        return self.p & 0b00000010 != 0;
    }

    pub fn overflow_bit(&self) -> bool {
        return self.p & 0b01000000 != 0;
    }

    pub fn decimal_bit(&self) -> bool {
        return self.p & 0b00001000 != 0;
    }

    pub fn negative_bit(&self) -> bool {
        return self.p & 0b10000000 != 0;
    }

    pub fn set_negative_bit(&mut self, value: bool) {
        if value {
            self.p |= 0b10000000;
        } else {
            self.p &= 0b01111111;
        }
    }

    pub fn set_overflow_bit(&mut self, value: bool) {
        if value {
            self.p |= 0b01000000;
        } else {
            self.p &= 0b10111111;
        }
    }

    pub fn set_decimal_bit(&mut self, value: bool) {
        if value {
            self.p |= 0b00001000;
        } else {
            self.p &= 0b11110111;
        }
    }

    pub fn set_break_bit(&mut self, value: bool) {
        if value {
            self.p |= 0b00010000;
        } else {
            self.p &= 0b11101111;
        }
    }

    pub fn set_interrupt_bit(&mut self, value: bool) {
        if value {
            self.p |= 0b00000100;
        } else {
            self.p &= 0b11111011;
        }
    }

    pub fn set_zero_bit(&mut self, value: bool) {
        if value {
            self.p |= 0b00000010;
        } else {
            self.p &= 0b11111101;
        }
    }

    pub fn set_carry_bit(&mut self, value: bool) {
        if value {
            self.p |= 0b00000001;
        } else {
            self.p &= 0b11111110;
        }
    }

    pub fn set_p(&mut self, value: u8) {
        self.p = value | 0b0010_0000;
    }

    pub fn set_x(&mut self, value: u8) {
        self.x = value;
    }

    pub fn set_y(&mut self, value: u8) {
        self.y = value;
    }

    pub fn calculate_nz_bits(&mut self, base: u8) {
        self.set_zero_bit(base == 0);
        self.set_negative_bit(base > 127);
    }
}

#[cfg(test)]
mod tests {
    use super::Register;

    #[test]
    fn test_getters() {
        let register = Register::new();
        assert_eq!(register.pc(), 0x0600);
        assert_eq!(register.s(), 0xFF);
        assert_eq!(register.a(), 0x00);
        assert_eq!(register.x(), 0x00);
        assert_eq!(register.y(), 0x00);
        assert_eq!(register.p(), 0x20);
    }

    #[test]
    fn test_pc() {
        let mut register = Register::new();

        register.increment_pc();
        assert_eq!(register.pc(), 0x0601);

        register.increment_pc_by(14);
        assert_eq!(register.pc(), 0x060F);
    }

    #[test]
    fn test_a() {
        let mut register = Register::new();

        register.set_accumulator(0x15);
        assert_eq!(register.a(), 0x15);
    }
}

