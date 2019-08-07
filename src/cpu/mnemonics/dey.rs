/*
DEY  Decrement Index Y by One

     Y - 1 -> Y                       N Z C I D V
                                      + + - - - -

     addressing    assembler    opc  bytes  cyles
     --------------------------------------------
     implied       DEC           88    1     2
*/

use crate::cpu::mnemonics::Mnemonic;
use crate::cpu::register::Register;
use crate::message_bus::MessageBus;
use crate::cpu::alu;

#[derive(Debug)]
pub struct Dey {
    mnemonic: String,
    opcode: u8
}

impl Dey {
    pub fn new(opcode: u8) -> Dey {
        return Dey { mnemonic: "DEY".to_string(), opcode: opcode };
    }
}

impl Mnemonic for Dey {
    fn determine_bytes(&self) -> usize {
        return match self.opcode {
            0x88 => 1,
            _ => panic!("Invalid opcode `0x{:x}` for mnemonic {}", self.opcode, self.mnemonic)
        }
    }

    fn call(&self, _arguments: Vec<u8>, register: &mut Register, _message_bus: &mut MessageBus) -> u8 {
        match self.opcode {
            0x88 => return self.call_implied(register, _message_bus),
            _ => panic!("Invalid opcode `0x{:x}` for mnemonic {}", self.opcode, self.mnemonic)
        }
    }

    fn call_implied(&self, register: &mut Register, _message_bus: &mut MessageBus) -> u8 {
        let result = alu::decrement(register.y(), register);

        register.set_y(result);

        return 2;
    }
}

#[cfg(test)]
mod tests {
    use super::Dey;
    use crate::cpu::mnemonics::Mnemonic;
    use crate::cpu::register::Register;
    use crate::memory::Memory;
    use crate::message_bus::MessageBus;

    #[test]
    fn test_implied() {
        let dey = Dey::new(0x88);
        let mut memory = Memory::new();
        let mut register = Register::new();
        register.set_y(0x00);

        let mut message_bus = MessageBus::new(&mut memory);

        let cycles = dey.call(vec![0x00], &mut register, &mut message_bus);

        assert_eq!(0xFF, register.y());
        assert_eq!(0b1011_0000, register.p());
        assert_eq!(cycles, 2);
    }

    #[test]
    #[should_panic]
    fn test_invalid_opcode() {
        let dey = Dey::new(0x00);
        let arguments = vec![0xFF];
        let mut memory = Memory::new();
        let mut message_bus = MessageBus::new(&mut memory);
        let mut register = Register::new();

        dey.call(arguments, &mut register, &mut message_bus);
    }
}



