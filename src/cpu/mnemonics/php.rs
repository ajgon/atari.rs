/*
PHP  Push Processor Status on Stack

     push SR                          N Z C I D V
                                      - - - - - -

     addressing    assembler    opc  bytes  cyles
     --------------------------------------------
     implied       PHP           08    1     3
*/

use crate::cpu::addressing;
use crate::cpu::mnemonics::Mnemonic;
use crate::cpu::register::Register;
use crate::message_bus::MessageBus;

#[derive(Debug)]
pub struct Php {
    mnemonic: String,
    opcode: u8
}

impl Php {
    pub fn new(opcode: u8) -> Php {
        return Php { mnemonic: "PHP".to_string(), opcode: opcode };
    }
}

impl Mnemonic for Php {
    fn determine_bytes(&self) -> usize {
        return match self.opcode {
            0x08 => 1,
            _ => panic!("Invalid opcode `0x{:x}` for mnemonic {}", self.opcode, self.mnemonic)
        }
    }

    fn call(&self, _arguments: Vec<u8>, register: &mut Register, _message_bus: &mut MessageBus) -> u8 {
        match self.opcode {
            0x08 => return self.call_implied(register, _message_bus),
            _ => panic!("Invalid opcode `0x{:x}` for mnemonic {}", self.opcode, self.mnemonic)
        }
    }

    fn call_implied(&self, register: &mut Register, _message_bus: &mut MessageBus) -> u8 {
        addressing::stack_push(register.p(), _message_bus, register);
        return 3;
    }
}

#[cfg(test)]
mod tests {
    use super::Php;
    use crate::cpu::mnemonics::Mnemonic;
    use crate::cpu::register::Register;
    use crate::memory::Memory;
    use crate::message_bus::MessageBus;

    #[test]
    fn test_implied() {
        let php = Php::new(0x08);
        let mut memory = Memory::new();
        let mut register = Register::new();

        let mut message_bus = MessageBus::new(&mut memory);

        let cycles = php.call(vec![0x00], &mut register, &mut message_bus);

        assert_eq!(memory.read_byte(0x01ff), 0x30);
        assert_eq!(0b0011_0000, register.p());
        assert_eq!(cycles, 3);
    }

    #[test]
    #[should_panic]
    fn test_invalid_opcode() {
        let php = Php::new(0x00);
        let arguments = vec![0xFF];
        let mut memory = Memory::new();
        let mut message_bus = MessageBus::new(&mut memory);
        let mut register = Register::new();

        php.call(arguments, &mut register, &mut message_bus);
    }
}



