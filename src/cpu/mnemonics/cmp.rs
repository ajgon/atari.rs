/*
CMP  Compare Memory with Accumulator

     A - M                            N Z C I D V
                                      + + + - - -

     addressing    assembler    opc  bytes  cyles
     --------------------------------------------
     immidiate     CMP #oper     C9    2     2
     zeropage      CMP oper      C5    2     3
     zeropage,X    CMP oper,X    D5    2     4
     absolute      CMP oper      CD    3     4
     absolute,X    CMP oper,X    DD    3     4*
     absolute,Y    CMP oper,Y    D9    3     4*
     (indirect,X)  CMP (oper,X)  C1    2     6
     (indirect),Y  CMP (oper),Y  D1    2     5*
*/

use crate::cpu::addressing;
use crate::cpu::mnemonics::Mnemonic;
use crate::cpu::register::Register;
use crate::message_bus::MessageBus;
use crate::cpu::alu;

#[derive(Debug)]
pub struct Cmp {
    mnemonic: String,
    opcode: u8
}

impl Cmp {
    pub fn new(opcode: u8) -> Cmp {
        return Cmp { mnemonic: "CMP".to_string(), opcode: opcode };
    }
}

impl Mnemonic for Cmp {
    fn determine_bytes(&self) -> usize {
        return match self.opcode {
            0xC9 => 2,
            0xC5 => 2,
            0xD5 => 2,
            0xCD => 3,
            0xDD => 3,
            0xD9 => 3,
            0xC1 => 2,
            0xD1 => 2,
            _ => panic!("Invalid opcode `0x{:x}` for mnemonic {}", self.opcode, self.mnemonic)
        }
    }

    fn call(&self, arguments: Vec<u8>, register: &mut Register, message_bus: &mut MessageBus) -> u8 {
        match self.opcode {
            0xC9 => return self.call_immidiate(arguments, register),
            0xC5 => return self.call_zero_page(arguments, register, message_bus),
            0xD5 => return self.call_zero_page_x(arguments, register, message_bus),
            0xCD => return self.call_absolute(arguments, register, message_bus),
            0xDD => return self.call_absolute_x(arguments, register, message_bus),
            0xD9 => return self.call_absolute_y(arguments, register, message_bus),
            0xC1 => return self.call_indirect_x(arguments, register, message_bus),
            0xD1 => return self.call_indirect_y(arguments, register, message_bus),
            _ => panic!("Invalid opcode `0x{:x}` for mnemonic {}", self.opcode, self.mnemonic)
        }
    }

    fn call_immidiate(&self, arguments: Vec<u8>, register: &mut Register) -> u8 {
        let decimal_bit = register.decimal_bit();
        let overflow_bit = register.overflow_bit();

        register.set_carry_bit(true);
        register.set_decimal_bit(false);
        alu::subtract(register.a(), arguments[0], register);
        register.set_decimal_bit(decimal_bit);
        register.set_overflow_bit(overflow_bit);

        return 2;
    }

    fn call_zero_page(&self, arguments: Vec<u8>, register: &mut Register, message_bus: &mut MessageBus) -> u8 {
        let (_memory_address, memory_value, _boundary_crossed) = addressing::zero_page(arguments, message_bus);
        let decimal_bit = register.decimal_bit();
        let overflow_bit = register.overflow_bit();

        register.set_carry_bit(true);
        register.set_decimal_bit(false);
        alu::subtract(register.a(), memory_value, register);
        register.set_decimal_bit(decimal_bit);
        register.set_overflow_bit(overflow_bit);

        return 3;
    }

    fn call_zero_page_x(&self, arguments: Vec<u8>, register: &mut Register, message_bus: &mut MessageBus) -> u8 {
        let (_memory_address, memory_value, _boundary_crossed) = addressing::zero_page_x(arguments, message_bus, register);
        let decimal_bit = register.decimal_bit();
        let overflow_bit = register.overflow_bit();

        register.set_carry_bit(true);
        register.set_decimal_bit(false);
        alu::subtract(register.a(), memory_value, register);
        register.set_decimal_bit(decimal_bit);
        register.set_overflow_bit(overflow_bit);

        return 4;
    }

    fn call_absolute(&self, arguments: Vec<u8>, register: &mut Register, message_bus: &mut MessageBus) -> u8 {
        let (_memory_address, memory_value, _boundary_crossed) = addressing::absolute(arguments, message_bus);
        let decimal_bit = register.decimal_bit();
        let overflow_bit = register.overflow_bit();

        register.set_carry_bit(true);
        register.set_decimal_bit(false);
        alu::subtract(register.a(), memory_value, register);
        register.set_decimal_bit(decimal_bit);
        register.set_overflow_bit(overflow_bit);

        return 4;
    }

