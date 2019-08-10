// Regarding BCD math - this ALU emulates behavior of 65C02 and 65816 CPU,
// which corrected N and Z flags for BCD. 6502 calculates them for binary
// math despite the fact, that the D flag is set.
// More info: http://www.6502.org/tutorials/decimal_mode.html#A

#[derive(Debug)]
pub struct AluResult {
    pub value: u8,
    pub negative: bool,
    pub overflow: bool,
    pub zero: bool,
    pub carry: bool
}

pub fn add(base: u8, operand: u8, carry_in: bool, decimal: bool) -> AluResult {
    let (result, carry) = calculate_addition_result_in_proper_math_mode(base, operand, carry_in, decimal);
    let overflow = calculate_overflow_bit('+', base, operand, carry_in, decimal);
    let (negative, zero) = calculate_nz_bits(result);

    AluResult {
        value: result,
        negative: negative,
        overflow: overflow,
        zero: zero,
        carry: carry
    }
}

pub fn and(base: u8, operand: u8) -> AluResult {
    let result = base & operand;
    let (negative, zero) = calculate_nz_bits(result);

    AluResult {
        value: result,
        negative: negative,
        overflow: false,
        zero: zero,
        carry: false
    }
}

pub fn decrement(operand: u8) -> AluResult {
    let result = operand.overflowing_sub(1).0;
    let (negative, zero) = calculate_nz_bits(result);

    AluResult {
        value: result,
        negative: negative,
        overflow: false,
        zero: zero,
        carry: false
    }
}

pub fn increment(operand: u8) -> AluResult {
    let result = operand.overflowing_add(1).0;
    let (negative, zero) = calculate_nz_bits(result);

    AluResult {
        value: result,
        negative: negative,
        overflow: false,
        zero: zero,
        carry: false
    }
}

pub fn or(base: u8, operand: u8) -> AluResult {
    let result = base | operand;
    let (negative, zero) = calculate_nz_bits(result);

    AluResult {
        value: result,
        negative: negative,
        overflow: false,
        zero: zero,
        carry: false
    }
}

pub fn shift_left(operand: u8) -> AluResult {
    let result = operand << 1;
    let carry = operand > 127;
    let (negative, zero) = calculate_nz_bits(result);

    AluResult {
        value: result,
        negative: negative,
        overflow: false,
        zero: zero,
        carry: carry
    }
}

pub fn shift_right(operand: u8) -> AluResult {
    let result = operand >> 1;
    let carry = operand & 1 == 1;
    let (negative, zero) = calculate_nz_bits(result);

    AluResult {
        value: result,
        negative: negative,
        overflow: false,
        zero: zero,
        carry: carry
    }
}

pub fn subtract(base: u8, operand: u8, carry_in: bool, decimal: bool) -> AluResult {
    let (result, carry) = calculate_subtraction_result_in_proper_math_mode(base, operand, carry_in, decimal);
    let overflow = calculate_overflow_bit('-', base, operand, carry_in, decimal);
    let (negative, zero) = calculate_nz_bits(result);

    AluResult {
        value: result,
        negative: negative,
        overflow: overflow,
        zero: zero,
        carry: carry
    }
}

pub fn xor(base: u8, operand: u8) -> AluResult {
    let result = base ^ operand;
    let (negative, zero) = calculate_nz_bits(result);

    AluResult {
        value: result,
        negative: negative,
        overflow: false,
        zero: zero,
        carry: false
    }
}

fn calculate_nz_bits(operand: u8) -> (bool, bool) {
    // negative, zero
    (operand > 127, operand == 0)
}

// Based on <http://www.6502.org/tutorials/vflag.html>
fn calculate_overflow_bit(operation: char, base: u8, operand: u8, carry: bool, decimal: bool) -> bool {
    if !decimal || operation == '-' {
        return bin_overflow(operation, base, operand, carry);
    }

    return bcd_overflow(base, operand, carry);
}

fn calculate_addition_result_in_proper_math_mode(base: u8, operand: u8, carry: bool, decimal: bool) -> (u8, bool) {
    if !decimal {
        return bin_add(base, operand, carry);
    }

    return bcd_add(base, operand, carry);
}

