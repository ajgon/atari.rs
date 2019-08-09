/*
ROL  Rotate One Bit Left (Memory or Accumulator)

     C <- [76543210] <- C             N Z C I D V
                                      + + + - - -

     addressing    assembler    opc  bytes  cyles
     --------------------------------------------
     accumulator   ROL A         2A    1     2
     zeropage      ROL oper      26    2     5
     zeropage,X    ROL oper,X    36    2     6
     absolute      ROL oper      2E    3     6
     absolute,X    ROL oper,X    3E    3     7
*/

use crate::cpu::addressing;
use crate::cpu::mnemonics::Mnemonic;
use crate::cpu::register::Register;
use crate::message_bus::MessageBus;
use crate::message_bus::MessageBusTarget;
use crate::message_bus::MessageBusMessage;
use crate::cpu::alu;

#[derive(Debug)]
pub struct Rol {
    mnemonic: String,
    opcode: u8
}

impl Rol {
    pub fn new(opcode: u8) -> Rol {
        return Rol { mnemonic: "ROL".to_string(), opcode: opcode };
    }
}

impl Mnemonic for Rol {
    fn determine_bytes(&self) -> usize {
        return match self.opcode {
            0x2A => 1,
            0x26 => 2,
            0x36 => 2,
            0x2E => 3,
            0x3E => 3,
            _ => panic!("Invalid opcode `0x{:x}` for mnemonic {}", self.opcode, self.mnemonic)
        }
    }

    fn call(&self, arguments: Vec<u8>, register: &mut Register, message_bus: &mut MessageBus) -> u8 {
        match self.opcode {
            0x2A => return self.call_accumulator(register),
            0x26 => return self.call_zero_page(arguments, register, message_bus),
            0x36 => return self.call_zero_page_x(arguments, register, message_bus),
            0x2E => return self.call_absolute(arguments, register, message_bus),
            0x3E => return self.call_absolute_x(arguments, register, message_bus),
            _ => panic!("Invalid opcode `0x{:x}` for mnemonic {}", self.opcode, self.mnemonic)
        }
    }

    fn call_accumulator(&self, register: &mut Register) -> u8 {
        let previous_carry_bit = register.carry_bit();

        let result = alu::shift_left(register.a(), register);
        let result = if previous_carry_bit { result | 0x01 } else { result & 0xFE };
        register.set_accumulator(result);

        return 2;
    }

    fn call_zero_page(&self, arguments: Vec<u8>, register: &mut Register, message_bus: &mut MessageBus) -> u8 {
        let (memory_address, memory_value, _boundary_crossed) = addressing::zero_page(arguments, message_bus);
        let previous_carry_bit = register.carry_bit();

        let result = alu::shift_left(memory_value, register);
        let result = if previous_carry_bit { result | 0x01 } else { result & 0xFE };
        message_bus.send_message(
            MessageBusTarget::Memory, MessageBusMessage::Write, vec![memory_address, result as u16]
        );

        return 5;
    }

    fn call_zero_page_x(&self, arguments: Vec<u8>, register: &mut Register, message_bus: &mut MessageBus) -> u8 {
        let (memory_address, memory_value, _boundary_crossed) = addressing::zero_page_x(arguments, message_bus, register);
        let previous_carry_bit = register.carry_bit();

        let result = alu::shift_left(memory_value, register);
        let result = if previous_carry_bit { result | 0x01 } else { result & 0xFE };
        message_bus.send_message(
            MessageBusTarget::Memory, MessageBusMessage::Write, vec![memory_address, result as u16]
        );

        return 6;
    }

    fn call_absolute(&self, arguments: Vec<u8>, register: &mut Register, message_bus: &mut MessageBus) -> u8 {
        let (memory_address, memory_value, _boundary_crossed) = addressing::absolute(arguments, message_bus);
        let previous_carry_bit = register.carry_bit();

        let result = alu::shift_left(memory_value, register);
        let result = if previous_carry_bit { result | 0x01 } else { result & 0xFE };
        message_bus.send_message(
            MessageBusTarget::Memory, MessageBusMessage::Write, vec![memory_address, result as u16]
        );

        return 6;
    }

