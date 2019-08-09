/*
AND  AND Memory with Accumulator

     A AND M -> A                     N Z C I D V
                                      + + - - - -

     addressing    assembler    opc  bytes  cyles
     --------------------------------------------
     immidiate     AND #oper     29    2     2
     zeropage      AND oper      25    2     3
     zeropage,X    AND oper,X    35    2     4
     absolute      AND oper      2D    3     4
     absolute,X    AND oper,X    3D    3     4*
     absolute,Y    AND oper,Y    39    3     4*
     (indirect,X)  AND (oper,X)  21    2     6
     (indirect),Y  AND (oper),Y  31    2     5*

*   16-bit address words are little endian, lo(w)-byte first, followed by the hi(gh)-byte.
(An assembler will use a human readable, big-endian notation as in $HHLL.)

*/

use crate::cpu::addressing;
use crate::cpu::mnemonics::Mnemonic;
use crate::cpu::register::Register;
use crate::message_bus::MessageBus;
use crate::cpu::alu;

#[derive(Debug)]
pub struct And {
    mnemonic: String,
    opcode: u8
}

impl And {
    pub fn new(opcode: u8) -> And {
        return And { mnemonic: "AND".to_string(), opcode: opcode };
    }
}

impl Mnemonic for And {
    fn determine_bytes(&self) -> usize {
        return match self.opcode {
            0x29 => 2,
            0x25 => 2,
            0x35 => 2,
            0x2D => 3,
            0x3D => 3,
            0x39 => 3,
            0x21 => 2,
            0x31 => 2,
            _ => panic!("Invalid opcode `0x{:x}` for mnemonic {}", self.opcode, self.mnemonic)
        }
    }

    fn call(&self, arguments: Vec<u8>, register: &mut Register, message_bus: &mut MessageBus) -> u8 {
        match self.opcode {
            0x29 => return self.call_immidiate(arguments, register),
            0x25 => return self.call_zero_page(arguments, register, message_bus),
            0x35 => return self.call_zero_page_x(arguments, register, message_bus),
            0x2D => return self.call_absolute(arguments, register, message_bus),
            0x3D => return self.call_absolute_x(arguments, register, message_bus),
            0x39 => return self.call_absolute_y(arguments, register, message_bus),
            0x21 => return self.call_indirect_x(arguments, register, message_bus),
            0x31 => return self.call_indirect_y(arguments, register, message_bus),
            _ => panic!("Invalid opcode `0x{:x}` for mnemonic {}", self.opcode, self.mnemonic)
        }
    }

    fn call_immidiate(&self, arguments: Vec<u8>, register: &mut Register) -> u8 {
        let result = alu::and(register.a(), arguments[0], register);
        register.set_accumulator(result);

        return 2;
    }

    fn call_zero_page(&self, arguments: Vec<u8>, register: &mut Register, message_bus: &mut MessageBus) -> u8 {
        let (_memory_address, memory_value, _boundary_crossed) = addressing::zero_page(arguments, message_bus);

        let result = alu::and(register.a(), memory_value, register);
        register.set_accumulator(result);

        return 3;
    }

    fn call_zero_page_x(&self, arguments: Vec<u8>, register: &mut Register, message_bus: &mut MessageBus) -> u8 {
        let (_memory_address, memory_value, _boundary_crossed) = addressing::zero_page_x(arguments, message_bus, register);

        let result = alu::and(register.a(), memory_value, register);
        register.set_accumulator(result);

        return 4;
    }

    fn call_absolute(&self, arguments: Vec<u8>, register: &mut Register, message_bus: &mut MessageBus) -> u8 {
        let (_memory_address, memory_value, _boundary_crossed) = addressing::absolute(arguments, message_bus);

        let result = alu::and(register.a(), memory_value, register);
        register.set_accumulator(result);

        return 4;
    }

    fn call_absolute_x(&self, arguments: Vec<u8>, register: &mut Register, message_bus: &mut MessageBus) -> u8 {
        let (_memory_address, memory_value, boundary_crossed) = addressing::absolute_x(arguments, message_bus, register);

        let result = alu::and(register.a(), memory_value, register);
        register.set_accumulator(result);

        return if boundary_crossed { 5u8 } else { 4u8 };
    }

