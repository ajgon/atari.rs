use crate::cpu::register::Register;

pub fn calculate(operation: char, operand: u8, register: &mut Register) {
    let (result, carry_bit) = calculate_result_in_proper_math_mode(operand, register);

    calculate_overflow_bit(operand, register);
    register.set_accumulator(result);
    register.set_carry_bit(carry_bit);
    register.set_zero_bit(result == 0);
}

fn calculate_overflow_bit(operand: u8, register: &mut Register) {
    if !register.bcd_bit() {
        let left_operand: i16 = (register.a() as i8) as i16;
        let right_operand: i16 = (operand as i8) as i16;
        let v_sum = left_operand + right_operand;
        println!("{}, {}", left_operand, right_operand);
        register.set_overflow_bit(v_sum < -128 || v_sum > 127);
        return;
    };

    let left_operand: i8 = ((register.a() & 0b1111_0000) >> 4) as i8;
    let right_operand: i8 = ((operand & 0b1111_0000) >> 4) as i8;
    let left_operand = if left_operand > 7 { left_operand - 16 } else { left_operand };
    let right_operand = if right_operand > 7 { right_operand - 16 } else { right_operand };
    let carry = if (register.a() & 0b1111) + (operand & 0b1111) > 9 { 1 } else { 0 };
    let v_sum = left_operand + right_operand + carry;
    println!("{}, {}, {}", left_operand, right_operand, carry);
    register.set_overflow_bit(v_sum < -8 || v_sum > 7);
}

fn calculate_result_in_proper_math_mode(operand: u8, register: &mut Register) -> (u8, bool) {
    if !register.bcd_bit() {
        return register.a().overflowing_add(operand);
    }

    let units_result = (register.a() & 0b0000_1111) + (operand & 0b0000_1111);
    let (units_result, units_carry_value) = if units_result > 9 {
        (units_result - 10, 1)
    } else {
        (units_result, 0)
    };
    let dozens_result = ((register.a() & 0b1111_0000) >> 4) + ((operand & 0b1111_0000) >> 4) + units_carry_value;
    let (dozens_result, carry_bit) = if dozens_result > 9 {
        (dozens_result - 10, true)
    } else {
        (dozens_result, false)
    };
    let result = (dozens_result << 4) + units_result;
    return (result, carry_bit)
}

#[cfg(test)]
mod tests {
    use super::calculate;

    use crate::cpu::register::Register;

    #[test]
    fn test_binary_sum() {
        let mut register = Register::new();
        register.set_accumulator(2);
        calculate('+', 3, &mut register);

        assert_eq!(register.a(), 5);
        assert_eq!(register.p(), 0b0010_0000);
    }

    #[test]
    fn test_binary_sum_with_carry() {
        let mut register = Register::new();
        register.set_accumulator(250);
        calculate('+', 250, &mut register);

        assert_eq!(register.a(), 244);
        assert_eq!(register.p(), 0b1010_0001);
    }

    #[test]
    fn test_binary_sum_with_zero() {
        let mut register = Register::new();
        register.set_accumulator(64);
        calculate('+', 192, &mut register);

        assert_eq!(register.a(), 0);
        assert_eq!(register.p(), 0b0010_0011);
    }

    #[test]
    fn test_binary_sum_with_overflow() {
        let mut register = Register::new();
        register.set_accumulator(128);
        calculate('+', 255, &mut register);

        assert_eq!(register.a(), 0b0111_1111);
        assert_eq!(register.p(), 0b0110_0001);
    }

    #[test]
    fn test_bcd_sum() {
        let mut register = Register::new();
        register.set_accumulator(0b0001_0101); // 15 in BCD
        register.set_bcd_bit(true);
        calculate('+', 0b0010_0111, &mut register); // 27 in BCD

        assert_eq!(register.a(), 0b0100_0010); // 42 in BCD
        assert_eq!(register.p(), 0b0011_0000);
    }

    #[test]
    fn test_bcd_sum_with_carry() {
        let mut register = Register::new();
        register.set_accumulator(0b0001_0101); // 15 in BCD
        register.set_bcd_bit(true);
        calculate('+', 0b1000_0111, &mut register); // 87 in BCD

        assert_eq!(register.a(), 0b0000_0010); // 2 in BCD
        assert_eq!(register.p(), 0b0011_0001);
    }

    #[test]
    fn test_bcd_sum_with_zero() {
        let mut register = Register::new();
        register.set_accumulator(0b0001_0101); // 15 in BCD
        register.set_bcd_bit(true);
        calculate('+', 0b1000_0101, &mut register); // 85 in BCD

        assert_eq!(register.a(), 0b0000_0000); // 2 in BCD
        assert_eq!(register.p(), 0b0011_0011);
    }

    // Following tests are for V bit in BCD mode, taken from:
    // <http://www.6502.org/tutorials/vflag.html#b>
    #[test]
    fn test_bcd_sum_with_overflow_1() {
        let mut register = Register::new();
        register.set_accumulator(0b0010_0100); // 24 in BCD
        register.set_bcd_bit(true);
        calculate('+', 0b0101_0110, &mut register); // 56 in BCD

        assert_eq!(register.a(), 0b1000_0000); // 80 in BCD
        assert_eq!(register.p(), 0b1111_0000);
    }

    #[test]
    fn test_bcd_sum_with_overflow_2() {
        let mut register = Register::new();
        register.set_accumulator(0b1001_0011); // 93 in BCD
        register.set_bcd_bit(true);
        calculate('+', 0b1000_0010, &mut register); // 82 in BCD

        assert_eq!(register.a(), 0b0111_0101); // 75 in BCD
        assert_eq!(register.p(), 0b0111_0001);
    }

    #[test]
    fn test_bcd_sum_with_overflow_3() {
        let mut register = Register::new();
        register.set_accumulator(0b1000_1001); // 89 in BCD
        register.set_bcd_bit(true);
        calculate('+', 0b0111_0110, &mut register); // 76 in BCD

        assert_eq!(register.a(), 0b0110_0101); // 65 in BCD
        assert_eq!(register.p(), 0b0011_0001);
    }

    #[test]
    fn test_bcd_sum_with_overflow_4() {
        let mut register = Register::new();
        register.set_accumulator(0b1000_0000); // 80 in BCD
        register.set_bcd_bit(true);
        calculate('+', 0b1111_0000, &mut register); // invalid number in BCD

        assert_eq!(register.a(), 0b1101_0000); // invalid result in BCD
        assert_eq!(register.p(), 0b1111_0001);
    }

    #[test]
    fn test_bcd_sum_with_overflow_5() {
        let mut register = Register::new();
        register.set_accumulator(0b1000_0000); // 80 in BCD
        register.set_bcd_bit(true);
        calculate('+', 0b1111_1010, &mut register); // invalid number in BCD

        assert_eq!(register.a(), 0b1110_0000); // invalid result in BCD
        assert_eq!(register.p(), 0b1011_0001);
    }

    #[test]
    fn test_bcd_sum_with_overflow_6() {
        let mut register = Register::new();
        register.set_accumulator(0b0010_1111); // invalid number in BCD
        register.set_bcd_bit(true);
        calculate('+', 0b0100_1111, &mut register); // invalid number in BCD

        assert_eq!(register.a(), 0b1000_0100); // invalid result in BCD
        assert_eq!(register.p(), 0b1011_0000);
    }
}