    fn call_absolute_x(&self, arguments: Vec<u8>, register: &mut Register, message_bus: &mut MessageBus) -> u8 {
        let (memory_address, memory_value, _boundary_crossed) = addressing::absolute_x(arguments, message_bus, register);
        let previous_carry_bit = register.carry_bit();

        let result = alu::shift_left(memory_value, register);
        let result = if previous_carry_bit { result | 0x01 } else { result & 0xFE };
        message_bus.send_message(
            MessageBusTarget::Memory, MessageBusMessage::Write, vec![memory_address, result as u16]
        );

        return 7;
    }
}

#[cfg(test)]
mod tests {
    use super::Rol;
    use crate::cpu::mnemonics::Mnemonic;
    use crate::cpu::register::Register;
    use crate::memory::Memory;
    use crate::message_bus::MessageBus;

    #[test]
    fn test_accumulator() {
        let rol = Rol::new(0x2A);
        let arguments = vec![0x00];
        let mut memory = Memory::new();
        let mut register = Register::new();
        register.set_accumulator(0b0010_1110);

        let mut message_bus = MessageBus::new(&mut memory);

        let cycles = rol.call(arguments, &mut register, &mut message_bus);

        assert_eq!(register.a(), 0b0101_1100);
        assert_eq!(register.p(), 0b0011_0000);
        assert_eq!(cycles, 2);
    }

    #[test]
    fn test_accumulator_with_carry_set_before() {
        let rol = Rol::new(0x2A);
        let arguments = vec![0x00];
        let mut memory = Memory::new();
        let mut register = Register::new();
        register.set_accumulator(0b0010_1110);
        register.set_carry_bit(true);

        let mut message_bus = MessageBus::new(&mut memory);

        let cycles = rol.call(arguments, &mut register, &mut message_bus);

        assert_eq!(register.a(), 0b0101_1101);
        assert_eq!(register.p(), 0b0011_0000);
        assert_eq!(cycles, 2);
    }

    #[test]
    fn test_accumulator_with_carry_set_after() {
        let rol = Rol::new(0x2A);
        let arguments = vec![0x00];
        let mut memory = Memory::new();
        let mut register = Register::new();
        register.set_accumulator(0b1010_1110);

        let mut message_bus = MessageBus::new(&mut memory);

        let cycles = rol.call(arguments, &mut register, &mut message_bus);

        assert_eq!(register.a(), 0b0101_1100);
        assert_eq!(register.p(), 0b0011_0001);
        assert_eq!(cycles, 2);
    }

    #[test]
    fn test_accumulator_with_zero() {
        let rol = Rol::new(0x2A);
        let arguments = vec![0x00];
        let mut memory = Memory::new();
        let mut register = Register::new();
        let mut message_bus = MessageBus::new(&mut memory);

        let cycles = rol.call(arguments, &mut register, &mut message_bus);

        assert_eq!(register.a(), 0x00);
        assert_eq!(register.p(), 0b0011_0010);
        assert_eq!(cycles, 2);
    }

    #[test]
    fn test_accumulator_with_negative() {
        let rol = Rol::new(0x2A);
        let arguments = vec![0xF2];
        let mut memory = Memory::new();
        let mut register = Register::new();
        register.set_accumulator(0b0100_0001);

        let mut message_bus = MessageBus::new(&mut memory);

        let cycles = rol.call(arguments, &mut register, &mut message_bus);

        assert_eq!(register.a(), 0b1000_0010);
        assert_eq!(register.p(), 0b1011_0000);
        assert_eq!(cycles, 2);
    }

    #[test]
    fn test_zero_page() {
        let rol = Rol::new(0x26);
        let arguments = vec![0x35];
        let mut memory = Memory::new();
        memory.write_byte(0x35, 0b0010_1110);

        let mut register = Register::new();

        let mut message_bus = MessageBus::new(&mut memory);

        let cycles = rol.call(arguments, &mut register, &mut message_bus);

        assert_eq!(memory.read_byte(0x35), 0b0101_1100);
        assert_eq!(register.p(), 0b0011_0000);
        assert_eq!(cycles, 5);
    }

