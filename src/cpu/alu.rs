use crate::cpu::register::Register;

// Regarding BCD math - this ALU emulates behavior of 65C02 and 65816 CPU,
// which corrected N and Z flags for BCD. 6502 calculates them for binary
// math despite the fact, that the D flag is set.
// More info: http://www.6502.org/tutorials/decimal_mode.html#A

pub fn add(base: u8, operand: u8, register: &mut Register) -> u8 {
    let (result, carry_bit) = calculate_addition_result_in_proper_math_mode(base, operand, register);

    // Order of these calls is important, don't change it
    calculate_overflow_bit('+', base, operand, register);
    register.set_carry_bit(carry_bit);
    register.set_zero_bit(result == 0);
    register.set_negative_bit(result > 127);

    return result;
}

pub fn and(base: u8, operand: u8, register: &mut Register) -> u8 {
    let result = base & operand;

    register.set_zero_bit(result == 0);
    register.set_negative_bit(result > 127);

    return result;
}

pub fn decrement(operand: u8, register: &mut Register) -> u8 {
    let result = operand.overflowing_sub(1).0;

    register.set_zero_bit(result == 0);
    register.set_negative_bit(result > 127);

    return result;
}

pub fn shift_left(operand: u8, register: &mut Register) -> u8 {
    let result = operand << 1;

    register.set_carry_bit(operand > 127);
    register.set_zero_bit(result == 0);
    register.set_negative_bit(result > 127);

    return result;
}

pub fn sub(base: u8, operand: u8, register: &mut Register) -> u8 {
    let (result, carry_bit) = calculate_subtraction_result_in_proper_math_mode(base, operand, register);

    calculate_overflow_bit('-', base, operand, register);
    register.set_carry_bit(carry_bit);
    register.set_zero_bit(result == 0);
    register.set_negative_bit(result > 127);

    return result;
}


// Based on <http://www.6502.org/tutorials/vflag.html>
fn calculate_overflow_bit(operation: char, base: u8, operand: u8, register: &mut Register) {
    if !register.decimal_bit() || operation == '-' {
        register.set_overflow_bit(bin_overflow(operation, base, operand, register.carry_bit()));
        return;
    };

    register.set_overflow_bit(bcd_overflow(base, operand, register.carry_bit()));
}

fn calculate_addition_result_in_proper_math_mode(base: u8, operand: u8, register: &mut Register) -> (u8, bool) {
    if !register.decimal_bit() {
        return bin_add(base, operand, register.carry_bit());
    }

    return bcd_add(base, operand, register.carry_bit());
}

fn calculate_subtraction_result_in_proper_math_mode(base: u8, operand: u8, register: &mut Register) -> (u8, bool) {
    if !register.decimal_bit() {
        return bin_sub(base, operand, register.carry_bit())
    }

    return bcd_sub(base, operand, register.carry_bit());
}

// BIN Math
fn bin_add(a: u8, b: u8, initial_carry: bool) -> (u8, bool) {
    let (result, computed_carry) = a.overflowing_add(b);

    if initial_carry {
        let out = result.overflowing_add(1);
        return (out.0, computed_carry | out.1);
    }

    return (result, computed_carry);
}

fn bin_sub(a: u8, b: u8, initial_carry: bool) -> (u8, bool) {
    let (result, _) = a.overflowing_sub(b);
    let computed_carry = a >= b;

    if !initial_carry {
        let out = result.overflowing_sub(1);
        return (out.0, computed_carry & (result >= 1));
    }

    return (result, computed_carry);
}

fn bin_overflow(operation: char, a: u8, b: u8, initial_carry: bool) -> bool {
    let carry_value = if initial_carry { 1u8 } else { 0u8 };
    let left_operand: i16 = (a as i8) as i16;
    let right_operand: i16 = (b as i8) as i16;

    let v_sum = if operation == '-' {
        left_operand - right_operand - (1i16 - carry_value as i16)
    } else {
        left_operand + right_operand + carry_value as i16
    };

    return v_sum < -128 || v_sum > 127;
}


// BCD Math
// https://homepage.cs.uiowa.edu/~jones/bcd/bcd.html
fn bcd_valid(a: u8) -> bool {
    let t1: u8 = a + 0x06;
    let t2: u8 = t1 ^ a;
    let t3: u8 = t2 & 0x10;
    return t3 == 0;
}

