/*
NOP  No Operation

     ---                              N Z C I D V
                                      - - - - - -

     addressing    assembler    opc  bytes  cyles
     --------------------------------------------
     implied       NOP           EA    1     2
*/

use crate::cpu::mnemonics::Mnemonic;
use crate::cpu::register::Register;
use crate::message_bus::MessageBus;

#[derive(Debug)]
pub struct Nop {
    mnemonic: String,
    opcode: u8
}

impl Nop {
    pub fn new(opcode: u8) -> Nop {
        return Nop { mnemonic: "NOP".to_string(), opcode: opcode };
    }
}

impl Mnemonic for Nop {
    fn determine_bytes(&self) -> usize {
        return match self.opcode {
            0xEA => 1,
            _ => panic!("Invalid opcode `0x{:x}` for mnemonic {}", self.opcode, self.mnemonic)
        }
    }

    fn call(&self, _arguments: Vec<u8>, register: &mut Register, _message_bus: &mut MessageBus) -> u8 {
        match self.opcode {
            0xEA => return self.call_implied(register, _message_bus),
            _ => panic!("Invalid opcode `0x{:x}` for mnemonic {}", self.opcode, self.mnemonic)
        }
    }

    fn call_implied(&self, _register: &mut Register, _message_bus: &mut MessageBus) -> u8 {
        return 2;
    }
}

#[cfg(test)]
mod tests {
    use super::Nop;
    use crate::cpu::mnemonics::Mnemonic;
    use crate::cpu::register::Register;
    use crate::memory::Memory;
    use crate::message_bus::MessageBus;

    #[test]
    fn test_implied() {
        let nop = Nop::new(0xEA);
        let mut memory = Memory::new();
        let mut register = Register::new();
        let mut message_bus = MessageBus::new(&mut memory);

        let cycles = nop.call(vec![0x00], &mut register, &mut message_bus);

        assert_eq!(cycles, 2);
    }

    #[test]
    #[should_panic]
    fn test_invalid_opcode() {
        let nop = Nop::new(0x00);
        let arguments = vec![0xFF];
        let mut memory = Memory::new();
        let mut message_bus = MessageBus::new(&mut memory);
        let mut register = Register::new();

        nop.call(arguments, &mut register, &mut message_bus);
    }
}