    #[test]
    fn test_zero_page_with_carry_set_before() {
        let rol = Rol::new(0x26);
        let arguments = vec![0x35];
        let mut memory = Memory::new();
        memory.write_byte(0x35, 0b0010_1110);

        let mut register = Register::new();
        register.set_carry_bit(true);

        let mut message_bus = MessageBus::new(&mut memory);

        let cycles = rol.call(arguments, &mut register, &mut message_bus);

        assert_eq!(memory.read_byte(0x35), 0b0101_1101);
        assert_eq!(register.p(), 0b0011_0000);
        assert_eq!(cycles, 5);
    }

    #[test]
    fn test_zero_page_with_carry_set_after() {
        let rol = Rol::new(0x26);
        let arguments = vec![0x35];
        let mut memory = Memory::new();
        memory.write_byte(0x35, 0b1010_1110);

        let mut register = Register::new();
        let mut message_bus = MessageBus::new(&mut memory);

        let cycles = rol.call(arguments, &mut register, &mut message_bus);

        assert_eq!(memory.read_byte(0x35), 0b0101_1100);
        assert_eq!(register.p(), 0b0011_0001);
        assert_eq!(cycles, 5);
    }

    #[test]
    fn test_zero_page_with_zero() {
        let rol = Rol::new(0x26);
        let arguments = vec![0x35];
        let mut memory = Memory::new();
        let mut register = Register::new();
        let mut message_bus = MessageBus::new(&mut memory);

        let cycles = rol.call(arguments, &mut register, &mut message_bus);

        assert_eq!(memory.read_byte(0x35), 0x00);
        assert_eq!(register.p(), 0b0011_0010);
        assert_eq!(cycles, 5);
    }

    #[test]
    fn test_zero_page_with_negative() {
        let rol = Rol::new(0x26);
        let arguments = vec![0x35];
        let mut memory = Memory::new();
        memory.write_byte(0x35, 0b0100_0001);

        let mut register = Register::new();
        let mut message_bus = MessageBus::new(&mut memory);

        let cycles = rol.call(arguments, &mut register, &mut message_bus);
        assert_eq!(memory.read_byte(0x35), 0b1000_0010);
        assert_eq!(register.p(), 0b1011_0000);
        assert_eq!(cycles, 5);
    }

    #[test]
    fn test_zero_page_x() {
        let rol = Rol::new(0x36);
        let arguments = vec![0x32];
        let mut memory = Memory::new();
        memory.write_byte(0x35, 0b0010_1110);

        let mut register = Register::new();
        register.set_x(0x03);

        let mut message_bus = MessageBus::new(&mut memory);

        let cycles = rol.call(arguments, &mut register, &mut message_bus);

        assert_eq!(memory.read_byte(0x35), 0b0101_1100);
        assert_eq!(register.p(), 0b0011_0000);
        assert_eq!(cycles, 6);
    }

    #[test]
    fn test_zero_page_x_with_carry_set_before() {
        let rol = Rol::new(0x36);
        let arguments = vec![0x32];
        let mut memory = Memory::new();
        memory.write_byte(0x35, 0b0010_1110);

        let mut register = Register::new();
        register.set_carry_bit(true);
        register.set_x(0x03);

        let mut message_bus = MessageBus::new(&mut memory);

        let cycles = rol.call(arguments, &mut register, &mut message_bus);

        assert_eq!(memory.read_byte(0x35), 0b0101_1101);
        assert_eq!(register.p(), 0b0011_0000);
        assert_eq!(cycles, 6);
    }

