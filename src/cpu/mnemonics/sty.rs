/*
STY  Sore Index Y in Memory

     Y -> M                           N Z C I D V
                                      - - - - - -

     addressing    assembler    opc  bytes  cyles
     --------------------------------------------
     zeropage      STY oper      84    2     3
     zeropage,X    STY oper,X    94    2     4
     absolute      STY oper      8C    3     4
*/

use crate::cpu::addressing;
use crate::cpu::mnemonics::Mnemonic;
use crate::cpu::register::Register;
use crate::message_bus::MessageBus;
use crate::message_bus::MessageBusTarget;
use crate::message_bus::MessageBusMessage;

#[derive(Debug)]
pub struct Sty {
    mnemonic: String,
    opcode: u8
}

impl Sty {
    pub fn new(opcode: u8) -> Sty {
        return Sty { mnemonic: "STY".to_string(), opcode: opcode };
    }
}

impl Mnemonic for Sty {
    fn determine_bytes(&self) -> usize {
        return match self.opcode {
            0x84 => 2,
            0x94 => 2,
            0x8C => 3,
            _ => panic!("Invalid opcode `0x{:x}` for mnemonic {}", self.opcode, self.mnemonic)
        }
    }

    fn call(&self, arguments: Vec<u8>, register: &mut Register, message_bus: &mut MessageBus) -> u8 {
        match self.opcode {
            0x84 => return self.call_zero_page(arguments, register, message_bus),
            0x94 => return self.call_zero_page_x(arguments, register, message_bus),
            0x8C => return self.call_absolute(arguments, register, message_bus),
            _ => panic!("Invalid opcode `0x{:x}` for mnemonic {}", self.opcode, self.mnemonic)
        }
    }

    fn call_zero_page(&self, arguments: Vec<u8>, register: &mut Register, message_bus: &mut MessageBus) -> u8 {
        let (memory_address, _memory_value, _boundary_crossed) = addressing::zero_page(arguments, message_bus);

        message_bus.send_message(
            MessageBusTarget::Memory, MessageBusMessage::Write, vec![memory_address, register.y() as u16]
        );

        return 3;
    }

    fn call_zero_page_x(&self, arguments: Vec<u8>, register: &mut Register, message_bus: &mut MessageBus) -> u8 {
        let (memory_address, _memory_value, _boundary_crossed) = addressing::zero_page_x(arguments, message_bus, register);

        message_bus.send_message(
            MessageBusTarget::Memory, MessageBusMessage::Write, vec![memory_address, register.y() as u16]
        );

        return 4;
    }

    fn call_absolute(&self, arguments: Vec<u8>, register: &mut Register, message_bus: &mut MessageBus) -> u8 {
        let (memory_address, _memory_value, _boundary_crossed) = addressing::absolute(arguments, message_bus);

        message_bus.send_message(
            MessageBusTarget::Memory, MessageBusMessage::Write, vec![memory_address, register.y() as u16]
        );

        return 4;
    }
}

#[cfg(test)]
mod tests {
    use super::Sty;
    use crate::cpu::mnemonics::Mnemonic;
    use crate::cpu::register::Register;
    use crate::memory::Memory;
    use crate::message_bus::MessageBus;

    #[test]
    fn test_zero_page() {
        let sty = Sty::new(0x84);
        let arguments = vec![0x30];
        let mut memory = Memory::new();
        let mut register = Register::new();
        register.set_y(0x42);

        let mut message_bus = MessageBus::new(&mut memory);

        let cycles = sty.call(arguments, &mut register, &mut message_bus);

        assert_eq!(memory.read_byte(0x30), 0x42);
        assert_eq!(register.p(), 0b00110000);
        assert_eq!(cycles, 3);
    }

    #[test]
    fn test_zero_page_x() {
        let sty = Sty::new(0x94);
        let arguments = vec![0x30];
        let mut memory = Memory::new();
        let mut register = Register::new();
        register.set_x(0x05);
        register.set_y(0x42);

        let mut message_bus = MessageBus::new(&mut memory);

        let cycles = sty.call(arguments, &mut register, &mut message_bus);

        assert_eq!(memory.read_byte(0x35), 0x42);
        assert_eq!(register.p(), 0b00110000);
        assert_eq!(cycles, 4);
    }

    #[test]
    fn test_absolute() {
        let sty = Sty::new(0x8C);
        let arguments = vec![0x3c, 0x5a];
        let mut memory = Memory::new();
        let mut register = Register::new();
        register.set_y(0x42);

        let mut message_bus = MessageBus::new(&mut memory);

        let cycles = sty.call(arguments, &mut register, &mut message_bus);

        assert_eq!(memory.read_byte(0x5a3c), 0x42);
        assert_eq!(register.p(), 0b00110000);
        assert_eq!(cycles, 4);
    }

    #[test]
    #[should_panic]
    fn test_invalid_opcode() {
        let sty = Sty::new(0x00);
        let arguments = vec![0xFF];
        let mut memory = Memory::new();
        let mut message_bus = MessageBus::new(&mut memory);
        let mut register = Register::new();

        sty.call(arguments, &mut register, &mut message_bus);
    }
}




