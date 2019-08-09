/*
RTS  Return from Subroutine

     pull PC, PC+1 -> PC              N Z C I D V
                                      - - - - - -

     addressing    assembler    opc  bytes  cyles
     --------------------------------------------
     implied       RTS           60    1     6
*/

use crate::cpu::addressing;
use crate::cpu::mnemonics::Mnemonic;
use crate::cpu::register::Register;
use crate::message_bus::MessageBus;

#[derive(Debug)]
pub struct Rts {
    mnemonic: String,
    opcode: u8
}

impl Rts {
    pub fn new(opcode: u8) -> Rts {
        return Rts { mnemonic: "RTS".to_string(), opcode: opcode };
    }
}

impl Mnemonic for Rts {
    fn determine_bytes(&self) -> usize {
        return match self.opcode {
            0x60 => 1,
            _ => panic!("Invalid opcode `0x{:x}` for mnemonic {}", self.opcode, self.mnemonic)
        }
    }

    fn call(&self, _arguments: Vec<u8>, register: &mut Register, _message_bus: &mut MessageBus) -> u8 {
        match self.opcode {
            0x60 => return self.call_implied(register, _message_bus),
            _ => panic!("Invalid opcode `0x{:x}` for mnemonic {}", self.opcode, self.mnemonic)
        }
    }

    fn call_implied(&self, register: &mut Register, _message_bus: &mut MessageBus) -> u8 {
        let pcl = addressing::stack_pull(_message_bus, register);
        let pch = addressing::stack_pull(_message_bus, register);

        register.set_pc((((pch as u16) << 8) + pcl as u16).overflowing_add(1).0);

        return 6;
    }
}

#[cfg(test)]
mod tests {
    use super::Rts;
    use crate::cpu::mnemonics::Mnemonic;
    use crate::cpu::register::Register;
    use crate::memory::Memory;
    use crate::message_bus::MessageBus;

    #[test]
    fn test_implied() {
        let rts = Rts::new(0x60);
        let mut memory = Memory::new();
        memory.write_byte(0x01FF, 0x06);
        memory.write_byte(0x01FE, 0x55);

        let mut register = Register::new();
        register.push_s();
        register.push_s();

        let mut message_bus = MessageBus::new(&mut memory);

        let cycles = rts.call(vec![0x00], &mut register, &mut message_bus);

        assert_eq!(0b0011_0000, register.p());
        assert_eq!(0x0656, register.pc());
        assert_eq!(cycles, 6);
    }

    #[test]
    #[should_panic]
    fn test_invalid_opcode() {
        let rts = Rts::new(0x00);
        let arguments = vec![0xFF];
        let mut memory = Memory::new();
        let mut message_bus = MessageBus::new(&mut memory);
        let mut register = Register::new();

        rts.call(arguments, &mut register, &mut message_bus);
    }
}

