/*
LDA  Load Accumulator with Memory

     M -> A                           N Z C I D V
                                      + + - - - -

     addressing    assembler    opc  bytes  cyles
     --------------------------------------------
     immidiate     LDA #oper     A9    2     2
     zeropage      LDA oper      A5    2     3
     zeropage,X    LDA oper,X    B5    2     4
     absolute      LDA oper      AD    3     4
     absolute,X    LDA oper,X    BD    3     4*
     absolute,Y    LDA oper,Y    B9    3     4*
     (indirect,X)  LDA (oper,X)  A1    2     6
     (indirect),Y  LDA (oper),Y  B1    2     5*
*/

use crate::cpu::addressing;
use crate::cpu::mnemonics::Mnemonic;
use crate::cpu::register::Register;
use crate::message_bus::MessageBus;

#[derive(Debug)]
pub struct Lda {
    mnemonic: String,
    opcode: u8
}

impl Lda {
    pub fn new(opcode: u8) -> Lda {
        return Lda { mnemonic: "LDA".to_string(), opcode: opcode };
    }
}

impl Mnemonic for Lda {
    fn determine_bytes(&self) -> usize {
        return match self.opcode {
            0xA9 => 2,
            0xA5 => 2,
            0xB5 => 2,
            0xAD => 3,
            0xBD => 3,
            0xB9 => 3,
            0xA1 => 2,
            0xB1 => 2,
            _ => panic!("Invalid opcode `0x{:x}` for mnemonic {}", self.opcode, self.mnemonic)
        }
    }

    fn call(&self, arguments: Vec<u8>, register: &mut Register, message_bus: &mut MessageBus) -> u8 {
        match self.opcode {
            0xA9 => return self.call_immidiate(arguments, register),
            0xA5 => return self.call_zero_page(arguments, register, message_bus),
            0xB5 => return self.call_zero_page_x(arguments, register, message_bus),
            0xAD => return self.call_absolute(arguments, register, message_bus),
            0xBD => return self.call_absolute_x(arguments, register, message_bus),
            0xB9 => return self.call_absolute_y(arguments, register, message_bus),
            0xA1 => return self.call_indirect_x(arguments, register, message_bus),
            0xB1 => return self.call_indirect_y(arguments, register, message_bus),
            _ => panic!("Invalid opcode `0x{:x}` for mnemonic {}", self.opcode, self.mnemonic)
        }
    }

    fn call_immidiate(&self, arguments: Vec<u8>, register: &mut Register) -> u8 {
        register.set_accumulator(arguments[0]);
        register.calculate_nz_bits(arguments[0]);

        return 2;
    }

    fn call_zero_page(&self, arguments: Vec<u8>, register: &mut Register, message_bus: &mut MessageBus) -> u8 {
        let (_memory_address, memory_value, _boundary_crossed) = addressing::zero_page(arguments, message_bus);

        register.set_accumulator(memory_value);
        register.calculate_nz_bits(memory_value);

        return 3;
    }

    fn call_zero_page_x(&self, arguments: Vec<u8>, register: &mut Register, message_bus: &mut MessageBus) -> u8 {
        let (_memory_address, memory_value, _boundary_crossed) = addressing::zero_page_x(arguments, message_bus, register);

        register.set_accumulator(memory_value);
        register.calculate_nz_bits(memory_value);

        return 4;
    }

    fn call_absolute(&self, arguments: Vec<u8>, register: &mut Register, message_bus: &mut MessageBus) -> u8 {
        let (_memory_address, memory_value, _boundary_crossed) = addressing::absolute(arguments, message_bus);

        register.set_accumulator(memory_value);
        register.calculate_nz_bits(memory_value);

        return 4;
    }

    fn call_absolute_x(&self, arguments: Vec<u8>, register: &mut Register, message_bus: &mut MessageBus) -> u8 {
        let (_memory_address, memory_value, boundary_crossed) = addressing::absolute_x(arguments, message_bus, register);

        register.set_accumulator(memory_value);
        register.calculate_nz_bits(memory_value);

        return if boundary_crossed { 5u8 } else { 4u8 };
    }

    fn call_absolute_y(&self, arguments: Vec<u8>, register: &mut Register, message_bus: &mut MessageBus) -> u8 {
        let (_memory_address, memory_value, boundary_crossed) = addressing::absolute_y(arguments, message_bus, register);

        register.set_accumulator(memory_value);
        register.calculate_nz_bits(memory_value);

        return if boundary_crossed { 5u8 } else { 4u8 };
    }

