/*
ASL  Shift Left One Bit (Memory or Accumulator)

     C <- [76543210] <- 0             N Z C I D V
                                      + + + - - -

     addressing    assembler    opc  bytes  cyles
     --------------------------------------------
     accumulator   ASL A         0A    1     2
     zeropage      ASL oper      06    2     5
     zeropage,X    ASL oper,X    16    2     6
     absolute      ASL oper      0E    3     6
     absolute,X    ASL oper,X    1E    3     7
*/

use crate::cpu::addressing;
use crate::cpu::mnemonics::Mnemonic;
use crate::cpu::register::Register;
use crate::message_bus::MessageBus;
use crate::cpu::alu;

#[derive(Debug)]
pub struct Asl {
    mnemonic: String,
    opcode: u8
}

impl Asl {
    pub fn new(opcode: u8) -> Asl {
        return Asl { mnemonic: "ASL".to_string(), opcode: opcode };
    }
}

impl Mnemonic for Asl {
    fn determine_bytes(&self) -> usize {
        return match self.opcode {
            0x0A => 1,
            0x06 => 2,
            0x16 => 2,
            0x0E => 3,
            0x1E => 3,
            _ => panic!("Invalid opcode `0x{:x}` for mnemonic {}", self.opcode, self.mnemonic)
        }
    }

    fn call(&self, arguments: Vec<u8>, register: &mut Register, message_bus: &mut MessageBus) -> u8 {
        match self.opcode {
            0x0A => return self.call_accumulator(register),
            0x06 => return self.call_zero_page(arguments, register, message_bus),
            0x16 => return self.call_zero_page_x(arguments, register, message_bus),
            0x0E => return self.call_absolute(arguments, register, message_bus),
            0x1E => return self.call_absolute_x(arguments, register, message_bus),
            _ => panic!("Invalid opcode `0x{:x}` for mnemonic {}", self.opcode, self.mnemonic)
        }
    }

    fn call_accumulator(&self, register: &mut Register) -> u8 {
        let result = alu::shift_left(register.a(), register);
        register.set_accumulator(result);

        return 2;
    }

    fn call_zero_page(&self, arguments: Vec<u8>, register: &mut Register, message_bus: &mut MessageBus) -> u8 {
        let (_memory_address, memory_value, _boundary_crossed) = addressing::zero_page(arguments, message_bus);

        let result = alu::shift_left(memory_value, register);
        register.set_accumulator(result);

        return 5;
    }

    fn call_zero_page_x(&self, arguments: Vec<u8>, register: &mut Register, message_bus: &mut MessageBus) -> u8 {
        let (_memory_address, memory_value, _boundary_crossed) = addressing::zero_page_x(arguments, message_bus, register);

        let result = alu::shift_left(memory_value, register);
        register.set_accumulator(result);

        return 6;
    }

    fn call_absolute(&self, arguments: Vec<u8>, register: &mut Register, message_bus: &mut MessageBus) -> u8 {
        let (_memory_address, memory_value, _boundary_crossed) = addressing::absolute(arguments, message_bus);

        let result = alu::shift_left(memory_value, register);
        register.set_accumulator(result);

        return 6;
    }

    fn call_absolute_x(&self, arguments: Vec<u8>, register: &mut Register, message_bus: &mut MessageBus) -> u8 {
        let (_memory_address, memory_value, _boundary_crossed) = addressing::absolute_x(arguments, message_bus, register);

        let result = alu::shift_left(memory_value, register);
        register.set_accumulator(result);

        return 7;
    }
}

#[cfg(test)]
mod tests {
    use super::Asl;
    use crate::cpu::mnemonics::Mnemonic;
    use crate::cpu::register::Register;
    use crate::memory::Memory;
    use crate::message_bus::MessageBus;

    #[test]
    fn test_accumulator() {
        let asl = Asl::new(0x0A);
        let mut memory = Memory::new();
        let mut register = Register::new();
        register.set_accumulator(0b0010_1100);

        let mut message_bus = MessageBus::new(&mut memory);

        let cycles = asl.call(vec![register.a()], &mut register, &mut message_bus);

        assert_eq!(register.a(), 0b0101_1000);
        assert_eq!(register.p(), 0b0011_0000);
        assert_eq!(cycles, 2);
    }

    #[test]
    fn test_zero_page() {
        let asl = Asl::new(0x06);
        let arguments = vec![0x30];
        let mut memory = Memory::new();
        memory.write_byte(0x30, 0b0010_1100);

        let mut register = Register::new();

        let mut message_bus = MessageBus::new(&mut memory);

        let cycles = asl.call(arguments, &mut register, &mut message_bus);

        assert_eq!(register.a(), 0b0101_1000);
        assert_eq!(register.p(), 0b0011_0000);
        assert_eq!(cycles, 5);
    }

    #[test]
    fn test_zero_page_x() {
        let asl = Asl::new(0x16);
        let arguments = vec![0x30];
        let mut memory = Memory::new();
        memory.write_byte(0x35, 0b0010_1100);

        let mut register = Register::new();
        register.set_x(0x05);

        let mut message_bus = MessageBus::new(&mut memory);

        let cycles = asl.call(arguments, &mut register, &mut message_bus);

        assert_eq!(register.a(), 0b0101_1000);
        assert_eq!(register.p(), 0b0011_0000);
        assert_eq!(cycles, 6);
    }

    #[test]
    fn test_absolute() {
        let asl = Asl::new(0x0E);
        let arguments = vec![0x3c, 0x5a];
        let mut memory = Memory::new();
        memory.write_byte(0x5a3c, 0b0010_1100);

        let mut register = Register::new();

        let mut message_bus = MessageBus::new(&mut memory);

        let cycles = asl.call(arguments, &mut register, &mut message_bus);

        assert_eq!(register.a(), 0b0101_1000);
        assert_eq!(register.p(), 0b0011_0000);
        assert_eq!(cycles, 6);
    }

    #[test]
    fn test_absolute_x() {
        let asl = Asl::new(0x1E);
        let arguments = vec![0x3c, 0x5a];
        let mut memory = Memory::new();
        memory.write_byte(0x5a4c, 0b0010_1100);

        let mut register = Register::new();
        register.set_x(0x10);
        register.set_y(0x20);

        let mut message_bus = MessageBus::new(&mut memory);

        let cycles = asl.call(arguments, &mut register, &mut message_bus);

        assert_eq!(register.a(), 0b0101_1000);
        assert_eq!(register.p(), 0b0011_0000);
        assert_eq!(cycles, 7);
    }

    #[test]
    #[should_panic]
    fn test_invalid_opcode() {
        let asl = Asl::new(0x00);
        let arguments = vec![0xFF];
        let mut memory = Memory::new();
        let mut message_bus = MessageBus::new(&mut memory);
        let mut register = Register::new();

        asl.call(arguments, &mut register, &mut message_bus);
    }
}


