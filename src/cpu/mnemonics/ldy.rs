/*
LDY  Load Index Y with Memory

     M -> Y                           N Z C I D V
                                      + + - - - -

     addressing    assembler    opc  bytes  cyles
     --------------------------------------------
     immidiate     LDY #oper     A0    2     2
     zeropage      LDY oper      A4    2     3
     zeropage,X    LDY oper,X    B4    2     4
     absolute      LDY oper      AC    3     4
     absolute,X    LDY oper,X    BC    3     4*
*/

use crate::cpu::addressing;
use crate::cpu::mnemonics::Mnemonic;
use crate::cpu::register::Register;
use crate::message_bus::MessageBus;

#[derive(Debug)]
pub struct Ldy {
    mnemonic: String,
    opcode: u8
}

impl Ldy {
    pub fn new(opcode: u8) -> Ldy {
        return Ldy { mnemonic: "LDY".to_string(), opcode: opcode };
    }
}

impl Mnemonic for Ldy {
    fn determine_bytes(&self) -> usize {
        return match self.opcode {
            0xA0 => 2,
            0xA4 => 2,
            0xB4 => 2,
            0xAC => 3,
            0xBC => 3,
            _ => panic!("Invalid opcode `0x{:x}` for mnemonic {}", self.opcode, self.mnemonic)
        }
    }

    fn call(&self, arguments: Vec<u8>, register: &mut Register, message_bus: &mut MessageBus) -> u8 {
        match self.opcode {
            0xA0 => return self.call_immidiate(arguments, register),
            0xA4 => return self.call_zero_page(arguments, register, message_bus),
            0xB4 => return self.call_zero_page_x(arguments, register, message_bus),
            0xAC => return self.call_absolute(arguments, register, message_bus),
            0xBC => return self.call_absolute_x(arguments, register, message_bus),
            _ => panic!("Invalid opcode `0x{:x}` for mnemonic {}", self.opcode, self.mnemonic)
        }
    }

    fn call_immidiate(&self, arguments: Vec<u8>, register: &mut Register) -> u8 {
        register.set_y(arguments[0]);
        register.calculate_nz_bits(arguments[0]);

        return 2;
    }

    fn call_zero_page(&self, arguments: Vec<u8>, register: &mut Register, message_bus: &mut MessageBus) -> u8 {
        let (_memory_address, memory_value, _boundary_crossed) = addressing::zero_page(arguments, message_bus);

        register.set_y(memory_value);
        register.calculate_nz_bits(memory_value);

        return 3;
    }

    fn call_zero_page_x(&self, arguments: Vec<u8>, register: &mut Register, message_bus: &mut MessageBus) -> u8 {
        let (_memory_address, memory_value, _boundary_crossed) = addressing::zero_page_x(arguments, message_bus, register);

        register.set_y(memory_value);
        register.calculate_nz_bits(memory_value);

        return 4;
    }

    fn call_absolute(&self, arguments: Vec<u8>, register: &mut Register, message_bus: &mut MessageBus) -> u8 {
        let (_memory_address, memory_value, _boundary_crossed) = addressing::absolute(arguments, message_bus);

        register.set_y(memory_value);
        register.calculate_nz_bits(memory_value);

        return 4;
    }

    fn call_absolute_x(&self, arguments: Vec<u8>, register: &mut Register, message_bus: &mut MessageBus) -> u8 {
        let (_memory_address, memory_value, boundary_crossed) = addressing::absolute_x(arguments, message_bus, register);

        register.set_y(memory_value);
        register.calculate_nz_bits(memory_value);

        return if boundary_crossed { 5u8 } else { 4u8 };
    }
}

#[cfg(test)]
mod tests {
    use super::Ldy;
    use crate::cpu::mnemonics::Mnemonic;
    use crate::cpu::register::Register;
    use crate::memory::Memory;
    use crate::message_bus::MessageBus;

    #[test]
    fn test_immidiate() {
        let ldy = Ldy::new(0xA0);
        let arguments = vec![0x42];
        let mut memory = Memory::new();
        let mut register = Register::new();
        let mut message_bus = MessageBus::new(&mut memory);

        let cycles = ldy.call(arguments, &mut register, &mut message_bus);

        assert_eq!(register.y(), 0x42);
        assert_eq!(register.p(), 0b00110000);
        assert_eq!(cycles, 2);
    }