fn bcd_tencomp(a: u8) -> u8 {
    return bcd_add(0x99 - a, 0x01, false).0;
}

fn bcd_add(a: u8, b: u8, initial_carry: bool) -> (u8, bool) {
    let t1: u16 = a as u16 + 0x0666;
    let t2: u16 = t1 + b as u16;
    let t3: u16 = t1 ^ b as u16;
    let t4: u16 = t2 ^ t3;
    let carry_correction = if !bcd_valid(a) && !bcd_valid(b) && t4 != 0 { 0x10 } else { 0 };
    let t5: u16 = !t4 & 0x1110;
    let t6: u16 = (t5 >> 2) | (t5 >> 3);
    let t7: u16 = t2 - t6 - carry_correction;

    let result = (t7 & 0xff) as u8;
    let computed_carry = t7 & 0xff00 > 0;

    if initial_carry {
        let out = bcd_add(result, 0x01, false);
        return (out.0, computed_carry | out.1);
    }

    return (result, computed_carry);
}

fn bcd_sub(a: u8, b: u8, initial_carry: bool) -> (u8, bool) {
    let t1 = bcd_tencomp(b);
    let result = bcd_add(a, t1, false).0;
    let computed_carry = a >= b;

    if !initial_carry {
        let out = bcd_sub(result, 0x01, true);
        return (out.0, computed_carry & out.1);
    }

    return (result, computed_carry);
}

fn bcd_overflow(a: u8, b: u8, initial_carry: bool) -> bool {
    let carry_value = if initial_carry { 1u8 } else { 0u8 };
    let left_operand: i8 = ((a & 0b1111_0000) >> 4) as i8;
    let right_operand: i8 = ((b & 0b1111_0000) >> 4) as i8;
    let left_operand = if left_operand > 7 { left_operand - 16 } else { left_operand };
    let right_operand = if right_operand > 7 { right_operand - 16 } else { right_operand };
    let carry = if (a & 0b1111) + (b & 0b1111) + carry_value > 9 { 1i8 } else { 0i8 };

    let v_sum = left_operand + right_operand + carry;

    return v_sum < -8 || v_sum > 7;
}

#[cfg(test)]
mod tests {
    use super::add;
    use super::and;
    use super::decrement;
    use super::shift_left;
    use super::sub;

    use crate::cpu::register::Register;

    #[test]
    fn test_binary_sum() {
        let mut register = Register::new();
        let result = add(2, 3, &mut register);

        assert_eq!(result, 5);
        assert_eq!(register.p(), 0b0011_0000);
    }

    #[test]
    fn test_binary_sum_with_carry() {
        let mut register = Register::new();
        let result = add(250, 250, &mut register);

        assert_eq!(result, 244);
        assert_eq!(register.p(), 0b1011_0001);
    }

    #[test]
    fn test_binary_sum_with_carry_set() {
        let mut register = Register::new();
        register.set_carry_bit(true);
        let result = add(10, 31, &mut register);

        assert_eq!(result, 42);
        assert_eq!(register.p(), 0b0011_0000);
    }

    #[test]
    fn test_binary_sum_with_carry_set_and_basic_sum_with_carry() {
        let mut register = Register::new();
        register.set_carry_bit(true);
        let result = add(100, 200, &mut register);

        assert_eq!(result, 45);
        assert_eq!(register.p(), 0b0011_0001);
    }

    #[test]
    fn test_binary_sum_with_carry_set_and_basic_sum_with_overflow() {
        let mut register = Register::new();
        register.set_carry_bit(true);
        let result = add(100, 27, &mut register);

        assert_eq!(result, 128);
        assert_eq!(register.p(), 0b1111_0000);
    }

    #[test]
    fn test_binary_sum_with_carry_set_and_basic_sum_255() {
        let mut register = Register::new();
        register.set_carry_bit(true);
        let result = add(127, 128, &mut register);

        assert_eq!(result, 0);
        assert_eq!(register.p(), 0b0011_0011);
    }

    #[test]
    fn test_binary_sum_with_zero() {
        let mut register = Register::new();
        let result = add(64, 192, &mut register);

        assert_eq!(result, 0);
        assert_eq!(register.p(), 0b0011_0011);
    }

    #[test]
    fn test_binary_sum_with_overflow() {
        let mut register = Register::new();
        let result = add(128, 255, &mut register);

        assert_eq!(result, 127);
        assert_eq!(register.p(), 0b0111_0001);
    }