    #[test]
    fn test_zero_page_x_with_carry_set_after() {
        let rol = Rol::new(0x36);
        let arguments = vec![0x32];
        let mut memory = Memory::new();
        memory.write_byte(0x35, 0b1010_1110);

        let mut register = Register::new();
        register.set_x(0x03);

        let mut message_bus = MessageBus::new(&mut memory);

        let cycles = rol.call(arguments, &mut register, &mut message_bus);

        assert_eq!(memory.read_byte(0x35), 0b0101_1100);
        assert_eq!(register.p(), 0b0011_0001);
        assert_eq!(cycles, 6);
    }

    #[test]
    fn test_zero_page_x_with_zero() {
        let rol = Rol::new(0x36);
        let arguments = vec![0x32];
        let mut memory = Memory::new();
        let mut register = Register::new();
        register.set_x(0x03);

        let mut message_bus = MessageBus::new(&mut memory);

        let cycles = rol.call(arguments, &mut register, &mut message_bus);

        assert_eq!(memory.read_byte(0x35), 0x00);
        assert_eq!(register.p(), 0b0011_0010);
        assert_eq!(cycles, 6);
    }

    #[test]
    fn test_zero_page_x_with_negative() {
        let rol = Rol::new(0x36);
        let arguments = vec![0x32];
        let mut memory = Memory::new();
        memory.write_byte(0x35, 0b0100_0001);

        let mut register = Register::new();
        register.set_x(0x03);

        let mut message_bus = MessageBus::new(&mut memory);

        let cycles = rol.call(arguments, &mut register, &mut message_bus);
        assert_eq!(memory.read_byte(0x35), 0b1000_0010);
        assert_eq!(register.p(), 0b1011_0000);
        assert_eq!(cycles, 6);
    }

    #[test]
    fn test_absolute() {
        let rol = Rol::new(0x2E);
        let arguments = vec![0x35, 0x5a];
        let mut memory = Memory::new();
        memory.write_byte(0x5a35, 0b0010_1110);

        let mut register = Register::new();

        let mut message_bus = MessageBus::new(&mut memory);

        let cycles = rol.call(arguments, &mut register, &mut message_bus);

        assert_eq!(memory.read_byte(0x5a35), 0b0101_1100);
        assert_eq!(register.p(), 0b0011_0000);
        assert_eq!(cycles, 6);
    }

    #[test]
    fn test_absolute_with_carry_set_before() {
        let rol = Rol::new(0x2E);
        let arguments = vec![0x35, 0x5a];
        let mut memory = Memory::new();
        memory.write_byte(0x5a35, 0b0010_1110);

        let mut register = Register::new();
        register.set_carry_bit(true);

        let mut message_bus = MessageBus::new(&mut memory);

        let cycles = rol.call(arguments, &mut register, &mut message_bus);

        assert_eq!(memory.read_byte(0x5a35), 0b0101_1101);
        assert_eq!(register.p(), 0b0011_0000);
        assert_eq!(cycles, 6);
    }

    #[test]
    fn test_absolute_with_carry_set_after() {
        let rol = Rol::new(0x2E);
        let arguments = vec![0x35, 0x5a];
        let mut memory = Memory::new();
        memory.write_byte(0x5a35, 0b1010_1110);

        let mut register = Register::new();
        let mut message_bus = MessageBus::new(&mut memory);

        let cycles = rol.call(arguments, &mut register, &mut message_bus);

        assert_eq!(memory.read_byte(0x5a35), 0b0101_1100);
        assert_eq!(register.p(), 0b0011_0001);
        assert_eq!(cycles, 6);
    }

    #[test]
    fn test_absolute_with_zero() {
        let rol = Rol::new(0x2E);
        let arguments = vec![0x35, 0x5a];
        let mut memory = Memory::new();
        let mut register = Register::new();
        let mut message_bus = MessageBus::new(&mut memory);

        let cycles = rol.call(arguments, &mut register, &mut message_bus);

        assert_eq!(memory.read_byte(0x5a35), 0x00);
        assert_eq!(register.p(), 0b0011_0010);
        assert_eq!(cycles, 6);
    }