    fn call_indirect_x(&self, arguments: Vec<u8>, register: &mut Register, message_bus: &mut MessageBus) -> u8 {
        let (_memory_address, memory_value, _boundary_crossed) = addressing::indirect_x(arguments, message_bus, register);

        register.set_accumulator(memory_value);
        register.calculate_nz_bits(memory_value);

        return 6;
    }

    fn call_indirect_y(&self, arguments: Vec<u8>, register: &mut Register, message_bus: &mut MessageBus) -> u8 {
        let (_memory_address, memory_value, boundary_crossed) = addressing::indirect_y(arguments, message_bus, register);

        register.set_accumulator(memory_value);
        register.calculate_nz_bits(memory_value);

        return if boundary_crossed { 6u8 } else { 5u8 };
    }
}

#[cfg(test)]
mod tests {
    use super::Lda;
    use crate::cpu::mnemonics::Mnemonic;
    use crate::cpu::register::Register;
    use crate::memory::Memory;
    use crate::message_bus::MessageBus;

    #[test]
    fn test_immidiate() {
        let lda = Lda::new(0xA9);
        let arguments = vec![0x42];
        let mut memory = Memory::new();
        let mut register = Register::new();
        let mut message_bus = MessageBus::new(&mut memory);

        let cycles = lda.call(arguments, &mut register, &mut message_bus);

        assert_eq!(register.a(), 0x42);
        assert_eq!(register.p(), 0b00110000);
        assert_eq!(cycles, 2);
    }

    #[test]
    fn test_immidiate_with_zero() {
        let lda = Lda::new(0xA9);
        let arguments = vec![0x00];
        let mut memory = Memory::new();
        let mut register = Register::new();
        let mut message_bus = MessageBus::new(&mut memory);

        let cycles = lda.call(arguments, &mut register, &mut message_bus);

        assert_eq!(register.a(), 0x00);
        assert_eq!(register.p(), 0b00110010);
        assert_eq!(cycles, 2);
    }

    #[test]
    fn test_immidiate_with_negative() {
        let lda = Lda::new(0xA9);
        let arguments = vec![0xF2];
        let mut memory = Memory::new();
        let mut register = Register::new();
        let mut message_bus = MessageBus::new(&mut memory);

        let cycles = lda.call(arguments, &mut register, &mut message_bus);

        assert_eq!(register.a(), 0xF2);
        assert_eq!(register.p(), 0b10110000);
        assert_eq!(cycles, 2);
    }

    #[test]
    fn test_zero_page() {
        let lda = Lda::new(0xA5);
        let arguments = vec![0x30];
        let mut memory = Memory::new();
        memory.write_byte(0x30, 0x42);

        let mut register = Register::new();
        let mut message_bus = MessageBus::new(&mut memory);

        let cycles = lda.call(arguments, &mut register, &mut message_bus);

        assert_eq!(register.a(), 0x42);
        assert_eq!(register.p(), 0b00110000);
        assert_eq!(cycles, 3);
    }

    #[test]
    fn test_zero_page_with_zero() {
        let lda = Lda::new(0xA5);
        let arguments = vec![0x30];
        let mut memory = Memory::new();
        memory.write_byte(0x30, 0x00);

        let mut register = Register::new();
        let mut message_bus = MessageBus::new(&mut memory);

        let cycles = lda.call(arguments, &mut register, &mut message_bus);

        assert_eq!(register.a(), 0x00);
        assert_eq!(register.p(), 0b00110010);
        assert_eq!(cycles, 3);
    }

    #[test]
    fn test_zero_page_with_negative() {
        let lda = Lda::new(0xA5);
        let arguments = vec![0x30];
        let mut memory = Memory::new();
        memory.write_byte(0x30, 0xF0);

        let mut register = Register::new();
        let mut message_bus = MessageBus::new(&mut memory);

        let cycles = lda.call(arguments, &mut register, &mut message_bus);

        assert_eq!(register.a(), 0xF0);
        assert_eq!(register.p(), 0b10110000);
        assert_eq!(cycles, 3);
    }