    fn call_absolute_x(&self, arguments: Vec<u8>, register: &mut Register, message_bus: &mut MessageBus) -> u8 {
        let (_memory_address, memory_value, boundary_crossed) = addressing::absolute_x(arguments, message_bus, register);
        let decimal_bit = register.decimal_bit();
        let overflow_bit = register.overflow_bit();

        register.set_carry_bit(true);
        register.set_decimal_bit(false);
        alu::subtract(register.a(), memory_value, register);
        register.set_decimal_bit(decimal_bit);
        register.set_overflow_bit(overflow_bit);

        return if boundary_crossed { 5u8 } else { 4u8 };
    }

    fn call_absolute_y(&self, arguments: Vec<u8>, register: &mut Register, message_bus: &mut MessageBus) -> u8 {
        let (_memory_address, memory_value, boundary_crossed) = addressing::absolute_y(arguments, message_bus, register);
        let decimal_bit = register.decimal_bit();
        let overflow_bit = register.overflow_bit();

        register.set_carry_bit(true);
        register.set_decimal_bit(false);
        alu::subtract(register.a(), memory_value, register);
        register.set_decimal_bit(decimal_bit);
        register.set_overflow_bit(overflow_bit);

        return if boundary_crossed { 5u8 } else { 4u8 };
    }

    fn call_indirect_x(&self, arguments: Vec<u8>, register: &mut Register, message_bus: &mut MessageBus) -> u8 {
        let (_memory_address, memory_value, _boundary_crossed) = addressing::indirect_x(arguments, message_bus, register);
        let decimal_bit = register.decimal_bit();
        let overflow_bit = register.overflow_bit();

        register.set_carry_bit(true);
        register.set_decimal_bit(false);
        alu::subtract(register.a(), memory_value, register);
        register.set_decimal_bit(decimal_bit);
        register.set_overflow_bit(overflow_bit);

        return 6;
    }

    fn call_indirect_y(&self, arguments: Vec<u8>, register: &mut Register, message_bus: &mut MessageBus) -> u8 {
        let (_memory_address, memory_value, boundary_crossed) = addressing::indirect_y(arguments, message_bus, register);
        let decimal_bit = register.decimal_bit();
        let overflow_bit = register.overflow_bit();

        register.set_carry_bit(true);
        register.set_decimal_bit(false);
        alu::subtract(register.a(), memory_value, register);
        register.set_decimal_bit(decimal_bit);
        register.set_overflow_bit(overflow_bit);

        return if boundary_crossed { 6u8 } else { 5u8 };
    }
}

#[cfg(test)]
mod tests {
    use super::Cmp;
    use crate::cpu::mnemonics::Mnemonic;
    use crate::cpu::register::Register;
    use crate::memory::Memory;
    use crate::message_bus::MessageBus;

    #[test]
    fn test_immidiate_argument_equals() {
        let cmp = Cmp::new(0xC9);
        let arguments = vec![0x42];
        let mut memory = Memory::new();
        let mut register = Register::new();
        register.set_accumulator(0x42);

        let mut message_bus = MessageBus::new(&mut memory);

        let cycles = cmp.call(arguments, &mut register, &mut message_bus);

        assert_eq!(register.a(), 0x42);
        assert_eq!(register.p(), 0b0010_0011);
        assert_eq!(cycles, 2);
    }

    #[test]
    fn test_immidiate_argument_greater() {
        let cmp = Cmp::new(0xC9);
        let arguments = vec![0x62];
        let mut memory = Memory::new();
        let mut register = Register::new();
        register.set_accumulator(0x42);

        let mut message_bus = MessageBus::new(&mut memory);

        let cycles = cmp.call(arguments, &mut register, &mut message_bus);

        assert_eq!(register.a(), 0x42);
        assert_eq!(register.p(), 0b1010_0000);
        assert_eq!(cycles, 2);
    }

    #[test]
    fn test_immidiate_argument_greater_negative() {
        let cmp = Cmp::new(0xC9);
        let arguments = vec![0xF2];
        let mut memory = Memory::new();
        let mut register = Register::new();
        register.set_accumulator(0x42);

        let mut message_bus = MessageBus::new(&mut memory);

        let cycles = cmp.call(arguments, &mut register, &mut message_bus);

        assert_eq!(register.a(), 0x42);
        assert_eq!(register.p(), 0b0010_0000);
        assert_eq!(cycles, 2);
    }

