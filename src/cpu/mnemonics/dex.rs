/*
DEX  Decrement Index X by One

     X - 1 -> X                       N Z C I D V
                                      + + - - - -

     addressing    assembler    opc  bytes  cyles
     --------------------------------------------
     implied       DEC           CA    1     2
*/

use crate::cpu::mnemonics::Mnemonic;
use crate::cpu::register::Register;
use crate::message_bus::MessageBus;
use crate::cpu::alu;

#[derive(Debug)]
pub struct Dex {
    mnemonic: String,
    opcode: u8
}

impl Dex {
    pub fn new(opcode: u8) -> Dex {
        return Dex { mnemonic: "DEX".to_string(), opcode: opcode };
    }
}

impl Mnemonic for Dex {
    fn determine_bytes(&self) -> usize {
        return match self.opcode {
            0xCA => 1,
            _ => panic!("Invalid opcode `0x{:x}` for mnemonic {}", self.opcode, self.mnemonic)
        }
    }

    fn call(&self, _arguments: Vec<u8>, register: &mut Register, _message_bus: &mut MessageBus) -> u8 {
        match self.opcode {
            0xCA => return self.call_implied(register, _message_bus),
            _ => panic!("Invalid opcode `0x{:x}` for mnemonic {}", self.opcode, self.mnemonic)
        }
    }

    fn call_implied(&self, register: &mut Register, _message_bus: &mut MessageBus) -> u8 {
        let result = alu::decrement(register.x(), register);

        register.set_x(result);

        return 2;
    }
}

#[cfg(test)]
mod tests {
    use super::Dex;
    use crate::cpu::mnemonics::Mnemonic;
    use crate::cpu::register::Register;
    use crate::memory::Memory;
    use crate::message_bus::MessageBus;

    #[test]
    fn test_implied() {
        let dex = Dex::new(0xCA);
        let mut memory = Memory::new();
        let mut register = Register::new();
        register.set_x(0x00);

        let mut message_bus = MessageBus::new(&mut memory);

        let cycles = dex.call(vec![0x00], &mut register, &mut message_bus);

        assert_eq!(0xFF, register.x());
        assert_eq!(0b1011_0000, register.p());
        assert_eq!(cycles, 2);
    }

    #[test]
    #[should_panic]
    fn test_invalid_opcode() {
        let dex = Dex::new(0x00);
        let arguments = vec![0xFF];
        let mut memory = Memory::new();
        let mut message_bus = MessageBus::new(&mut memory);
        let mut register = Register::new();

        dex.call(arguments, &mut register, &mut message_bus);
    }
}