    fn call_absolute_y(&self, arguments: Vec<u8>, register: &mut Register, message_bus: &mut MessageBus) -> u8 {
        let (_memory_address, memory_value, boundary_crossed) = addressing::absolute_y(arguments, message_bus, register);

        let result = alu::and(register.a(), memory_value, register);
        register.set_accumulator(result);

        return if boundary_crossed { 5u8 } else { 4u8 };
    }

    fn call_indirect_x(&self, arguments: Vec<u8>, register: &mut Register, message_bus: &mut MessageBus) -> u8 {
        let (_memory_address, memory_value, _boundary_crossed) = addressing::indirect_x(arguments, message_bus, register);

        let result = alu::and(register.a(), memory_value, register);
        register.set_accumulator(result);

        return 6;
    }

    fn call_indirect_y(&self, arguments: Vec<u8>, register: &mut Register, message_bus: &mut MessageBus) -> u8 {
        let (_memory_address, memory_value, boundary_crossed) = addressing::indirect_y(arguments, message_bus, register);

        let result = alu::and(register.a(), memory_value, register);
        register.set_accumulator(result);

        return if boundary_crossed { 6u8 } else { 5u8 };
    }
}

#[cfg(test)]
mod tests {
    use super::And;
    use crate::cpu::mnemonics::Mnemonic;
    use crate::cpu::register::Register;
    use crate::memory::Memory;
    use crate::message_bus::MessageBus;

    #[test]
    fn test_immidiate() {
        let and = And::new(0x29);
        let arguments = vec![0b1010_0101];
        let mut memory = Memory::new();
        let mut register = Register::new();
        register.set_accumulator(0b0101_0101);

        let mut message_bus = MessageBus::new(&mut memory);

        let cycles = and.call(arguments, &mut register, &mut message_bus);

        assert_eq!(register.a(), 0b0000_0101);
        assert_eq!(register.p(), 0b0010_0000);
        assert_eq!(cycles, 2);
    }

    #[test]
    fn test_zero_page() {
        let and = And::new(0x25);
        let arguments = vec![0x30];
        let mut memory = Memory::new();
        memory.write_byte(0x30, 0b1010_0101);

        let mut register = Register::new();
        register.set_accumulator(0b0101_0101);

        let mut message_bus = MessageBus::new(&mut memory);

        let cycles = and.call(arguments, &mut register, &mut message_bus);

        assert_eq!(register.a(), 0b0000_0101);
        assert_eq!(register.p(), 0b0010_0000);
        assert_eq!(cycles, 3);
    }

    #[test]
    fn test_zero_page_x() {
        let and = And::new(0x35);
        let arguments = vec![0x30];
        let mut memory = Memory::new();
        memory.write_byte(0x35, 0b1010_0101);

        let mut register = Register::new();
        register.set_accumulator(0b0101_0101);
        register.set_x(0x05);

        let mut message_bus = MessageBus::new(&mut memory);

        let cycles = and.call(arguments, &mut register, &mut message_bus);

        assert_eq!(register.a(), 0b0000_0101);
        assert_eq!(register.p(), 0b0010_0000);
        assert_eq!(cycles, 4);
    }

    #[test]
    fn test_absolute() {
        let and = And::new(0x2D);
        let arguments = vec![0x3c, 0x5a];
        let mut memory = Memory::new();
        memory.write_byte(0x5a3c, 0b1010_0101);

        let mut register = Register::new();
        register.set_accumulator(0b0101_0101);

        let mut message_bus = MessageBus::new(&mut memory);

        let cycles = and.call(arguments, &mut register, &mut message_bus);

        assert_eq!(register.a(), 0b0000_0101);
        assert_eq!(register.p(), 0b0010_0000);
        assert_eq!(cycles, 4);
    }

    #[test]
    fn test_absolute_x() {
        let and = And::new(0x3D);
        let arguments = vec![0x3c, 0x5a];
        let mut memory = Memory::new();
        memory.write_byte(0x5a4c, 0b1010_0101);

        let mut register = Register::new();
        register.set_accumulator(0b0101_0101);
        register.set_x(0x10);
        register.set_y(0x20);

        let mut message_bus = MessageBus::new(&mut memory);

        let cycles = and.call(arguments, &mut register, &mut message_bus);

        assert_eq!(register.a(), 0b0000_0101);
        assert_eq!(register.p(), 0b0010_0000);
        assert_eq!(cycles, 4);
    }