    #[test]
    fn test_immidiate_bcd_math_no_affect() {
        let cmp = Cmp::new(0xC9);
        let arguments = vec![0xF2];
        let mut memory = Memory::new();
        let mut register = Register::new();
        register.set_accumulator(0x42);
        register.set_decimal_bit(true);

        let mut message_bus = MessageBus::new(&mut memory);

        let cycles = cmp.call(arguments, &mut register, &mut message_bus);

        assert_eq!(register.a(), 0x42);
        assert_eq!(register.p(), 0b0010_1000);
        assert_eq!(cycles, 2);
    }

    #[test]
    fn test_immidiate_overflow_no_affect() {
        let cmp = Cmp::new(0xC9);
        let arguments = vec![0x01];
        let mut memory = Memory::new();
        let mut register = Register::new();
        register.set_accumulator(0x80);

        let mut message_bus = MessageBus::new(&mut memory);

        let cycles = cmp.call(arguments, &mut register, &mut message_bus);

        assert_eq!(register.a(), 0x80);
        assert_eq!(register.p(), 0b0010_0001);
        assert_eq!(cycles, 2);
    }

    #[test]
    fn test_immidiate_argument_less() {
        let cmp = Cmp::new(0xC9);
        let arguments = vec![0x22];
        let mut memory = Memory::new();
        let mut register = Register::new();
        register.set_accumulator(0x42);

        let mut message_bus = MessageBus::new(&mut memory);

        let cycles = cmp.call(arguments, &mut register, &mut message_bus);

        assert_eq!(register.a(), 0x42);
        assert_eq!(register.p(), 0b0010_0001);
        assert_eq!(cycles, 2);
    }

    #[test]
    fn test_immidiate_argument_less_negative() {
        let cmp = Cmp::new(0xC9);
        let arguments = vec![0xC2];
        let mut memory = Memory::new();
        let mut register = Register::new();
        register.set_accumulator(0xF2);

        let mut message_bus = MessageBus::new(&mut memory);

        let cycles = cmp.call(arguments, &mut register, &mut message_bus);

        assert_eq!(register.a(), 0xF2);
        assert_eq!(register.p(), 0b0010_0001);
        assert_eq!(cycles, 2);
    }

    #[test]
    fn test_zero_page() {
        let cmp = Cmp::new(0xC5);
        let arguments = vec![0x30];
        let mut memory = Memory::new();
        memory.write_byte(0x30, 0xF2);

        let mut register = Register::new();
        register.set_accumulator(0x42);

        let mut message_bus = MessageBus::new(&mut memory);

        let cycles = cmp.call(arguments, &mut register, &mut message_bus);

        assert_eq!(register.a(), 0x42);
        assert_eq!(register.p(), 0b0010_0000);
        assert_eq!(cycles, 3);
    }

    #[test]
    fn test_zero_page_x() {
        let cmp = Cmp::new(0xD5);
        let arguments = vec![0x30];
        let mut memory = Memory::new();
        memory.write_byte(0x35, 0xF2);

        let mut register = Register::new();
        register.set_accumulator(0x42);
        register.set_x(0x05);

        let mut message_bus = MessageBus::new(&mut memory);

        let cycles = cmp.call(arguments, &mut register, &mut message_bus);

        assert_eq!(register.a(), 0x42);
        assert_eq!(register.p(), 0b0010_0000);
        assert_eq!(cycles, 4);
    }

    #[test]
    fn test_absolute() {
        let cmp = Cmp::new(0xCD);
        let arguments = vec![0x3c, 0x5a];
        let mut memory = Memory::new();
        memory.write_byte(0x5a3c, 0xF2);

        let mut register = Register::new();
        register.set_accumulator(0x42);

        let mut message_bus = MessageBus::new(&mut memory);

        let cycles = cmp.call(arguments, &mut register, &mut message_bus);

        assert_eq!(register.a(), 0x42);
        assert_eq!(register.p(), 0b0010_0000);
        assert_eq!(cycles, 4);
    }

    #[test]
    fn test_absolute_x() {
        let cmp = Cmp::new(0xDD);
        let arguments = vec![0x3c, 0x5a];
        let mut memory = Memory::new();
        memory.write_byte(0x5a4c, 0xF2);

        let mut register = Register::new();
        register.set_accumulator(0x42);
        register.set_x(0x10);
        register.set_y(0x20);

        let mut message_bus = MessageBus::new(&mut memory);

        let cycles = cmp.call(arguments, &mut register, &mut message_bus);

        assert_eq!(register.a(), 0x42);
        assert_eq!(register.p(), 0b0010_0000);
        assert_eq!(cycles, 4);
    }