fn calculate_subtraction_result_in_proper_math_mode(base: u8, operand: u8, carry: bool, decimal: bool) -> (u8, bool) {
    if !decimal {
        return bin_subtract(base, operand, carry);
    }

    return bcd_subtract(base, operand, carry);
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

fn bin_subtract(a: u8, b: u8, initial_carry: bool) -> (u8, bool) {
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

fn bcd_subtract(a: u8, b: u8, initial_carry: bool) -> (u8, bool) {
    let t1 = bcd_tencomp(b);
    let result = bcd_add(a, t1, false).0;
    let computed_carry = a >= b;

    if !initial_carry {
        let out = bcd_subtract(result, 0x01, true);
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
    use super::increment;
    use super::or;
    use super::shift_left;
    use super::shift_right;
    use super::subtract;
    use super::xor;

    #[test]
    fn test_binary_sum() {
        let result = add(2, 3, false, false);

        assert_eq!(result.value, 5);
        assert_eq!(result.negative, false);
        assert_eq!(result.overflow, false);
        assert_eq!(result.zero, false);
        assert_eq!(result.carry, false);
    }

    #[test]
    fn test_binary_sum_with_carry() {
        let result = add(250, 250, false, false);

        assert_eq!(result.value, 244);
        assert_eq!(result.negative, true);
        assert_eq!(result.overflow, false);
        assert_eq!(result.zero, false);
        assert_eq!(result.carry, true);
    }

    #[test]
    fn test_binary_sum_with_carry_set() {
        let result = add(10, 31, true, false);

        assert_eq!(result.value, 42);
        assert_eq!(result.negative, false);
        assert_eq!(result.overflow, false);
        assert_eq!(result.zero, false);
        assert_eq!(result.carry, false);
    }

    #[test]
    fn test_binary_sum_with_carry_set_and_basic_sum_with_carry() {
        let result = add(100, 200, true, false);

        assert_eq!(result.value, 45);
        assert_eq!(result.negative, false);
        assert_eq!(result.overflow, false);
        assert_eq!(result.zero, false);
        assert_eq!(result.carry, true);
    }

    #[test]
    fn test_binary_sum_with_carry_set_and_basic_sum_with_overflow() {
        let result = add(100, 27, true, false);

        assert_eq!(result.value, 128);
        assert_eq!(result.negative, true);
        assert_eq!(result.overflow, true);
        assert_eq!(result.zero, false);
        assert_eq!(result.carry, false);
    }

    #[test]
    fn test_binary_sum_with_carry_set_and_basic_sum_255() {
        let result = add(127, 128, true, false);

        assert_eq!(result.value, 0);
        assert_eq!(result.negative, false);
        assert_eq!(result.overflow, false);
        assert_eq!(result.zero, true);
        assert_eq!(result.carry, true);
    }

    #[test]
    fn test_binary_sum_with_zero() {
        let result = add(64, 192, false, false);

        assert_eq!(result.value, 0);
        assert_eq!(result.negative, false);
        assert_eq!(result.overflow, false);
        assert_eq!(result.zero, true);
        assert_eq!(result.carry, true);
    }

    #[test]
    fn test_binary_sum_with_overflow() {
        let result = add(128, 255, false, false);

        assert_eq!(result.value, 127);
        assert_eq!(result.negative, false);
        assert_eq!(result.overflow, true);
        assert_eq!(result.zero, false);
        assert_eq!(result.carry, true);
    }

    #[test]
    fn test_bcd_sum() {
        let result = add(0b0001_0101, 0b0010_0111, false, true); // 15 and 27 in BCD

        assert_eq!(result.value, 0b0100_0010); // 42 in BCD
        assert_eq!(result.negative, false);
        assert_eq!(result.overflow, false);
        assert_eq!(result.zero, false);
        assert_eq!(result.carry, false);
    }

    #[test]
    fn test_bcd_sum_with_carry_bit_set() {
        let result = add(0b0001_0101, 0b0010_0111, true, true); // 15 and 27 in BCD

        assert_eq!(result.value, 0b0100_0011); // 43 in BCD
        assert_eq!(result.negative, false);
        assert_eq!(result.overflow, false);
        assert_eq!(result.zero, false);
        assert_eq!(result.carry, false);
    }

    #[test]
    fn test_bcd_sum_with_carry() {
        let result = add(0b0001_0101, 0b1000_0111, false, true); // 15 and 87 in BCD

        assert_eq!(result.value, 0b0000_0010); // 2 in BCD
        assert_eq!(result.negative, false);
        assert_eq!(result.overflow, false);
        assert_eq!(result.zero, false);
        assert_eq!(result.carry, true);
    }

    #[test]
    fn test_bcd_sum_with_zero() {
        let result = add(0b0001_0101, 0b1000_0101, false, true); // 15 and 85 in BCD

        assert_eq!(result.value, 0b0000_0000); // 2 in BCD
        assert_eq!(result.negative, false);
        assert_eq!(result.overflow, false);
        assert_eq!(result.zero, true);
        assert_eq!(result.carry, true);
    }

    // Following tests are for V bit in BCD mode, taken from:
    // <http://www.6502.org/tutorials/vflag.html#b>
    #[test]
    fn test_bcd_sum_with_overflow_1() {
        let result = add(0b0010_0100, 0b0101_0110, false, true); // 24 and 56 in BCD

        assert_eq!(result.value, 0b1000_0000); // 80 in BCD
        assert_eq!(result.negative, true);
        assert_eq!(result.overflow, true);
        assert_eq!(result.zero, false);
        assert_eq!(result.carry, false);
    }

    #[test]
    fn test_bcd_sum_with_overflow_2() {
        let result = add(0b1001_0011, 0b1000_0010, false, true); // 93 and 82 in BCD

        assert_eq!(result.value, 0b0111_0101); // 75 in BCD
        assert_eq!(result.negative, false);
        assert_eq!(result.overflow, true);
        assert_eq!(result.zero, false);
        assert_eq!(result.carry, true);
    }

    #[test]
    fn test_bcd_sum_with_overflow_3() {
        let result = add(0b1000_1001, 0b0111_0110, false, true); // 89 and 76 in BCD

        assert_eq!(result.value, 0b0110_0101); // 65 in BCD
        assert_eq!(result.negative, false);
        assert_eq!(result.overflow, false);
        assert_eq!(result.zero, false);
        assert_eq!(result.carry, true);
    }

    #[test]
    fn test_bcd_sum_with_overflow_4() {
        let result = add(0b1000_0000, 0b1111_0000, false, true); // 80 and invalid number in BCD

        assert_eq!(result.value, 0b1101_0000); // invalid result in BCD
        assert_eq!(result.negative, true);
        assert_eq!(result.overflow, true);
        assert_eq!(result.zero, false);
        assert_eq!(result.carry, true);
    }

    #[test]
    fn test_bcd_sum_with_overflow_5() {
        let result = add(0b1000_0000, 0b1111_1010, false, true); // 80 and invalid number in BCD

        assert_eq!(result.value, 0b1110_0000); // invalid result in BCD
        assert_eq!(result.negative, true);
        assert_eq!(result.overflow, false);
        assert_eq!(result.zero, false);
        assert_eq!(result.carry, true);
    }

    #[test]
    fn test_bcd_sum_with_overflow_6() {
        let result = add(0b0010_1111, 0b0100_1111, false, true); // two invalid numbers in BCD

        assert_eq!(result.value, 0b0111_0100); // invalid result in BCD
        assert_eq!(result.negative, false);
        assert_eq!(result.overflow, false);
        assert_eq!(result.zero, false);
        assert_eq!(result.carry, false);
    }

    #[test]
    fn test_bcd_sum_with_overflow_7() {
        let result = add(0b0010_1111, 0b0010_1111, false, true); // two invalid numbers in BCD

        assert_eq!(result.value, 0b0101_0100); // invalid result in BCD
        assert_eq!(result.negative, false);
        assert_eq!(result.overflow, false);
        assert_eq!(result.zero, false);
        assert_eq!(result.carry, false);
    }

    #[test]
    fn test_binary_and() {
        let result = and(0b0110_0111, 0b1010_1010);

        assert_eq!(result.value, 0b0010_0010);
        assert_eq!(result.negative, false);
        assert_eq!(result.overflow, false);
        assert_eq!(result.zero, false);
        assert_eq!(result.carry, false);
    }

    #[test]
    fn test_binary_and_with_zero() {
        let result = and(0b1111_0000, 0b0000_1111);

        assert_eq!(result.value, 0b0000_0000);
        assert_eq!(result.negative, false);
        assert_eq!(result.overflow, false);
        assert_eq!(result.zero, true);
        assert_eq!(result.carry, false);
    }

    #[test]
    fn test_binary_and_with_negative_result() {
        let result = and(0b1001_0101, 0b1010_1010);

        assert_eq!(result.value, 0b1000_0000);
        assert_eq!(result.negative, true);
        assert_eq!(result.overflow, false);
        assert_eq!(result.zero, false);
        assert_eq!(result.carry, false);
    }

    #[test]
    fn test_increment() {
        let result = increment(0x41);

        assert_eq!(result.value, 0x42);
        assert_eq!(result.negative, false);
        assert_eq!(result.overflow, false);
        assert_eq!(result.zero, false);
        assert_eq!(result.carry, false);
    }

    #[test]
    fn test_increment_with_zero() {
        let result = increment(0xFF);

        assert_eq!(result.value, 0x00);
        assert_eq!(result.negative, false);
        assert_eq!(result.overflow, false);
        assert_eq!(result.zero, true);
        assert_eq!(result.carry, false);
    }

    #[test]
    fn test_increment_with_negative_bit() {
        let result = increment(0x81);

        assert_eq!(result.value, 0x82);
        assert_eq!(result.negative, true);
        assert_eq!(result.overflow, false);
        assert_eq!(result.zero, false);
        assert_eq!(result.carry, false);
    }

    #[test]
    fn test_decrement() {
        let result = decrement(0x43);

        assert_eq!(result.value, 0x42);
        assert_eq!(result.negative, false);
        assert_eq!(result.overflow, false);
        assert_eq!(result.zero, false);
        assert_eq!(result.carry, false);
    }

    #[test]
    fn test_decrement_with_zero() {
        let result = decrement(0x01);

        assert_eq!(result.value, 0x00);
        assert_eq!(result.negative, false);
        assert_eq!(result.overflow, false);
        assert_eq!(result.zero, true);
        assert_eq!(result.carry, false);
    }

    #[test]
    fn test_decrement_with_negative_bit() {
        let result = decrement(0x83);

        assert_eq!(result.value, 0x82);
        assert_eq!(result.negative, true);
        assert_eq!(result.overflow, false);
        assert_eq!(result.zero, false);
        assert_eq!(result.carry, false);
    }

    #[test]
    fn test_decrement_with_overflow() {
        let result = decrement(0x00);

        assert_eq!(result.value, 0xff);
        assert_eq!(result.negative, true);
        assert_eq!(result.overflow, false);
        assert_eq!(result.zero, false);
        assert_eq!(result.carry, false);
    }

    #[test]
    fn test_binary_or() {
        let result = or(0b0110_0111, 0b0010_1010);

        assert_eq!(result.value, 0b0110_1111);
        assert_eq!(result.negative, false);
        assert_eq!(result.overflow, false);
        assert_eq!(result.zero, false);
        assert_eq!(result.carry, false);
    }

    #[test]
    fn test_binary_or_with_zero() {
        let result = or(0b0000_0000, 0b0000_0000);

        assert_eq!(result.value, 0b0000_0000);
        assert_eq!(result.negative, false);
        assert_eq!(result.overflow, false);
        assert_eq!(result.zero, true);
        assert_eq!(result.carry, false);
    }

    #[test]
    fn test_binary_or_with_negative_result() {
        let result = or(0b0001_0101, 0b1010_1010);

        assert_eq!(result.value, 0b1011_1111);
        assert_eq!(result.negative, true);
        assert_eq!(result.overflow, false);
        assert_eq!(result.zero, false);
        assert_eq!(result.carry, false);
    }

    #[test]
    fn test_shift_left() {
        let result = shift_left(0b0010_1100);

        assert_eq!(result.value, 0b0101_1000);
        assert_eq!(result.negative, false);
        assert_eq!(result.overflow, false);
        assert_eq!(result.zero, false);
        assert_eq!(result.carry, false);
    }

    #[test]
    fn test_shift_left_with_carry() {
        let result = shift_left(0b1010_1100);

        assert_eq!(result.value, 0b0101_1000);
        assert_eq!(result.negative, false);
        assert_eq!(result.overflow, false);
        assert_eq!(result.zero, false);
        assert_eq!(result.carry, true);
    }

    #[test]
    fn test_shift_left_with_zero() {
        let result = shift_left(0b0000_0000);

        assert_eq!(result.value, 0b0000_0000);
        assert_eq!(result.negative, false);
        assert_eq!(result.overflow, false);
        assert_eq!(result.zero, true);
        assert_eq!(result.carry, false);
    }

    #[test]
    fn test_shift_left_with_negative_bit() {
        let result = shift_left(0b0110_1100);

        assert_eq!(result.value, 0b1101_1000);
        assert_eq!(result.negative, true);
        assert_eq!(result.overflow, false);
        assert_eq!(result.zero, false);
        assert_eq!(result.carry, false);
    }

    #[test]
    fn test_shift_right() {
        let result = shift_right(0b0010_1100);

        assert_eq!(result.value, 0b0001_0110);
        assert_eq!(result.negative, false);
        assert_eq!(result.overflow, false);
        assert_eq!(result.zero, false);
        assert_eq!(result.carry, false);
    }

    #[test]
    fn test_shift_right_with_carry() {
        let result = shift_right(0b1010_1101);

        assert_eq!(result.value, 0b0101_0110);
        assert_eq!(result.negative, false);
        assert_eq!(result.overflow, false);
        assert_eq!(result.zero, false);
        assert_eq!(result.carry, true);
    }

    #[test]
    fn test_shift_right_with_zero() {
        let result = shift_right(0b0000_0000);

        assert_eq!(result.value, 0b0000_0000);
        assert_eq!(result.negative, false);
        assert_eq!(result.overflow, false);
        assert_eq!(result.zero, true);
        assert_eq!(result.carry, false);
    }

    #[test]
    fn test_binary_subtraction() {
        let result = subtract(100, 31, true, false);

        assert_eq!(result.value, 69);
        assert_eq!(result.negative, false);
        assert_eq!(result.overflow, false);
        assert_eq!(result.zero, false);
        assert_eq!(result.carry, true);
    }

    #[test]
    fn test_binary_subtraction_with_negative_result() {
        let result = subtract(100, 120, true, false);

        assert_eq!(result.value, 236);
        assert_eq!(result.negative, true);
        assert_eq!(result.overflow, false);
        assert_eq!(result.zero, false);
        assert_eq!(result.carry, false);
    }

    #[test]
    fn test_binary_subtraction_without_carry() {
        let result = subtract(100, 31, false, false);

        assert_eq!(result.value, 68);
        assert_eq!(result.negative, false);
        assert_eq!(result.overflow, false);
        assert_eq!(result.zero, false);
        assert_eq!(result.carry, true);
    }

    #[test]
    fn test_binary_subtraction_with_negative_result_without_carry() {
        let result = subtract(100, 120, false, false);

        assert_eq!(result.value, 235);
        assert_eq!(result.negative, true);
        assert_eq!(result.overflow, false);
        assert_eq!(result.zero, false);
        assert_eq!(result.carry, false);
    }

    #[test]
    fn test_binary_subtraction_with_overflow() {
        let result = subtract(0, 1, true, false);

        assert_eq!(result.value, 255);
        assert_eq!(result.negative, true);
        assert_eq!(result.overflow, false);
        assert_eq!(result.zero, false);
        assert_eq!(result.carry, false);
    }

    #[test]
    fn test_binary_subtraction_with_overflow_2() {
        let result = subtract(128, 1, true, false);

        assert_eq!(result.value, 127);
        assert_eq!(result.negative, false);
        assert_eq!(result.overflow, true);
        assert_eq!(result.zero, false);
        assert_eq!(result.carry, true);
    }

    #[test]
    fn test_binary_subtraction_with_overflow_3() {
        let result = subtract(127, 255, true, false);

        assert_eq!(result.value, 128);
        assert_eq!(result.negative, true);
        assert_eq!(result.overflow, true);
        assert_eq!(result.zero, false);
        assert_eq!(result.carry, false);
    }

    #[test]
    fn test_binary_subtraction_with_overflow_4() {
        let result = subtract(192, 64, false, false);

        assert_eq!(result.value, 127);
        assert_eq!(result.negative, false);
        assert_eq!(result.overflow, true);
        assert_eq!(result.zero, false);
        assert_eq!(result.carry, true);
    }

    #[test]
    fn test_binary_subtraction_with_zero() {
        let result = subtract(50, 50, true, false);

        assert_eq!(result.value, 0);
        assert_eq!(result.negative, false);
        assert_eq!(result.overflow, false);
        assert_eq!(result.zero, true);
        assert_eq!(result.carry, true);
    }

    #[test]
    fn test_binary_subtraction_of_negatives_with_zero() {
        let result = subtract(150, 149, false, false);

        assert_eq!(result.value, 0);
        assert_eq!(result.negative, false);
        assert_eq!(result.overflow, false);
        assert_eq!(result.zero, true);
        assert_eq!(result.carry, true);
    }

    #[test]
    fn test_bcd_subtraction() {
        let result = subtract(0b0101_0000, 0b0001_0101, true, true); // 50 and 15 in BCD

        assert_eq!(result.value, 0b0011_0101); // 35 in BCD
        assert_eq!(result.negative, false);
        assert_eq!(result.overflow, false);
        assert_eq!(result.zero, false);
        assert_eq!(result.carry, true);
    }

    #[test]
    fn test_bcd_subtraction_with_negative_result() {
        let result = subtract(0b0001_0101, 0b0101_0000, true, true); // 15 and 50 in BCD

        assert_eq!(result.value, 0b0110_0101); // 65 in BCD (wraparound)
        assert_eq!(result.negative, false);
        assert_eq!(result.overflow, false);
        assert_eq!(result.zero, false);
        assert_eq!(result.carry, false);
    }

    #[test]
    fn test_bcd_subtraction_with_zero() {
        let result = subtract(0b0101_0000, 0b0101_0000, true, true); // 50 and 50 in BCD

        assert_eq!(result.value, 0b0000_0000); // 0 in BCD
        assert_eq!(result.negative, false);
        assert_eq!(result.overflow, false);
        assert_eq!(result.zero, true);
        assert_eq!(result.carry, true);
    }

    #[test]
    fn test_bcd_subtraction_with_negative_flag() {
        let result = subtract(0b1001_0101, 0b0000_0010, true, true); // 95 and 2 in BCD

        assert_eq!(result.value, 0b1001_0011); // 93 in BCD
        assert_eq!(result.negative, true);
        assert_eq!(result.overflow, false);
        assert_eq!(result.zero, false);
        assert_eq!(result.carry, true);
    }

    #[test]
    fn test_bcd_subtraction_with_overflow() {
        let result = subtract(0b1000_0000, 0b0000_0001, true, true); // 80 and 1 in BCD

        assert_eq!(result.value, 0b0111_1001); // 79 in BCD
        assert_eq!(result.negative, false);
        assert_eq!(result.overflow, true);
        assert_eq!(result.zero, false);
        assert_eq!(result.carry, true);
    }

    #[test]
    fn test_bcd_subtraction_without_carry() {
        let result = subtract(0b0101_0000, 0b0001_0101, false, true); // 50 and 15 in BCD

        assert_eq!(result.value, 0b0011_0100); // 34 in BCD
        assert_eq!(result.negative, false);
        assert_eq!(result.overflow, false);
        assert_eq!(result.zero, false);
        assert_eq!(result.carry, true);
    }

    #[test]
    fn test_bcd_subtraction_with_negative_result_without_carry() {
        let result = subtract(0b0001_0101, 0b0101_0000, false, true); // 15 and 50 in BCD

        assert_eq!(result.value, 0b0110_0100); // 64 in BCD (wraparound)
        assert_eq!(result.negative, false);
        assert_eq!(result.overflow, false);
        assert_eq!(result.zero, false);
        assert_eq!(result.carry, false);
    }

    #[test]
    fn test_bcd_subtraction_with_zero_without_carry() {
        let result = subtract(0b0101_0000, 0b0100_1001, false, true); // 50 and 49 in BCD

        assert_eq!(result.value, 0b0000_0000); // 0 in BCD
        assert_eq!(result.negative, false);
        assert_eq!(result.overflow, false);
        assert_eq!(result.zero, true);
        assert_eq!(result.carry, true);
    }

    #[test]
    fn test_binary_xor() {
        let result = xor(0b0110_0111, 0b0010_1010);

        assert_eq!(result.value, 0b0100_1101);
        assert_eq!(result.negative, false);
        assert_eq!(result.overflow, false);
        assert_eq!(result.zero, false);
        assert_eq!(result.carry, false);
    }

    #[test]
    fn test_binary_xor_with_zero() {
        let result = xor(0b0000_1111, 0b0000_1111);

        assert_eq!(result.value, 0b0000_0000);
        assert_eq!(result.negative, false);
        assert_eq!(result.overflow, false);
        assert_eq!(result.zero, true);
        assert_eq!(result.carry, false);
    }

    #[test]
    fn test_binary_xor_with_negative_result() {
        let result = xor(0b0001_0101, 0b1010_1010);

        assert_eq!(result.value, 0b1011_1111);
        assert_eq!(result.negative, true);
        assert_eq!(result.overflow, false);
        assert_eq!(result.zero, false);
        assert_eq!(result.carry, false);
    }
}