    #[test]
    fn test_bcd_sum() {
        let mut register = Register::new();
        register.set_decimal_bit(true);
        let result = add(0b0001_0101, 0b0010_0111, &mut register); // 15 and 27 in BCD

        assert_eq!(result, 0b0100_0010); // 42 in BCD
        assert_eq!(register.p(), 0b0011_1000);
    }

    #[test]
    fn test_bcd_sum_with_carry_bit_set() {
        let mut register = Register::new();
        register.set_decimal_bit(true);
        register.set_carry_bit(true);
        let result = add(0b0001_0101, 0b0010_0111, &mut register); // 15 and 27 in BCD

        assert_eq!(result, 0b0100_0011); // 43 in BCD
        assert_eq!(register.p(), 0b0011_1000);
    }

    #[test]
    fn test_bcd_sum_with_carry() {
        let mut register = Register::new();
        register.set_decimal_bit(true);
        let result = add(0b0001_0101, 0b1000_0111, &mut register); // 15 and 87 in BCD

        assert_eq!(result, 0b0000_0010); // 2 in BCD
        assert_eq!(register.p(), 0b0011_1001);
    }

    #[test]
    fn test_bcd_sum_with_zero() {
        let mut register = Register::new();
        register.set_decimal_bit(true);
        let result = add(0b0001_0101, 0b1000_0101, &mut register); // 15 and 85 in BCD

        assert_eq!(result, 0b0000_0000); // 2 in BCD
        assert_eq!(register.p(), 0b0011_1011);
    }

    // Following tests are for V bit in BCD mode, taken from:
    // <http://www.6502.org/tutorials/vflag.html#b>
    #[test]
    fn test_bcd_sum_with_overflow_1() {
        let mut register = Register::new();
        register.set_decimal_bit(true);
        let result = add(0b0010_0100, 0b0101_0110, &mut register); // 24 and 56 in BCD

        assert_eq!(result, 0b1000_0000); // 80 in BCD
        assert_eq!(register.p(), 0b1111_1000);
    }

    #[test]
    fn test_bcd_sum_with_overflow_2() {
        let mut register = Register::new();
        register.set_decimal_bit(true);
        let result = add(0b1001_0011, 0b1000_0010, &mut register); // 93 and 82 in BCD

        assert_eq!(result, 0b0111_0101); // 75 in BCD
        assert_eq!(register.p(), 0b0111_1001);
    }

    #[test]
    fn test_bcd_sum_with_overflow_3() {
        let mut register = Register::new();
        register.set_decimal_bit(true);
        let result = add(0b1000_1001, 0b0111_0110, &mut register); // 89 and 76 in BCD

        assert_eq!(result, 0b0110_0101); // 65 in BCD
        assert_eq!(register.p(), 0b0011_1001);
    }

    #[test]
    fn test_bcd_sum_with_overflow_4() {
        let mut register = Register::new();
        register.set_decimal_bit(true);
        let result = add(0b1000_0000, 0b1111_0000, &mut register); // 80 and invalid number in BCD

        assert_eq!(result, 0b1101_0000); // invalid result in BCD
        assert_eq!(register.p(), 0b1111_1001);
    }

    #[test]
    fn test_bcd_sum_with_overflow_5() {
        let mut register = Register::new();
        register.set_decimal_bit(true);
        let result = add(0b1000_0000, 0b1111_1010, &mut register); // 80 and invalid number in BCD

        assert_eq!(result, 0b1110_0000); // invalid result in BCD
        assert_eq!(register.p(), 0b1011_1001);
    }

    #[test]
    fn test_bcd_sum_with_overflow_6() {
        let mut register = Register::new();
        register.set_decimal_bit(true);
        let result = add(0b0010_1111, 0b0100_1111, &mut register); // two invalid numbers in BCD

        assert_eq!(result, 0b0111_0100); // invalid result in BCD
        assert_eq!(register.p(), 0b0011_1000);
    }

    #[test]
    fn test_bcd_sum_with_overflow_7() {
        let mut register = Register::new();
        register.set_decimal_bit(true);
        let result = add(0b0010_1111, 0b0010_1111, &mut register); // two invalid numbers in BCD

        assert_eq!(result, 0b0101_0100); // invalid result in BCD
        assert_eq!(register.p(), 0b0011_1000);
    }