    #[test]
    fn test_absolute_with_negative() {
        let rol = Rol::new(0x2E);
        let arguments = vec![0x35, 0x5a];
        let mut memory = Memory::new();
        memory.write_byte(0x5a35, 0b0100_0001);

        let mut register = Register::new();
        let mut message_bus = MessageBus::new(&mut memory);

        let cycles = rol.call(arguments, &mut register, &mut message_bus);
        assert_eq!(memory.read_byte(0x5a35), 0b1000_0010);
        assert_eq!(register.p(), 0b1011_0000);
        assert_eq!(cycles, 6);
    }

    #[test]
    fn test_absolute_x() {
        let rol = Rol::new(0x3E);
        let arguments = vec![0x32, 0x5a];
        let mut memory = Memory::new();
        memory.write_byte(0x5a35, 0b0010_1110);

        let mut register = Register::new();
        register.set_x(0x03);

        let mut message_bus = MessageBus::new(&mut memory);

        let cycles = rol.call(arguments, &mut register, &mut message_bus);

        assert_eq!(memory.read_byte(0x5a35), 0b0101_1100);
        assert_eq!(register.p(), 0b0011_0000);
        assert_eq!(cycles, 7);
    }

    #[test]
    fn test_absolute_x_with_carry_set_before() {
        let rol = Rol::new(0x3E);
        let arguments = vec![0x32, 0x5a];
        let mut memory = Memory::new();
        memory.write_byte(0x5a35, 0b0010_1110);

        let mut register = Register::new();
        register.set_carry_bit(true);
        register.set_x(0x03);

        let mut message_bus = MessageBus::new(&mut memory);

        let cycles = rol.call(arguments, &mut register, &mut message_bus);

        assert_eq!(memory.read_byte(0x5a35), 0b0101_1101);
        assert_eq!(register.p(), 0b0011_0000);
        assert_eq!(cycles, 7);
    }

    #[test]
    fn test_absolute_x_with_carry_set_after() {
        let rol = Rol::new(0x3E);
        let arguments = vec![0x32, 0x5a];
        let mut memory = Memory::new();
        memory.write_byte(0x5a35, 0b1010_1110);

        let mut register = Register::new();
        register.set_x(0x03);

        let mut message_bus = MessageBus::new(&mut memory);

        let cycles = rol.call(arguments, &mut register, &mut message_bus);

        assert_eq!(memory.read_byte(0x5a35), 0b0101_1100);
        assert_eq!(register.p(), 0b0011_0001);
        assert_eq!(cycles, 7);
    }

    #[test]
    fn test_absolute_x_with_zero() {
        let rol = Rol::new(0x3E);
        let arguments = vec![0x32, 0x5a];
        let mut memory = Memory::new();
        let mut register = Register::new();
        register.set_x(0x03);

        let mut message_bus = MessageBus::new(&mut memory);

        let cycles = rol.call(arguments, &mut register, &mut message_bus);

        assert_eq!(memory.read_byte(0x5a35), 0x00);
        assert_eq!(register.p(), 0b0011_0010);
        assert_eq!(cycles, 7);
    }

    #[test]
    fn test_absolute_x_with_negative() {
        let rol = Rol::new(0x3E);
        let arguments = vec![0x32, 0x5a];
        let mut memory = Memory::new();
        memory.write_byte(0x5a35, 0b0100_0001);

        let mut register = Register::new();
        register.set_x(0x03);

        let mut message_bus = MessageBus::new(&mut memory);

        let cycles = rol.call(arguments, &mut register, &mut message_bus);
        assert_eq!(memory.read_byte(0x5a35), 0b1000_0010);
        assert_eq!(register.p(), 0b1011_0000);
        assert_eq!(cycles, 7);
    }

    #[test]
    #[should_panic]
    fn test_invalid_opcode() {
        let rol = Rol::new(0x00);
        let arguments = vec![0xFF];
        let mut memory = Memory::new();
        let mut message_bus = MessageBus::new(&mut memory);
        let mut register = Register::new();

        rol.call(arguments, &mut register, &mut message_bus);
    }
}

