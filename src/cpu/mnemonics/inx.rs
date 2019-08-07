/*
INX  Increment Index X by One

     X + 1 -> X                       N Z C I D V
                                      + + - - - -

     addressing    assembler    opc  bytes  cyles
     --------------------------------------------
     implied       INX           E8    1     2
*/

use crate::cpu::mnemonics::Mnemonic;
use crate::cpu::register::Register;
use crate::message_bus::MessageBus;
use crate::cpu::alu;

#[derive(Debug)]
pub struct Inx {
    mnemonic: String,
    opcode: u8
}

impl Inx {
    pub fn new(opcode: u8) -> Inx {
        return Inx { mnemonic: "INX".to_string(), opcode: opcode };
    }
}

impl Mnemonic for Inx {
    fn determine_bytes(&self) -> usize {
        return match self.opcode {
            0xE8 => 1,
            _ => panic!("Invalid opcode `0x{:x}` for mnemonic {}", self.opcode, self.mnemonic)
        }
    }

    fn call(&self, _arguments: Vec<u8>, register: &mut Register, _message_bus: &mut MessageBus) -> u8 {
        match self.opcode {
            0xE8 => return self.call_implied(register, _message_bus),
            _ => panic!("Invalid opcode `0x{:x}` for mnemonic {}", self.opcode, self.mnemonic)
        }
    }

    fn call_implied(&self, register: &mut Register, _message_bus: &mut MessageBus) -> u8 {
        let result = alu::increment(register.x(), register);

        register.set_x(result);

        return 2;
    }
}

#[cfg(test)]
mod tests {
    use super::Inx;
    use crate::cpu::mnemonics::Mnemonic;
    use crate::cpu::register::Register;
    use crate::memory::Memory;
    use crate::message_bus::MessageBus;

    #[test]
    fn test_implied() {
        let inx = Inx::new(0xE8);
        let mut memory = Memory::new();
        let mut register = Register::new();
        register.set_x(0xFF);

        let mut message_bus = MessageBus::new(&mut memory);

        let cycles = inx.call(vec![0x00], &mut register, &mut message_bus);

        assert_eq!(0x00, register.x());
        assert_eq!(0b0011_0010, register.p());
        assert_eq!(cycles, 2);
    }

    #[test]
    #[should_panic]
    fn test_invalid_opcode() {
        let inx = Inx::new(0x00);
        let arguments = vec![0xFF];
        let mut memory = Memory::new();
        let mut message_bus = MessageBus::new(&mut memory);
        let mut register = Register::new();

        inx.call(arguments, &mut register, &mut message_bus);
    }
}



