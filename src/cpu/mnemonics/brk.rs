/*
BRK  Force Break

     interrupt,                       N Z C I D V
     push PC+2, push SR               - - - 1 - -

     addressing    assembler    opc  bytes  cyles
     --------------------------------------------
     implied       BRK           00    1     7
*/

use crate::cpu::addressing;
use crate::cpu::mnemonics::Mnemonic;
use crate::cpu::register::Register;
use crate::message_bus::MessageBus;
use crate::message_bus::MessageBusMessage;
use crate::message_bus::MessageBusTarget;

#[derive(Debug)]
pub struct Brk {
    mnemonic: String,
    opcode: u8
}

impl Brk {
    pub fn new(opcode: u8) -> Brk {
        return Brk { mnemonic: "BRK".to_string(), opcode: opcode };
    }
}

impl Mnemonic for Brk {
    fn determine_bytes(&self) -> usize {
        return match self.opcode {
            0x00 => 1,
            _ => panic!("Invalid opcode `0x{:x}` for mnemonic {}", self.opcode, self.mnemonic)
        }
    }

    fn call(&self, _arguments: Vec<u8>, register: &mut Register, message_bus: &mut MessageBus) -> u8 {
        match self.opcode {
            0x00 => return self.call_implied(register, message_bus),
            _ => panic!("Invalid opcode `0x{:x}` for mnemonic {}", self.opcode, self.mnemonic)
        }
    }

    fn call_implied(&self, register: &mut Register, message_bus: &mut MessageBus) -> u8 {
        addressing::stack_push((register.pc() >> 8) as u8, message_bus, register);
        addressing::stack_push((register.pc() & 0x00FF) as u8, message_bus, register);
        register.set_break_bit(true);
        addressing::stack_push(register.p(), message_bus, register);
        let pc_low = message_bus.send_message(
            MessageBusTarget::Memory, MessageBusMessage::Read, vec![0xfffe]
        );
        let pc_high = message_bus.send_message(
            MessageBusTarget::Memory, MessageBusMessage::Read, vec![0xffff]
        );
        register.set_pc(((pc_high as u16) << 8) + pc_low as u16);
        register.set_interrupt_bit(true);
        register.increment_pc();

        return 7;
    }
}

#[cfg(test)]
mod tests {
    use super::Brk;
    use crate::cpu::mnemonics::Mnemonic;
    use crate::cpu::register::Register;
    use crate::memory::Memory;
    use crate::message_bus::MessageBus;

    #[test]
    fn test_implied() {
        let brk = Brk::new(0x0);
        let mut memory = Memory::new();
        memory.write_byte(0xfffe, 0x20);
        memory.write_byte(0xffff, 0x04);
        let mut register = Register::new();
        register.set_pc(0x0305);

        let mut message_bus = MessageBus::new(&mut memory);

        let cycles = brk.call(vec![0x00], &mut register, &mut message_bus);

        assert_eq!(register.pc(), 0x0421);
        assert_eq!(register.p(), 0b0011_0100);
        assert_eq!(register.s(), 0xfc);
        assert_eq!(memory.read_byte(0x1ff), 0x03);
        assert_eq!(memory.read_byte(0x1fe), 0x05);
        assert_eq!(cycles, 7);
    }

    #[test]
    #[should_panic]
    fn test_invalid_opcode() {
        let brk = Brk::new(0x0F);
        let arguments = vec![0xFF];
        let mut memory = Memory::new();
        let mut message_bus = MessageBus::new(&mut memory);
        let mut register = Register::new();

        brk.call(arguments, &mut register, &mut message_bus);
    }
}