    #[test]
    fn test_absolute_x_out_of_bounds() {
        let and = And::new(0x3D);
        let arguments = vec![0xff, 0xff];
        let mut memory = Memory::new();
        memory.write_byte(0x5a, 0b1010_0101);

        let mut register = Register::new();
        register.set_accumulator(0b0101_0101);
        register.set_x(0x5b);
        register.set_y(0x20);

        let mut message_bus = MessageBus::new(&mut memory);

        let cycles = and.call(arguments, &mut register, &mut message_bus);

        assert_eq!(register.a(), 0b0000_0101);
        assert_eq!(register.p(), 0b0010_0000);
        assert_eq!(cycles, 5);
    }

    #[test]
    fn test_absolute_y() {
        let and = And::new(0x39);
        let arguments = vec![0x3c, 0x5a];
        let mut memory = Memory::new();
        memory.write_byte(0x5a4c, 0b1010_0101);

        let mut register = Register::new();
        register.set_accumulator(0b0101_0101);
        register.set_x(0x20);
        register.set_y(0x10);

        let mut message_bus = MessageBus::new(&mut memory);

        let cycles = and.call(arguments, &mut register, &mut message_bus);

        assert_eq!(register.a(), 0b0000_0101);
        assert_eq!(register.p(), 0b0010_0000);
        assert_eq!(cycles, 4);
    }

    #[test]
    fn test_absolute_y_out_of_bounds() {
        let and = And::new(0x39);
        let arguments = vec![0xff, 0xff];
        let mut memory = Memory::new();
        memory.write_byte(0x5a, 0b1010_0101);

        let mut register = Register::new();
        register.set_accumulator(0b0101_0101);
        register.set_x(0x20);
        register.set_y(0x5b);

        let mut message_bus = MessageBus::new(&mut memory);

        let cycles = and.call(arguments, &mut register, &mut message_bus);

        assert_eq!(register.a(), 0b0000_0101);
        assert_eq!(register.p(), 0b0010_0000);
        assert_eq!(cycles, 5);
    }

    #[test]
    fn test_indirect_x() {
        let and = And::new(0x21);
        let arguments = vec![0x44];
        let mut memory = Memory::new();
        memory.write_byte(0x77, 0x05);
        memory.write_byte(0x78, 0x01);
        memory.write_byte(0x0105, 0b1010_0101);

        let mut register = Register::new();
        register.set_accumulator(0b0101_0101);
        register.set_x(0x33);

        let mut message_bus = MessageBus::new(&mut memory);

        let cycles = and.call(arguments, &mut register, &mut message_bus);

        assert_eq!(register.a(), 0b0000_0101);
        assert_eq!(register.p(), 0b0010_0000);
        assert_eq!(cycles, 6);
    }

    #[test]
    fn test_indirect_y() {
        let and = And::new(0x31);
        let arguments = vec![0x77];
        let mut memory = Memory::new();
        memory.write_byte(0x77, 0x05);
        memory.write_byte(0x78, 0x01);
        memory.write_byte(0x0109, 0b1010_0101);

        let mut register = Register::new();
        register.set_accumulator(0b0101_0101);
        register.set_y(0x04);

        let mut message_bus = MessageBus::new(&mut memory);

        let cycles = and.call(arguments, &mut register, &mut message_bus);

        assert_eq!(register.a(), 0b0000_0101);
        assert_eq!(register.p(), 0b0010_0000);
        assert_eq!(cycles, 5);
    }

    #[test]
    fn test_indirect_y_out_of_bonds() {
        let and = And::new(0x31);
        let arguments = vec![0x77];
        let mut memory = Memory::new();
        memory.write_byte(0x77, 0xff);
        memory.write_byte(0x78, 0x01);
        memory.write_byte(0x0205, 0b1010_0101);

        let mut register = Register::new();
        register.set_accumulator(0b0101_0101);
        register.set_y(0x06);

        let mut message_bus = MessageBus::new(&mut memory);

        let cycles = and.call(arguments, &mut register, &mut message_bus);

        assert_eq!(register.a(), 0b0000_0101);
        assert_eq!(register.p(), 0b0010_0000);
        assert_eq!(cycles, 6);
    }

    #[test]
    #[should_panic]
    fn test_invalid_opcode() {
        let and = And::new(0x00);
        let arguments = vec![0xFF];
        let mut memory = Memory::new();
        let mut message_bus = MessageBus::new(&mut memory);
        let mut register = Register::new();

        and.call(arguments, &mut register, &mut message_bus);
    }
}