    #[test]
    fn test_binary_and() {
        let mut register = Register::new();
        let result = and(0b0110_0111, 0b1010_1010, &mut register);

        assert_eq!(result, 0b0010_0010);
        assert_eq!(register.p(), 0b0011_0000);
    }

    #[test]
    fn test_binary_and_with_zero() {
        let mut register = Register::new();
        let result = and(0b1111_0000, 0b0000_1111, &mut register);

        assert_eq!(result, 0b0000_0000);
        assert_eq!(register.p(), 0b0011_0010);
    }

    #[test]
    fn test_binary_and_with_negative_result() {
        let mut register = Register::new();
        let result = and(0b1001_0101, 0b1010_1010, &mut register);

        assert_eq!(result, 0b1000_0000);
        assert_eq!(register.p(), 0b1011_0000);
    }

    #[test]
    fn test_decrement() {
        let mut register = Register::new();
        let result = decrement(0x43, &mut register);

        assert_eq!(result, 0x42);
        assert_eq!(register.p(), 0b0011_0000);
    }

    #[test]
    fn test_decrement_with_zero() {
        let mut register = Register::new();
        let result = decrement(0x01, &mut register);

        assert_eq!(result, 0x00);
        assert_eq!(register.p(), 0b0011_0010);
    }

    #[test]
    fn test_decrement_with_negative_bit() {
        let mut register = Register::new();
        let result = decrement(0x83, &mut register);

        assert_eq!(result, 0x82);
        assert_eq!(register.p(), 0b1011_0000);
    }

    #[test]
    fn test_decrement_with_overflow() {
        let mut register = Register::new();
        let result = decrement(0x00, &mut register);

        assert_eq!(result, 0xff);
        assert_eq!(register.p(), 0b1011_0000);
    }

    #[test]
    fn test_shift_left() {
        let mut register = Register::new();
        let result = shift_left(0b0010_1100, &mut register);

        assert_eq!(result, 0b0101_1000);
        assert_eq!(register.p(), 0b0011_0000);
    }

    #[test]
    fn test_shift_left_with_carry() {
        let mut register = Register::new();
        let result = shift_left(0b1010_1100, &mut register);

        assert_eq!(result, 0b0101_1000);
        assert_eq!(register.p(), 0b0011_0001);
    }

    #[test]
    fn test_shift_left_with_zero() {
        let mut register = Register::new();
        let result = shift_left(0b0000_0000, &mut register);

        assert_eq!(result, 0b0000_0000);
        assert_eq!(register.p(), 0b0011_0010);
    }

    #[test]
    fn test_shift_left_with_negative_bit() {
        let mut register = Register::new();
        let result = shift_left(0b0110_1100, &mut register);

        assert_eq!(result, 0b1101_1000);
        assert_eq!(register.p(), 0b1011_0000);
    }

    #[test]
    fn test_binary_subtraction() {
        let mut register = Register::new();
        register.set_carry_bit(true);
        let result = sub(100, 31, &mut register);

        assert_eq!(result, 69);
        assert_eq!(register.p(), 0b0011_0001);
    }

    #[test]
    fn test_binary_subtraction_with_negative_result() {
        let mut register = Register::new();
        register.set_carry_bit(true);
        let result = sub(100, 120, &mut register);

        assert_eq!(result, 236);
        assert_eq!(register.p(), 0b1011_0000);
    }

    #[test]
    fn test_binary_subtraction_without_carry() {
        let mut register = Register::new();
        let result = sub(100, 31, &mut register);

        assert_eq!(result, 68);
        assert_eq!(register.p(), 0b0011_0001);
    }

    #[test]
    fn test_binary_subtraction_with_negative_result_without_carry() {
        let mut register = Register::new();
        let result = sub(100, 120, &mut register);

        assert_eq!(result, 235);
        assert_eq!(register.p(), 0b1011_0000);
    }

    #[test]
    fn test_binary_subtraction_with_overflow() {
        let mut register = Register::new();
        register.set_carry_bit(true);
        let result = sub(0, 1, &mut register);

        assert_eq!(result, 255);
        assert_eq!(register.p(), 0b1011_0000);
    }

    #[test]
    fn test_binary_subtraction_with_overflow_2() {
        let mut register = Register::new();
        register.set_carry_bit(true);
        let result = sub(128, 1, &mut register);

        assert_eq!(result, 127);
        assert_eq!(register.p(), 0b0111_0001);
    }

