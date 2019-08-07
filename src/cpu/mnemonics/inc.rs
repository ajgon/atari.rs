/*
INC  Increment Memory by One

     M + 1 -> M                       N Z C I D V
                                      + + - - - -

     addressing    assembler    opc  bytes  cyles
     --------------------------------------------
     zeropage      INC oper      E6    2     5
     zeropage,X    INC oper,X    F6    2     6
     absolute      INC oper      EE    3     6
     absolute,X    INC oper,X    FE    3     7
*/

use crate::cpu::addressing;
use crate::cpu::mnemonics::Mnemonic;
use crate::cpu::register::Register;
use crate::message_bus::MessageBus;
use crate::message_bus::MessageBusMessage;
use crate::message_bus::MessageBusTarget;
use crate::cpu::alu;

#[derive(Debug)]
pub struct Inc {
    mnemonic: String,
    opcode: u8
}

impl Inc {
    pub fn new(opcode: u8) -> Inc {
        return Inc { mnemonic: "INC".to_string(), opcode: opcode };
    }
}

impl Mnemonic for Inc {
    fn determine_bytes(&self) -> usize {
        return match self.opcode {
            0xE6 => 2,
            0xF6 => 2,
            0xEE => 3,
            0xFE => 3,
            _ => panic!("Invalid opcode `0x{:x}` for mnemonic {}", self.opcode, self.mnemonic)
        }
    }

    fn call(&self, arguments: Vec<u8>, register: &mut Register, message_bus: &mut MessageBus) -> u8 {
        match self.opcode {
            0xE6 => return self.call_zero_page(arguments, register, message_bus),
            0xF6 => return self.call_zero_page_x(arguments, register, message_bus),
            0xEE => return self.call_absolute(arguments, register, message_bus),
            0xFE => return self.call_absolute_x(arguments, register, message_bus),
            _ => panic!("Invalid opcode `0x{:x}` for mnemonic {}", self.opcode, self.mnemonic)
        }
    }

    fn call_zero_page(&self, arguments: Vec<u8>, register: &mut Register, message_bus: &mut MessageBus) -> u8 {
        let (memory_address, memory_value, _boundary_crossed) = addressing::zero_page(arguments, message_bus);

        let result = alu::increment(memory_value, register);
        message_bus.send_message(
            MessageBusTarget::Memory, MessageBusMessage::Write, vec![memory_address, result as u16]
        );

        return 5;
    }

    fn call_zero_page_x(&self, arguments: Vec<u8>, register: &mut Register, message_bus: &mut MessageBus) -> u8 {
        let (memory_address, memory_value, _boundary_crossed) = addressing::zero_page_x(arguments, message_bus, register);

        let result = alu::increment(memory_value, register);
        message_bus.send_message(
            MessageBusTarget::Memory, MessageBusMessage::Write, vec![memory_address, result as u16]
        );

        return 6;
    }

    fn call_absolute(&self, arguments: Vec<u8>, register: &mut Register, message_bus: &mut MessageBus) -> u8 {
        let (memory_address, memory_value, _boundary_crossed) = addressing::absolute(arguments, message_bus);

        let result = alu::increment(memory_value, register);
        message_bus.send_message(
            MessageBusTarget::Memory, MessageBusMessage::Write, vec![memory_address, result as u16]
        );

        return 6;
    }

    fn call_absolute_x(&self, arguments: Vec<u8>, register: &mut Register, message_bus: &mut MessageBus) -> u8 {
        let (memory_address, memory_value, _boundary_crossed) = addressing::absolute_x(arguments, message_bus, register);

        let result = alu::increment(memory_value, register);
        message_bus.send_message(
            MessageBusTarget::Memory, MessageBusMessage::Write, vec![memory_address, result as u16]
        );

        return 7;
    }
}

#[cfg(test)]
mod tests {
    use super::Inc;
    use crate::cpu::mnemonics::Mnemonic;
    use crate::cpu::register::Register;
    use crate::memory::Memory;
    use crate::message_bus::MessageBus;

    #[test]
    fn test_zero_page() {
        let inc = Inc::new(0xE6);
        let arguments = vec![0x30];
        let mut memory = Memory::new();
        memory.write_byte(0x30, 0xCC);

        let mut register = Register::new();

        let mut message_bus = MessageBus::new(&mut memory);

        let cycles = inc.call(arguments, &mut register, &mut message_bus);

        assert_eq!(memory.read_byte(0x30), 0xCD);
        assert_eq!(register.p(), 0b1011_0000);
        assert_eq!(cycles, 5);
    }

    #[test]
    fn test_zero_page_x() {
        let inc = Inc::new(0xF6);
        let arguments = vec![0x30];
        let mut memory = Memory::new();
        memory.write_byte(0x35, 0xCC);

        let mut register = Register::new();
        register.set_x(0x05);

        let mut message_bus = MessageBus::new(&mut memory);

        let cycles = inc.call(arguments, &mut register, &mut message_bus);

        assert_eq!(memory.read_byte(0x35), 0xCD);
        assert_eq!(register.p(), 0b1011_0000);
        assert_eq!(cycles, 6);
    }

    #[test]
    fn test_absolute() {
        let inc = Inc::new(0xEE);
        let arguments = vec![0x3c, 0x5a];
        let mut memory = Memory::new();
        memory.write_byte(0x5a3c, 0xCC);

        let mut register = Register::new();

        let mut message_bus = MessageBus::new(&mut memory);

        let cycles = inc.call(arguments, &mut register, &mut message_bus);

        assert_eq!(memory.read_byte(0x5a3c), 0xCD);
        assert_eq!(register.p(), 0b1011_0000);
        assert_eq!(cycles, 6);
    }

    #[test]
    fn test_absolute_x() {
        let inc = Inc::new(0xFE);
        let arguments = vec![0x3c, 0x5a];
        let mut memory = Memory::new();
        memory.write_byte(0x5a4c, 0xCC);

        let mut register = Register::new();
        register.set_x(0x10);
        register.set_y(0x20);

        let mut message_bus = MessageBus::new(&mut memory);

        let cycles = inc.call(arguments, &mut register, &mut message_bus);

        assert_eq!(memory.read_byte(0x5a4c), 0xCD);
        assert_eq!(register.p(), 0b1011_0000);
        assert_eq!(cycles, 7);
    }

    #[test]
    #[should_panic]
    fn test_invalid_opcode() {
        let inc = Inc::new(0x00);
        let arguments = vec![0xFF];
        let mut memory = Memory::new();
        let mut message_bus = MessageBus::new(&mut memory);
        let mut register = Register::new();

        inc.call(arguments, &mut register, &mut message_bus);
    }
}




