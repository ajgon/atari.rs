/*
SBC  Subtract Memory from Accumulator with Borrow

     A - M - C -> A                   N Z C I D V
                                      + + + - - +

     addressing    assembler    opc  bytes  cyles
     --------------------------------------------
     immidiate     SBC #oper     E9    2     2
     zeropage      SBC oper      E5    2     3
     zeropage,X    SBC oper,X    F5    2     4
     absolute      SBC oper      ED    3     4
     absolute,X    SBC oper,X    FD    3     4*
     absolute,Y    SBC oper,Y    F9    3     4*
     (indirect,X)  SBC (oper,X)  E1    2     6
     (indirect),Y  SBC (oper),Y  F1    2     5*
*/

use crate::cpu::addressing;
use crate::cpu::mnemonics::Mnemonic;
use crate::cpu::register::Register;
use crate::message_bus::MessageBus;
use crate::cpu::alu;

#[derive(Debug)]
pub struct Sbc {
    mnemonic: String,
    opcode: u8
}

impl Sbc {
    pub fn new(opcode: u8) -> Sbc {
        return Sbc { mnemonic: "SBC".to_string(), opcode: opcode };
    }
}

impl Mnemonic for Sbc {
    fn determine_bytes(&self) -> usize {
        return match self.opcode {
            0xE9 => 2,
            0xE5 => 2,
            0xF5 => 2,
            0xED => 3,
            0xFD => 3,
            0xF9 => 3,
            0xE1 => 2,
            0xF1 => 2,
            _ => panic!("Invalid opcode `0x{:x}` for mnemonic {}", self.opcode, self.mnemonic)
        }
    }

    fn call(&self, arguments: Vec<u8>, register: &mut Register, message_bus: &mut MessageBus) -> u8 {
        match self.opcode {
            0xE9 => return self.call_immidiate(arguments, register),
            0xE5 => return self.call_zero_page(arguments, register, message_bus),
            0xF5 => return self.call_zero_page_x(arguments, register, message_bus),
            0xED => return self.call_absolute(arguments, register, message_bus),
            0xFD => return self.call_absolute_x(arguments, register, message_bus),
            0xF9 => return self.call_absolute_y(arguments, register, message_bus),
            0xE1 => return self.call_indirect_x(arguments, register, message_bus),
            0xF1 => return self.call_indirect_y(arguments, register, message_bus),
            _ => panic!("Invalid opcode `0x{:x}` for mnemonic {}", self.opcode, self.mnemonic)
        }
    }

    fn call_immidiate(&self, arguments: Vec<u8>, register: &mut Register) -> u8 {
        let result = alu::subtract(register.a(), arguments[0], register);
        register.set_accumulator(result);

        return 2;
    }

    fn call_zero_page(&self, arguments: Vec<u8>, register: &mut Register, message_bus: &mut MessageBus) -> u8 {
        let (_memory_address, memory_value, _boundary_crossed) = addressing::zero_page(arguments, message_bus);

        let result = alu::subtract(register.a(), memory_value, register);
        register.set_accumulator(result);

        return 3;
    }

    fn call_zero_page_x(&self, arguments: Vec<u8>, register: &mut Register, message_bus: &mut MessageBus) -> u8 {
        let (_memory_address, memory_value, _boundary_crossed) = addressing::zero_page_x(arguments, message_bus, register);

        let result = alu::subtract(register.a(), memory_value, register);
        register.set_accumulator(result);

        return 4;
    }

    fn call_absolute(&self, arguments: Vec<u8>, register: &mut Register, message_bus: &mut MessageBus) -> u8 {
        let (_memory_address, memory_value, _boundary_crossed) = addressing::absolute(arguments, message_bus);

        let result = alu::subtract(register.a(), memory_value, register);
        register.set_accumulator(result);

        return 4;
    }

    fn call_absolute_x(&self, arguments: Vec<u8>, register: &mut Register, message_bus: &mut MessageBus) -> u8 {
        let (_memory_address, memory_value, boundary_crossed) = addressing::absolute_x(arguments, message_bus, register);

        let result = alu::subtract(register.a(), memory_value, register);
        register.set_accumulator(result);

        return if boundary_crossed { 5u8 } else { 4u8 };
    }

    fn call_absolute_y(&self, arguments: Vec<u8>, register: &mut Register, message_bus: &mut MessageBus) -> u8 {
        let (_memory_address, memory_value, boundary_crossed) = addressing::absolute_y(arguments, message_bus, register);

        let result = alu::subtract(register.a(), memory_value, register);
        register.set_accumulator(result);

        return if boundary_crossed { 5u8 } else { 4u8 };
    }