    #[test]
    fn test_zero_page_x() {
        let lda = Lda::new(0xB5);
        let arguments = vec![0x30];
        let mut memory = Memory::new();
        memory.write_byte(0x35, 0x42);

        let mut register = Register::new();
        register.set_x(0x05);

        let mut message_bus = MessageBus::new(&mut memory);

        let cycles = lda.call(arguments, &mut register, &mut message_bus);

        assert_eq!(register.a(), 0x42);
        assert_eq!(register.p(), 0b00110000);
        assert_eq!(cycles, 4);
    }

    #[test]
    fn test_zero_page_x_with_zero() {
        let lda = Lda::new(0xB5);
        let arguments = vec![0x30];
        let mut memory = Memory::new();
        memory.write_byte(0x35, 0x00);

        let mut register = Register::new();
        register.set_x(0x05);

        let mut message_bus = MessageBus::new(&mut memory);

        let cycles = lda.call(arguments, &mut register, &mut message_bus);

        assert_eq!(register.a(), 0x00);
        assert_eq!(register.p(), 0b00110010);
        assert_eq!(cycles, 4);
    }

    #[test]
    fn test_zero_page_x_with_negative() {
        let lda = Lda::new(0xB5);
        let arguments = vec![0x30];
        let mut memory = Memory::new();
        memory.write_byte(0x35, 0xF0);

        let mut register = Register::new();
        register.set_x(0x05);

        let mut message_bus = MessageBus::new(&mut memory);

        let cycles = lda.call(arguments, &mut register, &mut message_bus);

        assert_eq!(register.a(), 0xF0);
        assert_eq!(register.p(), 0b10110000);
        assert_eq!(cycles, 4);
    }

    #[test]
    fn test_absolute() {
        let lda = Lda::new(0xAD);
        let arguments = vec![0x3c, 0x5a];
        let mut memory = Memory::new();
        memory.write_byte(0x5a3c, 0x42);

        let mut register = Register::new();
        let mut message_bus = MessageBus::new(&mut memory);

        let cycles = lda.call(arguments, &mut register, &mut message_bus);

        assert_eq!(register.a(), 0x42);
        assert_eq!(register.p(), 0b00110000);
        assert_eq!(cycles, 4);
    }

    #[test]
    fn test_absolute_with_zero() {
        let lda = Lda::new(0xAD);
        let arguments = vec![0x3c, 0x5a];
        let mut memory = Memory::new();
        memory.write_byte(0x5a3c, 0x00);

        let mut register = Register::new();
        let mut message_bus = MessageBus::new(&mut memory);

        let cycles = lda.call(arguments, &mut register, &mut message_bus);

        assert_eq!(register.a(), 0x00);
        assert_eq!(register.p(), 0b00110010);
        assert_eq!(cycles, 4);
    }

    #[test]
    fn test_absolute_with_negative() {
        let lda = Lda::new(0xAD);
        let arguments = vec![0x3c, 0x5a];
        let mut memory = Memory::new();
        memory.write_byte(0x5a3c, 0xF0);

        let mut register = Register::new();
        let mut message_bus = MessageBus::new(&mut memory);

        let cycles = lda.call(arguments, &mut register, &mut message_bus);

        assert_eq!(register.a(), 0xF0);
        assert_eq!(register.p(), 0b10110000);
        assert_eq!(cycles, 4);
    }

    #[test]
    fn test_absolute_x() {
        let lda = Lda::new(0xBD);
        let arguments = vec![0x3c, 0x5a];
        let mut memory = Memory::new();
        memory.write_byte(0x5a4c, 0x42);

        let mut register = Register::new();
        register.set_x(0x10);
        register.set_y(0x20);

        let mut message_bus = MessageBus::new(&mut memory);

        let cycles = lda.call(arguments, &mut register, &mut message_bus);

        assert_eq!(register.a(), 0x42);
        assert_eq!(register.p(), 0b00110000);
        assert_eq!(cycles, 4);
    }

    #[test]
    fn test_absolute_x_with_zero() {
        let lda = Lda::new(0xBD);
        let arguments = vec![0x3c, 0x5a];
        let mut memory = Memory::new();
        memory.write_byte(0x5a4c, 0x00);

        let mut register = Register::new();
        register.set_x(0x10);
        register.set_y(0x20);

        let mut message_bus = MessageBus::new(&mut memory);

        let cycles = lda.call(arguments, &mut register, &mut message_bus);

        assert_eq!(register.a(), 0x00);
        assert_eq!(register.p(), 0b00110010);
        assert_eq!(cycles, 4);
    }

