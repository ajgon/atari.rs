/*
LDX  Load Index X with Memory

     M -> X                           N Z C I D V
                                      + + - - - -

     addressing    assembler    opc  bytes  cyles
     --------------------------------------------
     immidiate     LDX #oper     A2    2     2
     zeropage      LDX oper      A6    2     3
     zeropage,Y    LDX oper,Y    B6    2     4
     absolute      LDX oper      AE    3     4
     absolute,Y    LDX oper,Y    BE    3     4*
*/

use crate::cpu::addressing;
use crate::cpu::mnemonics::Mnemonic;
use crate::cpu::register::Register;
use crate::message_bus::MessageBus;

#[derive(Debug)]
pub struct Ldx {
    mnemonic: String,
    opcode: u8
}

impl Ldx {
    pub fn new(opcode: u8) -> Ldx {
        return Ldx { mnemonic: "LDX".to_string(), opcode: opcode };
    }
}

impl Mnemonic for Ldx {
    fn determine_bytes(&self) -> usize {
        return match self.opcode {
            0xA2 => 2,
            0xA6 => 2,
            0xB6 => 2,
            0xAE => 3,
            0xBE => 3,
            _ => panic!("Invalid opcode `0x{:x}` for mnemonic {}", self.opcode, self.mnemonic)
        }
    }

    fn call(&self, arguments: Vec<u8>, register: &mut Register, message_bus: &mut MessageBus) -> u8 {
        match self.opcode {
            0xA2 => return self.call_immidiate(arguments, register),
            0xA6 => return self.call_zero_page(arguments, register, message_bus),
            0xB6 => return self.call_zero_page_y(arguments, register, message_bus),
            0xAE => return self.call_absolute(arguments, register, message_bus),
            0xBE => return self.call_absolute_y(arguments, register, message_bus),
            _ => panic!("Invalid opcode `0x{:x}` for mnemonic {}", self.opcode, self.mnemonic)
        }
    }

    fn call_immidiate(&self, arguments: Vec<u8>, register: &mut Register) -> u8 {
        register.set_x(arguments[0]);
        register.calculate_nz_bits(arguments[0]);

        return 2;
    }

    fn call_zero_page(&self, arguments: Vec<u8>, register: &mut Register, message_bus: &mut MessageBus) -> u8 {
        let (_memory_address, memory_value, _boundary_crossed) = addressing::zero_page(arguments, message_bus);

        register.set_x(memory_value);
        register.calculate_nz_bits(memory_value);

        return 3;
    }

    fn call_zero_page_y(&self, arguments: Vec<u8>, register: &mut Register, message_bus: &mut MessageBus) -> u8 {
        let (_memory_address, memory_value, _boundary_crossed) = addressing::zero_page_y(arguments, message_bus, register);

        register.set_x(memory_value);
        register.calculate_nz_bits(memory_value);

        return 4;
    }

    fn call_absolute(&self, arguments: Vec<u8>, register: &mut Register, message_bus: &mut MessageBus) -> u8 {
        let (_memory_address, memory_value, _boundary_crossed) = addressing::absolute(arguments, message_bus);

        register.set_x(memory_value);
        register.calculate_nz_bits(memory_value);

        return 4;
    }

    fn call_absolute_y(&self, arguments: Vec<u8>, register: &mut Register, message_bus: &mut MessageBus) -> u8 {
        let (_memory_address, memory_value, boundary_crossed) = addressing::absolute_y(arguments, message_bus, register);

        register.set_x(memory_value);
        register.calculate_nz_bits(memory_value);

        return if boundary_crossed { 5u8 } else { 4u8 };
    }
}

#[cfg(test)]
mod tests {
    use super::Ldx;
    use crate::cpu::mnemonics::Mnemonic;
    use crate::cpu::register::Register;
    use crate::memory::Memory;
    use crate::message_bus::MessageBus;

    #[test]
    fn test_immidiate() {
        let ldx = Ldx::new(0xA2);
        let arguments = vec![0x42];
        let mut memory = Memory::new();
        let mut register = Register::new();
        let mut message_bus = MessageBus::new(&mut memory);

        let cycles = ldx.call(arguments, &mut register, &mut message_bus);

        assert_eq!(register.x(), 0x42);
        assert_eq!(register.p(), 0b00110000);
        assert_eq!(cycles, 2);
    }