    #[test]
    fn test_absolute_x_out_of_bounds() {
        let cmp = Cmp::new(0xDD);
        let arguments = vec![0xff, 0xff];
        let mut memory = Memory::new();
        memory.write_byte(0x5a, 0xF2);

        let mut register = Register::new();
        register.set_accumulator(0x42);
        register.set_x(0x5b);
        register.set_y(0x20);

        let mut message_bus = MessageBus::new(&mut memory);

        let cycles = cmp.call(arguments, &mut register, &mut message_bus);

        assert_eq!(register.a(), 0x42);
        assert_eq!(register.p(), 0b0010_0000);
        assert_eq!(cycles, 5);
    }

    #[test]
    fn test_absolute_y() {
        let cmp = Cmp::new(0xD9);
        let arguments = vec![0x3c, 0x5a];
        let mut memory = Memory::new();
        memory.write_byte(0x5a4c, 0xF2);

        let mut register = Register::new();
        register.set_accumulator(0x42);
        register.set_x(0x20);
        register.set_y(0x10);

        let mut message_bus = MessageBus::new(&mut memory);

        let cycles = cmp.call(arguments, &mut register, &mut message_bus);

        assert_eq!(register.a(), 0x42);
        assert_eq!(register.p(), 0b0010_0000);
        assert_eq!(cycles, 4);
    }

    #[test]
    fn test_absolute_y_out_of_bounds() {
        let cmp = Cmp::new(0xD9);
        let arguments = vec![0xff, 0xff];
        let mut memory = Memory::new();
        memory.write_byte(0x5a, 0xF2);

        let mut register = Register::new();
        register.set_accumulator(0x42);
        register.set_x(0x20);
        register.set_y(0x5b);

        let mut message_bus = MessageBus::new(&mut memory);

        let cycles = cmp.call(arguments, &mut register, &mut message_bus);

        assert_eq!(register.a(), 0x42);
        assert_eq!(register.p(), 0b0010_0000);
        assert_eq!(cycles, 5);
    }

    #[test]
    fn test_indirect_x() {
        let cmp = Cmp::new(0xC1);
        let arguments = vec![0x44];
        let mut memory = Memory::new();
        memory.write_byte(0x77, 0x05);
        memory.write_byte(0x78, 0x01);
        memory.write_byte(0x0105, 0xF2);

        let mut register = Register::new();
        register.set_accumulator(0x42);
        register.set_x(0x33);

        let mut message_bus = MessageBus::new(&mut memory);

        let cycles = cmp.call(arguments, &mut register, &mut message_bus);

        assert_eq!(register.a(), 0x42);
        assert_eq!(register.p(), 0b0010_0000);
        assert_eq!(cycles, 6);
    }

    #[test]
    fn test_indirect_y() {
        let cmp = Cmp::new(0xD1);
        let arguments = vec![0x77];
        let mut memory = Memory::new();
        memory.write_byte(0x77, 0x05);
        memory.write_byte(0x78, 0x01);
        memory.write_byte(0x0109, 0xF2);

        let mut register = Register::new();
        register.set_accumulator(0x42);
        register.set_y(0x04);

        let mut message_bus = MessageBus::new(&mut memory);

        let cycles = cmp.call(arguments, &mut register, &mut message_bus);

        assert_eq!(register.a(), 0x42);
        assert_eq!(register.p(), 0b0010_0000);
        assert_eq!(cycles, 5);
    }

    #[test]
    fn test_indirect_y_out_of_bonds() {
        let cmp = Cmp::new(0xD1);
        let arguments = vec![0x77];
        let mut memory = Memory::new();
        memory.write_byte(0x77, 0xff);
        memory.write_byte(0x78, 0x01);
        memory.write_byte(0x0205, 0xF2);

        let mut register = Register::new();
        register.set_accumulator(0x42);
        register.set_y(0x06);

        let mut message_bus = MessageBus::new(&mut memory);

        let cycles = cmp.call(arguments, &mut register, &mut message_bus);

        assert_eq!(register.a(), 0x42);
        assert_eq!(register.p(), 0b0010_0000);
        assert_eq!(cycles, 6);
    }

    #[test]
    #[should_panic]
    fn test_invalid_opcode() {
        let cmp = Cmp::new(0x00);
        let arguments = vec![0xFF];
        let mut memory = Memory::new();
        let mut message_bus = MessageBus::new(&mut memory);
        let mut register = Register::new();

        cmp.call(arguments, &mut register, &mut message_bus);
    }
}
