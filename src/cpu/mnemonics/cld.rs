/*
CLD  Clear Decimal Mode

     0 -> D                           N Z C I D V
                                      - - - - 0 -

     addressing    assembler    opc  bytes  cyles
     --------------------------------------------
     implied       CLD           D8    1     2
*/

use crate::cpu::mnemonics::Mnemonic;
use crate::cpu::register::Register;
use crate::message_bus::MessageBus;

#[derive(Debug)]
pub struct Cld {
    mnemonic: String,
    opcode: u8
}

impl Cld {
    pub fn new(opcode: u8) -> Cld {
        return Cld { mnemonic: "CLD".to_string(), opcode: opcode };
    }
}

impl Mnemonic for Cld {
    fn determine_bytes(&self) -> usize {
        return match self.opcode {
            0xD8 => 1,
            _ => panic!("Invalid opcode `0x{:x}` for mnemonic {}", self.opcode, self.mnemonic)
        }
    }

    fn call(&self, _arguments: Vec<u8>, register: &mut Register, _message_bus: &mut MessageBus) -> u8 {
        match self.opcode {
            0xD8 => return self.call_implied(register, _message_bus),
            _ => panic!("Invalid opcode `0x{:x}` for mnemonic {}", self.opcode, self.mnemonic)
        }
    }

    fn call_implied(&self, register: &mut Register, _message_bus: &mut MessageBus) -> u8 {
        register.set_decimal_bit(false);
        return 2;
    }
}

#[cfg(test)]
mod tests {
    use super::Cld;
    use crate::cpu::mnemonics::Mnemonic;
    use crate::cpu::register::Register;
    use crate::memory::Memory;
    use crate::message_bus::MessageBus;

    #[test]
    fn test_implied() {
        let cld = Cld::new(0xD8);
        let mut memory = Memory::new();
        let mut register = Register::new();
        register.set_decimal_bit(true);

        let mut message_bus = MessageBus::new(&mut memory);

        let cycles = cld.call(vec![0x00], &mut register, &mut message_bus);

        assert_eq!(0b0010_0000, register.p());
        assert_eq!(cycles, 2);
    }

    #[test]
    #[should_panic]
    fn test_invalid_opcode() {
        let cld = Cld::new(0x00);
        let arguments = vec![0xFF];
        let mut memory = Memory::new();
        let mut message_bus = MessageBus::new(&mut memory);
        let mut register = Register::new();

        cld.call(arguments, &mut register, &mut message_bus);
    }
}