    #[test]
    fn test_immidiate_with_zero() {
        let ldx = Ldx::new(0xA2);
        let arguments = vec![0x00];
        let mut memory = Memory::new();
        let mut register = Register::new();
        let mut message_bus = MessageBus::new(&mut memory);

        let cycles = ldx.call(arguments, &mut register, &mut message_bus);

        assert_eq!(register.x(), 0x00);
        assert_eq!(register.p(), 0b00110010);
        assert_eq!(cycles, 2);
    }

    #[test]
    fn test_immidiate_with_negative() {
        let ldx = Ldx::new(0xA2);
        let arguments = vec![0xF2];
        let mut memory = Memory::new();
        let mut register = Register::new();
        let mut message_bus = MessageBus::new(&mut memory);

        let cycles = ldx.call(arguments, &mut register, &mut message_bus);

        assert_eq!(register.x(), 0xF2);
        assert_eq!(register.p(), 0b10110000);
        assert_eq!(cycles, 2);
    }

    #[test]
    fn test_zero_page() {
        let ldx = Ldx::new(0xA6);
        let arguments = vec![0x30];
        let mut memory = Memory::new();
        memory.write_byte(0x30, 0x42);

        let mut register = Register::new();
        let mut message_bus = MessageBus::new(&mut memory);

        let cycles = ldx.call(arguments, &mut register, &mut message_bus);

        assert_eq!(register.x(), 0x42);
        assert_eq!(register.p(), 0b00110000);
        assert_eq!(cycles, 3);
    }

    #[test]
    fn test_zero_page_with_zero() {
        let ldx = Ldx::new(0xA6);
        let arguments = vec![0x30];
        let mut memory = Memory::new();
        memory.write_byte(0x30, 0x00);

        let mut register = Register::new();
        let mut message_bus = MessageBus::new(&mut memory);

        let cycles = ldx.call(arguments, &mut register, &mut message_bus);

        assert_eq!(register.x(), 0x00);
        assert_eq!(register.p(), 0b00110010);
        assert_eq!(cycles, 3);
    }

    #[test]
    fn test_zero_page_with_negative() {
        let ldx = Ldx::new(0xA6);
        let arguments = vec![0x30];
        let mut memory = Memory::new();
        memory.write_byte(0x30, 0xF0);

        let mut register = Register::new();
        let mut message_bus = MessageBus::new(&mut memory);

        let cycles = ldx.call(arguments, &mut register, &mut message_bus);

        assert_eq!(register.x(), 0xF0);
        assert_eq!(register.p(), 0b10110000);
        assert_eq!(cycles, 3);
    }

    #[test]
    fn test_zero_page_y() {
        let ldx = Ldx::new(0xB6);
        let arguments = vec![0x30];
        let mut memory = Memory::new();
        memory.write_byte(0x35, 0x42);

        let mut register = Register::new();
        register.set_y(0x05);

        let mut message_bus = MessageBus::new(&mut memory);

        let cycles = ldx.call(arguments, &mut register, &mut message_bus);

        assert_eq!(register.x(), 0x42);
        assert_eq!(register.p(), 0b00110000);
        assert_eq!(cycles, 4);
    }

    #[test]
    fn test_zero_page_y_with_zero() {
        let ldx = Ldx::new(0xB6);
        let arguments = vec![0x30];
        let mut memory = Memory::new();
        memory.write_byte(0x35, 0x00);

        let mut register = Register::new();
        register.set_y(0x05);

        let mut message_bus = MessageBus::new(&mut memory);

        let cycles = ldx.call(arguments, &mut register, &mut message_bus);

        assert_eq!(register.x(), 0x00);
        assert_eq!(register.p(), 0b00110010);
        assert_eq!(cycles, 4);
    }

    #[test]
    fn test_zero_page_y_with_negative() {
        let ldx = Ldx::new(0xB6);
        let arguments = vec![0x30];
        let mut memory = Memory::new();
        memory.write_byte(0x35, 0xF0);

        let mut register = Register::new();
        register.set_y(0x05);

        let mut message_bus = MessageBus::new(&mut memory);

        let cycles = ldx.call(arguments, &mut register, &mut message_bus);

        assert_eq!(register.x(), 0xF0);
        assert_eq!(register.p(), 0b10110000);
        assert_eq!(cycles, 4);
    }

