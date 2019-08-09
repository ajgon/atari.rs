/*
STX  Store Index X in Memory

     X -> M                           N Z C I D V
                                      - - - - - -

     addressing    assembler    opc  bytes  cyles
     --------------------------------------------
     zeropage      STX oper      86    2     3
     zeropage,Y    STX oper,Y    96    2     4
     absolute      STX oper      8E    3     4
*/

use crate::cpu::addressing;
use crate::cpu::mnemonics::Mnemonic;
use crate::cpu::register::Register;
use crate::message_bus::MessageBus;
use crate::message_bus::MessageBusTarget;
use crate::message_bus::MessageBusMessage;

#[derive(Debug)]
pub struct Stx {
    mnemonic: String,
    opcode: u8
}

impl Stx {
    pub fn new(opcode: u8) -> Stx {
        return Stx { mnemonic: "STX".to_string(), opcode: opcode };
    }
}

impl Mnemonic for Stx {
    fn determine_bytes(&self) -> usize {
        return match self.opcode {
            0x86 => 2,
            0x96 => 2,
            0x8E => 3,
            _ => panic!("Invalid opcode `0x{:x}` for mnemonic {}", self.opcode, self.mnemonic)
        }
    }

    fn call(&self, arguments: Vec<u8>, register: &mut Register, message_bus: &mut MessageBus) -> u8 {
        match self.opcode {
            0x86 => return self.call_zero_page(arguments, register, message_bus),
            0x96 => return self.call_zero_page_y(arguments, register, message_bus),
            0x8E => return self.call_absolute(arguments, register, message_bus),
            _ => panic!("Invalid opcode `0x{:x}` for mnemonic {}", self.opcode, self.mnemonic)
        }
    }

    fn call_zero_page(&self, arguments: Vec<u8>, register: &mut Register, message_bus: &mut MessageBus) -> u8 {
        let (memory_address, _memory_value, _boundary_crossed) = addressing::zero_page(arguments, message_bus);

        message_bus.send_message(
            MessageBusTarget::Memory, MessageBusMessage::Write, vec![memory_address, register.x() as u16]
        );

        return 3;
    }

    fn call_zero_page_y(&self, arguments: Vec<u8>, register: &mut Register, message_bus: &mut MessageBus) -> u8 {
        let (memory_address, _memory_value, _boundary_crossed) = addressing::zero_page_y(arguments, message_bus, register);

        message_bus.send_message(
            MessageBusTarget::Memory, MessageBusMessage::Write, vec![memory_address, register.x() as u16]
        );

        return 4;
    }

    fn call_absolute(&self, arguments: Vec<u8>, register: &mut Register, message_bus: &mut MessageBus) -> u8 {
        let (memory_address, _memory_value, _boundary_crossed) = addressing::absolute(arguments, message_bus);

        message_bus.send_message(
            MessageBusTarget::Memory, MessageBusMessage::Write, vec![memory_address, register.x() as u16]
        );

        return 4;
    }
}

#[cfg(test)]
mod tests {
    use super::Stx;
    use crate::cpu::mnemonics::Mnemonic;
    use crate::cpu::register::Register;
    use crate::memory::Memory;
    use crate::message_bus::MessageBus;

    #[test]
    fn test_zero_page() {
        let stx = Stx::new(0x86);
        let arguments = vec![0x30];
        let mut memory = Memory::new();
        let mut register = Register::new();
        register.set_x(0x42);

        let mut message_bus = MessageBus::new(&mut memory);

        let cycles = stx.call(arguments, &mut register, &mut message_bus);

        assert_eq!(memory.read_byte(0x30), 0x42);
        assert_eq!(register.p(), 0b0010_0000);
        assert_eq!(cycles, 3);
    }

    #[test]
    fn test_zero_page_y() {
        let stx = Stx::new(0x96);
        let arguments = vec![0x30];
        let mut memory = Memory::new();
        let mut register = Register::new();
        register.set_x(0x42);
        register.set_y(0x05);

        let mut message_bus = MessageBus::new(&mut memory);

        let cycles = stx.call(arguments, &mut register, &mut message_bus);

        assert_eq!(memory.read_byte(0x35), 0x42);
        assert_eq!(register.p(), 0b0010_0000);
        assert_eq!(cycles, 4);
    }

    #[test]
    fn test_absolute() {
        let stx = Stx::new(0x8E);
        let arguments = vec![0x3c, 0x5a];
        let mut memory = Memory::new();
        let mut register = Register::new();
        register.set_x(0x42);

        let mut message_bus = MessageBus::new(&mut memory);

        let cycles = stx.call(arguments, &mut register, &mut message_bus);

        assert_eq!(memory.read_byte(0x5a3c), 0x42);
        assert_eq!(register.p(), 0b0010_0000);
        assert_eq!(cycles, 4);
    }

    #[test]
    #[should_panic]
    fn test_invalid_opcode() {
        let stx = Stx::new(0x00);
        let arguments = vec![0xFF];
        let mut memory = Memory::new();
        let mut message_bus = MessageBus::new(&mut memory);
        let mut register = Register::new();

        stx.call(arguments, &mut register, &mut message_bus);
    }
}



