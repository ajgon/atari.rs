/*
INY  Increment Index Y by One

     Y + 1 -> Y                       N Z C I D V
                                      + + - - - -

     addressing    assembler    opc  bytes  cyles
     --------------------------------------------
     implied       INY           C8    1     2
*/

use crate::cpu::mnemonics::Mnemonic;
use crate::cpu::register::Register;
use crate::message_bus::MessageBus;
use crate::cpu::alu;

#[derive(Debug)]
pub struct Iny {
    mnemonic: String,
    opcode: u8
}

impl Iny {
    pub fn new(opcode: u8) -> Iny {
        return Iny { mnemonic: "INY".to_string(), opcode: opcode };
    }
}

impl Mnemonic for Iny {
    fn determine_bytes(&self) -> usize {
        return match self.opcode {
            0xC8 => 1,
            _ => panic!("Invalid opcode `0x{:x}` for mnemonic {}", self.opcode, self.mnemonic)
        }
    }

    fn call(&self, _arguments: Vec<u8>, register: &mut Register, _message_bus: &mut MessageBus) -> u8 {
        match self.opcode {
            0xC8 => return self.call_implied(register, _message_bus),
            _ => panic!("Invalid opcode `0x{:x}` for mnemonic {}", self.opcode, self.mnemonic)
        }
    }

    fn call_implied(&self, register: &mut Register, _message_bus: &mut MessageBus) -> u8 {
        let result = alu::increment(register.y(), register);

        register.set_y(result);

        return 2;
    }
}

#[cfg(test)]
mod tests {
    use super::Iny;
    use crate::cpu::mnemonics::Mnemonic;
    use crate::cpu::register::Register;
    use crate::memory::Memory;
    use crate::message_bus::MessageBus;

    #[test]
    fn test_implied() {
        let iny = Iny::new(0xC8);
        let mut memory = Memory::new();
        let mut register = Register::new();
        register.set_y(0xFF);

        let mut message_bus = MessageBus::new(&mut memory);

        let cycles = iny.call(vec![0x00], &mut register, &mut message_bus);

        assert_eq!(0x00, register.y());
        assert_eq!(0b0010_0010, register.p());
        assert_eq!(cycles, 2);
    }

    #[test]
    #[should_panic]
    fn test_invalid_opcode() {
        let iny = Iny::new(0x00);
        let arguments = vec![0xFF];
        let mut memory = Memory::new();
        let mut message_bus = MessageBus::new(&mut memory);
        let mut register = Register::new();

        iny.call(arguments, &mut register, &mut message_bus);
    }
}




