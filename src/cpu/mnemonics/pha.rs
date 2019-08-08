/*
PHA  Clear Carry Flag

     0 -> C                           N Z C I D V
                                      - - 0 - - -

     addressing    assembler    opc  bytes  cyles
     --------------------------------------------
     implied       PHA           18    1     2
*/

use crate::cpu::addressing;
use crate::cpu::mnemonics::Mnemonic;
use crate::cpu::register::Register;
use crate::message_bus::MessageBus;

#[derive(Debug)]
pub struct Pha {
    mnemonic: String,
    opcode: u8
}

impl Pha {
    pub fn new(opcode: u8) -> Pha {
        return Pha { mnemonic: "PHA".to_string(), opcode: opcode };
    }
}

impl Mnemonic for Pha {
    fn determine_bytes(&self) -> usize {
        return match self.opcode {
            0x48 => 1,
            _ => panic!("Invalid opcode `0x{:x}` for mnemonic {}", self.opcode, self.mnemonic)
        }
    }

    fn call(&self, _arguments: Vec<u8>, register: &mut Register, _message_bus: &mut MessageBus) -> u8 {
        match self.opcode {
            0x48 => return self.call_implied(register, _message_bus),
            _ => panic!("Invalid opcode `0x{:x}` for mnemonic {}", self.opcode, self.mnemonic)
        }
    }

    fn call_implied(&self, register: &mut Register, _message_bus: &mut MessageBus) -> u8 {
        addressing::stack_push(register.a(), _message_bus, register);
        return 3;
    }
}

#[cfg(test)]
mod tests {
    use super::Pha;
    use crate::cpu::mnemonics::Mnemonic;
    use crate::cpu::register::Register;
    use crate::memory::Memory;
    use crate::message_bus::MessageBus;

    #[test]
    fn test_implied() {
        let pha = Pha::new(0x48);
        let mut memory = Memory::new();
        let mut register = Register::new();
        register.set_accumulator(0x42);

        let mut message_bus = MessageBus::new(&mut memory);

        let cycles = pha.call(vec![0x00], &mut register, &mut message_bus);

        assert_eq!(memory.read_byte(0x01ff), 0x42);
        assert_eq!(0x42, register.a());
        assert_eq!(0b0011_0000, register.p());
        assert_eq!(cycles, 3);
    }

    #[test]
    #[should_panic]
    fn test_invalid_opcode() {
        let pha = Pha::new(0x00);
        let arguments = vec![0xFF];
        let mut memory = Memory::new();
        let mut message_bus = MessageBus::new(&mut memory);
        let mut register = Register::new();

        pha.call(arguments, &mut register, &mut message_bus);
    }
}