    #[test]
    fn test_immidiate_with_zero() {
        let ldy = Ldy::new(0xA0);
        let arguments = vec![0x00];
        let mut memory = Memory::new();
        let mut register = Register::new();
        let mut message_bus = MessageBus::new(&mut memory);

        let cycles = ldy.call(arguments, &mut register, &mut message_bus);

        assert_eq!(register.y(), 0x00);
        assert_eq!(register.p(), 0b00110010);
        assert_eq!(cycles, 2);
    }

    #[test]
    fn test_immidiate_with_negative() {
        let ldy = Ldy::new(0xA0);
        let arguments = vec![0xF2];
        let mut memory = Memory::new();
        let mut register = Register::new();
        let mut message_bus = MessageBus::new(&mut memory);

        let cycles = ldy.call(arguments, &mut register, &mut message_bus);

        assert_eq!(register.y(), 0xF2);
        assert_eq!(register.p(), 0b10110000);
        assert_eq!(cycles, 2);
    }

    #[test]
    fn test_zero_page() {
        let ldy = Ldy::new(0xA4);
        let arguments = vec![0x30];
        let mut memory = Memory::new();
        memory.write_byte(0x30, 0x42);

        let mut register = Register::new();
        let mut message_bus = MessageBus::new(&mut memory);

        let cycles = ldy.call(arguments, &mut register, &mut message_bus);

        assert_eq!(register.y(), 0x42);
        assert_eq!(register.p(), 0b00110000);
        assert_eq!(cycles, 3);
    }

    #[test]
    fn test_zero_page_with_zero() {
        let ldy = Ldy::new(0xA4);
        let arguments = vec![0x30];
        let mut memory = Memory::new();
        memory.write_byte(0x30, 0x00);

        let mut register = Register::new();
        let mut message_bus = MessageBus::new(&mut memory);

        let cycles = ldy.call(arguments, &mut register, &mut message_bus);

        assert_eq!(register.y(), 0x00);
        assert_eq!(register.p(), 0b00110010);
        assert_eq!(cycles, 3);
    }

    #[test]
    fn test_zero_page_with_negative() {
        let ldy = Ldy::new(0xA4);
        let arguments = vec![0x30];
        let mut memory = Memory::new();
        memory.write_byte(0x30, 0xF0);

        let mut register = Register::new();
        let mut message_bus = MessageBus::new(&mut memory);

        let cycles = ldy.call(arguments, &mut register, &mut message_bus);

        assert_eq!(register.y(), 0xF0);
        assert_eq!(register.p(), 0b10110000);
        assert_eq!(cycles, 3);
    }

    #[test]
    fn test_zero_page_x() {
        let ldy = Ldy::new(0xB4);
        let arguments = vec![0x30];
        let mut memory = Memory::new();
        memory.write_byte(0x35, 0x42);

        let mut register = Register::new();
        register.set_x(0x05);

        let mut message_bus = MessageBus::new(&mut memory);

        let cycles = ldy.call(arguments, &mut register, &mut message_bus);

        assert_eq!(register.y(), 0x42);
        assert_eq!(register.p(), 0b00110000);
        assert_eq!(cycles, 4);
    }

    #[test]
    fn test_zero_page_x_with_zero() {
        let ldy = Ldy::new(0xB4);
        let arguments = vec![0x30];
        let mut memory = Memory::new();
        memory.write_byte(0x35, 0x00);

        let mut register = Register::new();
        register.set_x(0x05);

        let mut message_bus = MessageBus::new(&mut memory);

        let cycles = ldy.call(arguments, &mut register, &mut message_bus);

        assert_eq!(register.y(), 0x00);
        assert_eq!(register.p(), 0b00110010);
        assert_eq!(cycles, 4);
    }

    #[test]
    fn test_zero_page_x_with_negative() {
        let ldy = Ldy::new(0xB4);
        let arguments = vec![0x30];
        let mut memory = Memory::new();
        memory.write_byte(0x35, 0xF0);

        let mut register = Register::new();
        register.set_x(0x05);

        let mut message_bus = MessageBus::new(&mut memory);

        let cycles = ldy.call(arguments, &mut register, &mut message_bus);

        assert_eq!(register.y(), 0xF0);
        assert_eq!(register.p(), 0b10110000);
        assert_eq!(cycles, 4);
    }