    fn call_indirect_x(&self, arguments: Vec<u8>, register: &mut Register, message_bus: &mut MessageBus) -> u8 {
        let (_memory_address, memory_value, _boundary_crossed) = addressing::indirect_x(arguments, message_bus, register);

        let result = alu::subtract(register.a(), memory_value, register);
        register.set_accumulator(result);

        return 6;
    }

    fn call_indirect_y(&self, arguments: Vec<u8>, register: &mut Register, message_bus: &mut MessageBus) -> u8 {
        let (_memory_address, memory_value, boundary_crossed) = addressing::indirect_y(arguments, message_bus, register);

        let result = alu::subtract(register.a(), memory_value, register);
        register.set_accumulator(result);

        return if boundary_crossed { 6u8 } else { 5u8 };
    }
}

#[cfg(test)]
mod tests {
    use super::Sbc;
    use crate::cpu::mnemonics::Mnemonic;
    use crate::cpu::register::Register;
    use crate::memory::Memory;
    use crate::message_bus::MessageBus;

    #[test]
    fn test_immidiate() {
        let sbc = Sbc::new(0xE9);
        let arguments = vec![0x03];
        let mut memory = Memory::new();
        let mut register = Register::new();
        register.set_accumulator(0x45);
        register.set_carry_bit(true);

        let mut message_bus = MessageBus::new(&mut memory);

        let cycles = sbc.call(arguments, &mut register, &mut message_bus);

        assert_eq!(register.a(), 0x42);
        assert_eq!(register.p(), 0b00110001);
        assert_eq!(cycles, 2);
    }

    #[test]
    fn test_zero_page() {
        let sbc = Sbc::new(0xE5);
        let arguments = vec![0x30];
        let mut memory = Memory::new();
        memory.write_byte(0x30, 0x03);

        let mut register = Register::new();
        register.set_accumulator(0x45);
        register.set_carry_bit(true);

        let mut message_bus = MessageBus::new(&mut memory);

        let cycles = sbc.call(arguments, &mut register, &mut message_bus);

        assert_eq!(register.a(), 0x42);
        assert_eq!(register.p(), 0b00110001);
        assert_eq!(cycles, 3);
    }

    #[test]
    fn test_zero_page_x() {
        let sbc = Sbc::new(0xF5);
        let arguments = vec![0x30];
        let mut memory = Memory::new();
        memory.write_byte(0x35, 0x03);

        let mut register = Register::new();
        register.set_accumulator(0x45);
        register.set_carry_bit(true);
        register.set_x(0x05);

        let mut message_bus = MessageBus::new(&mut memory);

        let cycles = sbc.call(arguments, &mut register, &mut message_bus);

        assert_eq!(register.a(), 0x42);
        assert_eq!(register.p(), 0b00110001);
        assert_eq!(cycles, 4);
    }

    #[test]
    fn test_absolute() {
        let sbc = Sbc::new(0xED);
        let arguments = vec![0x3c, 0x5a];
        let mut memory = Memory::new();
        memory.write_byte(0x5a3c, 0x03);

        let mut register = Register::new();
        register.set_accumulator(0x45);
        register.set_carry_bit(true);

        let mut message_bus = MessageBus::new(&mut memory);

        let cycles = sbc.call(arguments, &mut register, &mut message_bus);

        assert_eq!(register.a(), 0x42);
        assert_eq!(register.p(), 0b00110001);
        assert_eq!(cycles, 4);
    }

    #[test]
    fn test_absolute_x() {
        let sbc = Sbc::new(0xFD);
        let arguments = vec![0x3c, 0x5a];
        let mut memory = Memory::new();
        memory.write_byte(0x5a4c, 0x03);

        let mut register = Register::new();
        register.set_accumulator(0x45);
        register.set_carry_bit(true);
        register.set_x(0x10);
        register.set_y(0x20);

        let mut message_bus = MessageBus::new(&mut memory);

        let cycles = sbc.call(arguments, &mut register, &mut message_bus);

        assert_eq!(register.a(), 0x42);
        assert_eq!(register.p(), 0b00110001);
        assert_eq!(cycles, 4);
    }

