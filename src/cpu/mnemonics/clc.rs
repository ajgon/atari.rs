/*
CLC  Clear Carry Flag

     0 -> C                           N Z C I D V
                                      - - 0 - - -

     addressing    assembler    opc  bytes  cyles
     --------------------------------------------
     implied       CLC           18    1     2
*/

use crate::cpu::mnemonics::Mnemonic;
use crate::cpu::register::Register;
use crate::message_bus::MessageBus;

#[derive(Debug)]
pub struct Clc {
    mnemonic: String,
    opcode: u8
}

impl Clc {
    pub fn new(opcode: u8) -> Clc {
        return Clc { mnemonic: "CLC".to_string(), opcode: opcode };
    }
}

impl Mnemonic for Clc {
    fn determine_bytes(&self) -> usize {
        return match self.opcode {
            0x18 => 1,
            _ => panic!("Invalid opcode `0x{:x}` for mnemonic {}", self.opcode, self.mnemonic)
        }
    }

    fn call(&self, _arguments: Vec<u8>, register: &mut Register, _message_bus: &mut MessageBus) -> u8 {
        match self.opcode {
            0x18 => return self.call_implied(register, _message_bus),
            _ => panic!("Invalid opcode `0x{:x}` for mnemonic {}", self.opcode, self.mnemonic)
        }
    }

    fn call_implied(&self, register: &mut Register, _message_bus: &mut MessageBus) -> u8 {
        register.set_carry_bit(false);
        return 2;
    }
}

#[cfg(test)]
mod tests {
    use super::Clc;
    use crate::cpu::mnemonics::Mnemonic;
    use crate::cpu::register::Register;
    use crate::memory::Memory;
    use crate::message_bus::MessageBus;

    #[test]
    fn test_implied() {
        let clc = Clc::new(0x18);
        let mut memory = Memory::new();
        let mut register = Register::new();
        register.set_carry_bit(true);

        let mut message_bus = MessageBus::new(&mut memory);

        let cycles = clc.call(vec![0x00], &mut register, &mut message_bus);

        assert_eq!(0b0011_0000, register.p());
        assert_eq!(cycles, 2);
    }

    #[test]
    #[should_panic]
    fn test_invalid_opcode() {
        let clc = Clc::new(0x00);
        let arguments = vec![0xFF];
        let mut memory = Memory::new();
        let mut message_bus = MessageBus::new(&mut memory);
        let mut register = Register::new();

        clc.call(arguments, &mut register, &mut message_bus);
    }
}