    #[test]
    fn test_absolute() {
        let ldy = Ldy::new(0xAC);
        let arguments = vec![0x3c, 0x5a];
        let mut memory = Memory::new();
        memory.write_byte(0x5a3c, 0x42);

        let mut register = Register::new();
        let mut message_bus = MessageBus::new(&mut memory);

        let cycles = ldy.call(arguments, &mut register, &mut message_bus);

        assert_eq!(register.y(), 0x42);
        assert_eq!(register.p(), 0b00110000);
        assert_eq!(cycles, 4);
    }

    #[test]
    fn test_absolute_with_zero() {
        let ldy = Ldy::new(0xAC);
        let arguments = vec![0x3c, 0x5a];
        let mut memory = Memory::new();
        memory.write_byte(0x5a3c, 0x00);

        let mut register = Register::new();
        let mut message_bus = MessageBus::new(&mut memory);

        let cycles = ldy.call(arguments, &mut register, &mut message_bus);

        assert_eq!(register.y(), 0x00);
        assert_eq!(register.p(), 0b00110010);
        assert_eq!(cycles, 4);
    }

    #[test]
    fn test_absolute_with_negative() {
        let ldy = Ldy::new(0xAC);
        let arguments = vec![0x3c, 0x5a];
        let mut memory = Memory::new();
        memory.write_byte(0x5a3c, 0xF0);

        let mut register = Register::new();
        let mut message_bus = MessageBus::new(&mut memory);

        let cycles = ldy.call(arguments, &mut register, &mut message_bus);

        assert_eq!(register.y(), 0xF0);
        assert_eq!(register.p(), 0b10110000);
        assert_eq!(cycles, 4);
    }

    #[test]
    fn test_absolute_x() {
        let ldy = Ldy::new(0xBC);
        let arguments = vec![0x3c, 0x5a];
        let mut memory = Memory::new();
        memory.write_byte(0x5a4c, 0x42);

        let mut register = Register::new();
        register.set_x(0x10);
        register.set_y(0x20);

        let mut message_bus = MessageBus::new(&mut memory);

        let cycles = ldy.call(arguments, &mut register, &mut message_bus);

        assert_eq!(register.y(), 0x42);
        assert_eq!(register.p(), 0b00110000);
        assert_eq!(cycles, 4);
    }

    #[test]
    fn test_absolute_x_with_zero() {
        let ldy = Ldy::new(0xBC);
        let arguments = vec![0x3c, 0x5a];
        let mut memory = Memory::new();
        memory.write_byte(0x5a4c, 0x00);

        let mut register = Register::new();
        register.set_x(0x10);
        register.set_y(0x20);

        let mut message_bus = MessageBus::new(&mut memory);

        let cycles = ldy.call(arguments, &mut register, &mut message_bus);

        assert_eq!(register.y(), 0x00);
        assert_eq!(register.p(), 0b00110010);
        assert_eq!(cycles, 4);
    }

    #[test]
    fn test_absolute_x_with_negative() {
        let ldy = Ldy::new(0xBC);
        let arguments = vec![0x3c, 0x5a];
        let mut memory = Memory::new();
        memory.write_byte(0x5a4c, 0xF0);

        let mut register = Register::new();
        register.set_x(0x10);
        register.set_y(0x20);

        let mut message_bus = MessageBus::new(&mut memory);

        let cycles = ldy.call(arguments, &mut register, &mut message_bus);

        assert_eq!(register.y(), 0xF0);
        assert_eq!(register.p(), 0b10110000);
        assert_eq!(cycles, 4);
    }

    #[test]
    fn test_absolute_x_out_of_bounds() {
        let ldy = Ldy::new(0xBC);
        let arguments = vec![0xff, 0xff];
        let mut memory = Memory::new();
        memory.write_byte(0x5a, 0x42);

        let mut register = Register::new();
        register.set_x(0x5b);
        register.set_y(0x20);

        let mut message_bus = MessageBus::new(&mut memory);

        let cycles = ldy.call(arguments, &mut register, &mut message_bus);

        assert_eq!(register.y(), 0x42);
        assert_eq!(register.p(), 0b00110000);
        assert_eq!(cycles, 5);
    }

    #[test]
    #[should_panic]
    fn test_invalid_opcode() {
        let ldy = Ldy::new(0x00);
        let arguments = vec![0xFF];
        let mut memory = Memory::new();
        let mut message_bus = MessageBus::new(&mut memory);
        let mut register = Register::new();

        ldy.call(arguments, &mut register, &mut message_bus);
    }
}


