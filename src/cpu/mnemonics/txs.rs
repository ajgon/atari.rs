/*
TXS  Transfer Index X to Stack Register

     X -> SP                          N Z C I D V
                                      - - - - - -

     addressing    assembler    opc  bytes  cyles
     --------------------------------------------
     implied       TXS           9A    1     2
*/

use crate::cpu::mnemonics::Mnemonic;
use crate::cpu::register::Register;
use crate::message_bus::MessageBus;

#[derive(Debug)]
pub struct Txs {
    mnemonic: String,
    opcode: u8
}

impl Txs {
    pub fn new(opcode: u8) -> Txs {
        return Txs { mnemonic: "TXS".to_string(), opcode: opcode };
    }
}

impl Mnemonic for Txs {
    fn determine_bytes(&self) -> usize {
        return match self.opcode {
            0x9A => 1,
            _ => panic!("Invalid opcode `0x{:x}` for mnemonic {}", self.opcode, self.mnemonic)
        }
    }

    fn call(&self, _arguments: Vec<u8>, register: &mut Register, _message_bus: &mut MessageBus) -> u8 {
        match self.opcode {
            0x9A => return self.call_implied(register, _message_bus),
            _ => panic!("Invalid opcode `0x{:x}` for mnemonic {}", self.opcode, self.mnemonic)
        }
    }

    fn call_implied(&self, register: &mut Register, _message_bus: &mut MessageBus) -> u8 {
        let result = register.x();
        register.set_s(result);

        return 2;
    }
}

#[cfg(test)]
mod tests {
    use super::Txs;
    use crate::cpu::mnemonics::Mnemonic;
    use crate::cpu::register::Register;
    use crate::memory::Memory;
    use crate::message_bus::MessageBus;

    #[test]
    fn test_implied() {
        let txs = Txs::new(0x9A);
        let mut memory = Memory::new();
        let mut register = Register::new();
        register.set_x(0x42);

        let mut message_bus = MessageBus::new(&mut memory);

        let cycles = txs.call(vec![0x00], &mut register, &mut message_bus);

        assert_eq!(register.s(), 0x42);
        assert_eq!(0b0010_0000, register.p());
        assert_eq!(cycles, 2);
    }

    #[test]
    fn test_implied_with_zero() {
        let txs = Txs::new(0x9A);
        let mut memory = Memory::new();
        let mut register = Register::new();
        register.set_x(0x00);

        let mut message_bus = MessageBus::new(&mut memory);

        let cycles = txs.call(vec![0x00], &mut register, &mut message_bus);

        assert_eq!(register.s(), 0x00);
        assert_eq!(0b0010_0000, register.p());
        assert_eq!(cycles, 2);
    }

    #[test]
    fn test_implied_with_negative() {
        let txs = Txs::new(0x9A);
        let mut memory = Memory::new();
        let mut register = Register::new();
        register.set_x(0xF2);

        let mut message_bus = MessageBus::new(&mut memory);

        let cycles = txs.call(vec![0x00], &mut register, &mut message_bus);

        assert_eq!(register.s(), 0xF2);
        assert_eq!(0b0010_0000, register.p());
        assert_eq!(cycles, 2);
    }

    #[test]
    #[should_panic]
    fn test_invalid_opcode() {
        let txs = Txs::new(0x00);
        let arguments = vec![0xFF];
        let mut memory = Memory::new();
        let mut message_bus = MessageBus::new(&mut memory);
        let mut register = Register::new();

        txs.call(arguments, &mut register, &mut message_bus);
    }
}