    #[test]
    fn test_absolute() {
        let ldx = Ldx::new(0xAE);
        let arguments = vec![0x3c, 0x5a];
        let mut memory = Memory::new();
        memory.write_byte(0x5a3c, 0x42);

        let mut register = Register::new();
        let mut message_bus = MessageBus::new(&mut memory);

        let cycles = ldx.call(arguments, &mut register, &mut message_bus);

        assert_eq!(register.x(), 0x42);
        assert_eq!(register.p(), 0b00110000);
        assert_eq!(cycles, 4);
    }

    #[test]
    fn test_absolute_with_zero() {
        let ldx = Ldx::new(0xAE);
        let arguments = vec![0x3c, 0x5a];
        let mut memory = Memory::new();
        memory.write_byte(0x5a3c, 0x00);

        let mut register = Register::new();
        let mut message_bus = MessageBus::new(&mut memory);

        let cycles = ldx.call(arguments, &mut register, &mut message_bus);

        assert_eq!(register.x(), 0x00);
        assert_eq!(register.p(), 0b00110010);
        assert_eq!(cycles, 4);
    }

    #[test]
    fn test_absolute_with_negative() {
        let ldx = Ldx::new(0xAE);
        let arguments = vec![0x3c, 0x5a];
        let mut memory = Memory::new();
        memory.write_byte(0x5a3c, 0xF0);

        let mut register = Register::new();
        let mut message_bus = MessageBus::new(&mut memory);

        let cycles = ldx.call(arguments, &mut register, &mut message_bus);

        assert_eq!(register.x(), 0xF0);
        assert_eq!(register.p(), 0b10110000);
        assert_eq!(cycles, 4);
    }

    #[test]
    fn test_absolute_y() {
        let ldx = Ldx::new(0xBE);
        let arguments = vec![0x3c, 0x5a];
        let mut memory = Memory::new();
        memory.write_byte(0x5a4c, 0x42);

        let mut register = Register::new();
        register.set_x(0x20);
        register.set_y(0x10);

        let mut message_bus = MessageBus::new(&mut memory);

        let cycles = ldx.call(arguments, &mut register, &mut message_bus);

        assert_eq!(register.x(), 0x42);
        assert_eq!(register.p(), 0b00110000);
        assert_eq!(cycles, 4);
    }

    #[test]
    fn test_absolute_y_with_zero() {
        let ldx = Ldx::new(0xBE);
        let arguments = vec![0x3c, 0x5a];
        let mut memory = Memory::new();
        memory.write_byte(0x5a4c, 0x00);

        let mut register = Register::new();
        register.set_x(0x20);
        register.set_y(0x10);

        let mut message_bus = MessageBus::new(&mut memory);

        let cycles = ldx.call(arguments, &mut register, &mut message_bus);

        assert_eq!(register.x(), 0x00);
        assert_eq!(register.p(), 0b00110010);
        assert_eq!(cycles, 4);
    }

    #[test]
    fn test_absolute_y_with_negative() {
        let ldx = Ldx::new(0xBE);
        let arguments = vec![0x3c, 0x5a];
        let mut memory = Memory::new();
        memory.write_byte(0x5a4c, 0xF0);

        let mut register = Register::new();
        register.set_x(0x20);
        register.set_y(0x10);

        let mut message_bus = MessageBus::new(&mut memory);

        let cycles = ldx.call(arguments, &mut register, &mut message_bus);

        assert_eq!(register.x(), 0xF0);
        assert_eq!(register.p(), 0b10110000);
        assert_eq!(cycles, 4);
    }

    #[test]
    fn test_absolute_y_out_of_bounds() {
        let ldx = Ldx::new(0xBE);
        let arguments = vec![0xff, 0xff];
        let mut memory = Memory::new();
        memory.write_byte(0x5a, 0x42);

        let mut register = Register::new();
        register.set_x(0x20);
        register.set_y(0x5b);

        let mut message_bus = MessageBus::new(&mut memory);

        let cycles = ldx.call(arguments, &mut register, &mut message_bus);

        assert_eq!(register.x(), 0x42);
        assert_eq!(register.p(), 0b00110000);
        assert_eq!(cycles, 5);
    }

    #[test]
    #[should_panic]
    fn test_invalid_opcode() {
        let ldx = Ldx::new(0x00);
        let arguments = vec![0xFF];
        let mut memory = Memory::new();
        let mut message_bus = MessageBus::new(&mut memory);
        let mut register = Register::new();

        ldx.call(arguments, &mut register, &mut message_bus);
    }
}