    #[test]
    fn test_absolute_x_out_of_bounds() {
        let sbc = Sbc::new(0xFD);
        let arguments = vec![0xff, 0xff];
        let mut memory = Memory::new();
        memory.write_byte(0x5a, 0x03);

        let mut register = Register::new();
        register.set_accumulator(0x45);
        register.set_carry_bit(true);
        register.set_x(0x5b);
        register.set_y(0x20);

        let mut message_bus = MessageBus::new(&mut memory);

        let cycles = sbc.call(arguments, &mut register, &mut message_bus);

        assert_eq!(register.a(), 0x42);
        assert_eq!(register.p(), 0b00110001);
        assert_eq!(cycles, 5);
    }

    #[test]
    fn test_absolute_y() {
        let sbc = Sbc::new(0xF9);
        let arguments = vec![0x3c, 0x5a];
        let mut memory = Memory::new();
        memory.write_byte(0x5a4c, 0x03);

        let mut register = Register::new();
        register.set_accumulator(0x45);
        register.set_carry_bit(true);
        register.set_x(0x20);
        register.set_y(0x10);

        let mut message_bus = MessageBus::new(&mut memory);

        let cycles = sbc.call(arguments, &mut register, &mut message_bus);

        assert_eq!(register.a(), 0x42);
        assert_eq!(register.p(), 0b00110001);
        assert_eq!(cycles, 4);
    }

    #[test]
    fn test_absolute_y_out_of_bounds() {
        let sbc = Sbc::new(0xF9);
        let arguments = vec![0xff, 0xff];
        let mut memory = Memory::new();
        memory.write_byte(0x5a, 0x03);

        let mut register = Register::new();
        register.set_accumulator(0x45);
        register.set_carry_bit(true);
        register.set_x(0x20);
        register.set_y(0x5b);

        let mut message_bus = MessageBus::new(&mut memory);

        let cycles = sbc.call(arguments, &mut register, &mut message_bus);

        assert_eq!(register.a(), 0x42);
        assert_eq!(register.p(), 0b00110001);
        assert_eq!(cycles, 5);
    }

    #[test]
    fn test_indirect_x() {
        let sbc = Sbc::new(0xE1);
        let arguments = vec![0x44];
        let mut memory = Memory::new();
        memory.write_byte(0x77, 0x05);
        memory.write_byte(0x78, 0x01);
        memory.write_byte(0x0105, 0x07);

        let mut register = Register::new();
        register.set_accumulator(0x1F);
        register.set_x(0x33);

        let mut message_bus = MessageBus::new(&mut memory);

        let cycles = sbc.call(arguments, &mut register, &mut message_bus);

        assert_eq!(register.a(), 0x17);
        assert_eq!(register.p(), 0b00110001);
        assert_eq!(cycles, 6);
    }

    #[test]
    fn test_indirect_y() {
        let sbc = Sbc::new(0xF1);
        let arguments = vec![0x77];
        let mut memory = Memory::new();
        memory.write_byte(0x77, 0x05);
        memory.write_byte(0x78, 0x01);
        memory.write_byte(0x0109, 0x07);

        let mut register = Register::new();
        register.set_accumulator(0x1F);
        register.set_y(0x04);

        let mut message_bus = MessageBus::new(&mut memory);

        let cycles = sbc.call(arguments, &mut register, &mut message_bus);

        assert_eq!(register.a(), 0x17);
        assert_eq!(register.p(), 0b00110001);
        assert_eq!(cycles, 5);
    }

    #[test]
    fn test_indirect_y_out_of_bonds() {
        let sbc = Sbc::new(0xF1);
        let arguments = vec![0x77];
        let mut memory = Memory::new();
        memory.write_byte(0x77, 0xff);
        memory.write_byte(0x78, 0x01);
        memory.write_byte(0x0205, 0x07);

        let mut register = Register::new();
        register.set_accumulator(0x1F);
        register.set_y(0x06);

        let mut message_bus = MessageBus::new(&mut memory);

        let cycles = sbc.call(arguments, &mut register, &mut message_bus);

        assert_eq!(register.a(), 0x17);
        assert_eq!(register.p(), 0b00110001);
        assert_eq!(cycles, 6);
    }

    #[test]
    #[should_panic]
    fn test_invalid_opcode() {
        let sbc = Sbc::new(0x00);
        let arguments = vec![0xFF];
        let mut memory = Memory::new();
        let mut message_bus = MessageBus::new(&mut memory);
        let mut register = Register::new();

        sbc.call(arguments, &mut register, &mut message_bus);
    }
}