    #[test]
    fn test_absolute_x_with_negative() {
        let lda = Lda::new(0xBD);
        let arguments = vec![0x3c, 0x5a];
        let mut memory = Memory::new();
        memory.write_byte(0x5a4c, 0xF0);

        let mut register = Register::new();
        register.set_x(0x10);
        register.set_y(0x20);

        let mut message_bus = MessageBus::new(&mut memory);

        let cycles = lda.call(arguments, &mut register, &mut message_bus);

        assert_eq!(register.a(), 0xF0);
        assert_eq!(register.p(), 0b10110000);
        assert_eq!(cycles, 4);
    }

    #[test]
    fn test_absolute_x_out_of_bounds() {
        let lda = Lda::new(0xBD);
        let arguments = vec![0xff, 0xff];
        let mut memory = Memory::new();
        memory.write_byte(0x5a, 0x42);

        let mut register = Register::new();
        register.set_x(0x5b);
        register.set_y(0x20);

        let mut message_bus = MessageBus::new(&mut memory);

        let cycles = lda.call(arguments, &mut register, &mut message_bus);

        assert_eq!(register.a(), 0x42);
        assert_eq!(register.p(), 0b00110000);
        assert_eq!(cycles, 5);
    }

    #[test]
    fn test_absolute_y() {
        let lda = Lda::new(0xB9);
        let arguments = vec![0x3c, 0x5a];
        let mut memory = Memory::new();
        memory.write_byte(0x5a4c, 0x42);

        let mut register = Register::new();
        register.set_x(0x20);
        register.set_y(0x10);

        let mut message_bus = MessageBus::new(&mut memory);

        let cycles = lda.call(arguments, &mut register, &mut message_bus);

        assert_eq!(register.a(), 0x42);
        assert_eq!(register.p(), 0b00110000);
        assert_eq!(cycles, 4);
    }

    #[test]
    fn test_absolute_y_with_zero() {
        let lda = Lda::new(0xB9);
        let arguments = vec![0x3c, 0x5a];
        let mut memory = Memory::new();
        memory.write_byte(0x5a4c, 0x00);

        let mut register = Register::new();
        register.set_x(0x20);
        register.set_y(0x10);

        let mut message_bus = MessageBus::new(&mut memory);

        let cycles = lda.call(arguments, &mut register, &mut message_bus);

        assert_eq!(register.a(), 0x00);
        assert_eq!(register.p(), 0b00110010);
        assert_eq!(cycles, 4);
    }

    #[test]
    fn test_absolute_y_with_negative() {
        let lda = Lda::new(0xB9);
        let arguments = vec![0x3c, 0x5a];
        let mut memory = Memory::new();
        memory.write_byte(0x5a4c, 0xF0);

        let mut register = Register::new();
        register.set_x(0x20);
        register.set_y(0x10);

        let mut message_bus = MessageBus::new(&mut memory);

        let cycles = lda.call(arguments, &mut register, &mut message_bus);

        assert_eq!(register.a(), 0xF0);
        assert_eq!(register.p(), 0b10110000);
        assert_eq!(cycles, 4);
    }

    #[test]
    fn test_absolute_y_out_of_bounds() {
        let lda = Lda::new(0xB9);
        let arguments = vec![0xff, 0xff];
        let mut memory = Memory::new();
        memory.write_byte(0x5a, 0x42);

        let mut register = Register::new();
        register.set_x(0x20);
        register.set_y(0x5b);

        let mut message_bus = MessageBus::new(&mut memory);

        let cycles = lda.call(arguments, &mut register, &mut message_bus);

        assert_eq!(register.a(), 0x42);
        assert_eq!(register.p(), 0b00110000);
        assert_eq!(cycles, 5);
    }

    #[test]
    fn test_indirect_x() {
        let lda = Lda::new(0xA1);
        let arguments = vec![0x44];
        let mut memory = Memory::new();
        memory.write_byte(0x77, 0x05);
        memory.write_byte(0x78, 0x01);
        memory.write_byte(0x0105, 0x42);

        let mut register = Register::new();
        register.set_x(0x33);

        let mut message_bus = MessageBus::new(&mut memory);

        let cycles = lda.call(arguments, &mut register, &mut message_bus);

        assert_eq!(register.a(), 0x42);
        assert_eq!(register.p(), 0b00110000);
        assert_eq!(cycles, 6);
    }