    #[test]
    fn test_binary_subtraction_with_overflow_3() {
        let mut register = Register::new();
        register.set_carry_bit(true);
        let result = sub(127, 255, &mut register);

        assert_eq!(result, 128);
        assert_eq!(register.p(), 0b1111_0000);
    }

    #[test]
    fn test_binary_subtraction_with_overflow_4() {
        let mut register = Register::new();
        let result = sub(192, 64, &mut register);

        assert_eq!(result, 127);
        assert_eq!(register.p(), 0b0111_0001);
    }

    #[test]
    fn test_binary_subtraction_with_zero() {
        let mut register = Register::new();
        register.set_carry_bit(true);
        let result = sub(50, 50, &mut register);

        assert_eq!(result, 0);
        assert_eq!(register.p(), 0b0011_0011);
    }

    #[test]
    fn test_binary_subtraction_of_negatives_with_zero() {
        let mut register = Register::new();
        let result = sub(150, 149, &mut register);

        assert_eq!(result, 0);
        assert_eq!(register.p(), 0b0011_0011);
    }

    #[test]
    fn test_bcd_subtraction() {
        let mut register = Register::new();
        register.set_decimal_bit(true);
        register.set_carry_bit(true);
        let result = sub(0b0101_0000, 0b0001_0101, &mut register); // 50 and 15 in BCD

        assert_eq!(result, 0b0011_0101); // 35 in BCD
        assert_eq!(register.p(), 0b0011_1001);
    }

    #[test]
    fn test_bcd_subtraction_with_negative_result() {
        let mut register = Register::new();
        register.set_decimal_bit(true);
        register.set_carry_bit(true);
        let result = sub(0b0001_0101, 0b0101_0000, &mut register); // 15 and 50 in BCD

        assert_eq!(result, 0b0110_0101); // 65 in BCD (wraparound)
        assert_eq!(register.p(), 0b0011_1000);
    }

    #[test]
    fn test_bcd_subtraction_with_zero() {
        let mut register = Register::new();
        register.set_decimal_bit(true);
        register.set_carry_bit(true);
        let result = sub(0b0101_0000, 0b0101_0000, &mut register); // 50 and 50 in BCD

        assert_eq!(result, 0b0000_0000); // 0 in BCD
        assert_eq!(register.p(), 0b0011_1011);
    }

    #[test]
    fn test_bcd_subtraction_with_negative_flag() {
        let mut register = Register::new();
        register.set_decimal_bit(true);
        register.set_carry_bit(true);
        let result = sub(0b1001_0101, 0b0000_0010, &mut register); // 95 and 2 in BCD

        assert_eq!(result, 0b1001_0011); // 93 in BCD
        assert_eq!(register.p(), 0b1011_1001);
    }

    #[test]
    fn test_bcd_subtraction_with_overflow() {
        let mut register = Register::new();
        register.set_decimal_bit(true);
        register.set_carry_bit(true);
        let result = sub(0b1000_0000, 0b0000_0001, &mut register); // 80 and 1 in BCD

        assert_eq!(result, 0b0111_1001); // 79 in BCD
        assert_eq!(register.p(), 0b0111_1001);
    }

    #[test]
    fn test_bcd_subtraction_without_carry() {
        let mut register = Register::new();
        register.set_decimal_bit(true);
        let result = sub(0b0101_0000, 0b0001_0101, &mut register); // 50 and 15 in BCD

        assert_eq!(result, 0b0011_0100); // 34 in BCD
        assert_eq!(register.p(), 0b0011_1001);
    }

    #[test]
    fn test_bcd_subtraction_with_negative_result_without_carry() {
        let mut register = Register::new();
        register.set_decimal_bit(true);
        let result = sub(0b0001_0101, 0b0101_0000, &mut register); // 15 and 50 in BCD

        assert_eq!(result, 0b0110_0100); // 64 in BCD (wraparound)
        assert_eq!(register.p(), 0b0011_1000);
    }

    #[test]
    fn test_bcd_subtraction_with_zero_without_carry() {
        let mut register = Register::new();
        register.set_decimal_bit(true);
        let result = sub(0b0101_0000, 0b0100_1001, &mut register); // 50 and 49 in BCD

        assert_eq!(result, 0b0000_0000); // 0 in BCD
        assert_eq!(register.p(), 0b0011_1011);
    }
}