    #[test]
    fn test_indirect_x_with_zero() {
        let lda = Lda::new(0xA1);
        let arguments = vec![0x44];
        let mut memory = Memory::new();
        memory.write_byte(0x77, 0x05);
        memory.write_byte(0x78, 0x01);
        memory.write_byte(0x0105, 0x00);

        let mut register = Register::new();
        register.set_x(0x33);

        let mut message_bus = MessageBus::new(&mut memory);

        let cycles = lda.call(arguments, &mut register, &mut message_bus);

        assert_eq!(register.a(), 0x00);
        assert_eq!(register.p(), 0b00110010);
        assert_eq!(cycles, 6);
    }

    #[test]
    fn test_indirect_x_with_negative() {
        let lda = Lda::new(0xA1);
        let arguments = vec![0x44];
        let mut memory = Memory::new();
        memory.write_byte(0x77, 0x05);
        memory.write_byte(0x78, 0x01);
        memory.write_byte(0x0105, 0xF0);

        let mut register = Register::new();
        register.set_x(0x33);

        let mut message_bus = MessageBus::new(&mut memory);

        let cycles = lda.call(arguments, &mut register, &mut message_bus);

        assert_eq!(register.a(), 0xF0);
        assert_eq!(register.p(), 0b10110000);
        assert_eq!(cycles, 6);
    }

    #[test]
    fn test_indirect_y() {
        let lda = Lda::new(0xB1);
        let arguments = vec![0xB7];
        let mut memory = Memory::new();
        memory.write_byte(0xB7, 0x05);
        memory.write_byte(0xB8, 0x01);
        memory.write_byte(0x0109, 0x42);

        let mut register = Register::new();
        register.set_y(0x04);

        let mut message_bus = MessageBus::new(&mut memory);

        let cycles = lda.call(arguments, &mut register, &mut message_bus);

        assert_eq!(register.a(), 0x42);
        assert_eq!(register.p(), 0b00110000);
        assert_eq!(cycles, 5);
    }

    #[test]
    fn test_indirect_y_with_zero() {
        let lda = Lda::new(0xB1);
        let arguments = vec![0xB7];
        let mut memory = Memory::new();
        memory.write_byte(0xB7, 0x05);
        memory.write_byte(0xB8, 0x01);
        memory.write_byte(0x0109, 0x00);

        let mut register = Register::new();
        register.set_y(0x04);

        let mut message_bus = MessageBus::new(&mut memory);

        let cycles = lda.call(arguments, &mut register, &mut message_bus);

        assert_eq!(register.a(), 0x00);
        assert_eq!(register.p(), 0b00110010);
        assert_eq!(cycles, 5);
    }

    #[test]
    fn test_indirect_y_negative() {
        let lda = Lda::new(0xB1);
        let arguments = vec![0xB7];
        let mut memory = Memory::new();
        memory.write_byte(0xB7, 0x05);
        memory.write_byte(0xB8, 0x01);
        memory.write_byte(0x0109, 0xF0);

        let mut register = Register::new();
        register.set_y(0x04);

        let mut message_bus = MessageBus::new(&mut memory);

        let cycles = lda.call(arguments, &mut register, &mut message_bus);

        assert_eq!(register.a(), 0xF0);
        assert_eq!(register.p(), 0b10110000);
        assert_eq!(cycles, 5);
    }

    #[test]
    fn test_indirect_y_out_of_bonds() {
        let lda = Lda::new(0xB1);
        let arguments = vec![0xB7];
        let mut memory = Memory::new();
        memory.write_byte(0xB7, 0xff);
        memory.write_byte(0xB8, 0x01);
        memory.write_byte(0x0205, 0x42);

        let mut register = Register::new();
        register.set_y(0x06);

        let mut message_bus = MessageBus::new(&mut memory);

        let cycles = lda.call(arguments, &mut register, &mut message_bus);

        assert_eq!(register.a(), 0x42);
        assert_eq!(register.p(), 0b00110000);
        assert_eq!(cycles, 6);
    }

    #[test]
    #[should_panic]
    fn test_invalid_opcode() {
        let lda = Lda::new(0x00);
        let arguments = vec![0xFF];
        let mut memory = Memory::new();
        let mut message_bus = MessageBus::new(&mut memory);
        let mut register = Register::new();

        lda.call(arguments, &mut register, &mut message_bus);
    }
}


