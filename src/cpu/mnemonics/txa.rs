/*
TXA  Transfer Index X to Accumulator

     X -> A                           N Z C I D V
                                      + + - - - -

     addressing    assembler    opc  bytes  cyles
     --------------------------------------------
     implied       TXA           8A    1     2
*/

use crate::cpu::mnemonics::Mnemonic;
use crate::cpu::register::Register;
use crate::message_bus::MessageBus;

#[derive(Debug)]
pub struct Txa {
    mnemonic: String,
    opcode: u8
}

impl Txa {
    pub fn new(opcode: u8) -> Txa {
        return Txa { mnemonic: "TXA".to_string(), opcode: opcode };
    }
}

impl Mnemonic for Txa {
    fn determine_bytes(&self) -> usize {
        return match self.opcode {
            0x8A => 1,
            _ => panic!("Invalid opcode `0x{:x}` for mnemonic {}", self.opcode, self.mnemonic)
        }
    }

    fn call(&self, _arguments: Vec<u8>, register: &mut Register, _message_bus: &mut MessageBus) -> u8 {
        match self.opcode {
            0x8A => return self.call_implied(register, _message_bus),
            _ => panic!("Invalid opcode `0x{:x}` for mnemonic {}", self.opcode, self.mnemonic)
        }
    }

    fn call_implied(&self, register: &mut Register, _message_bus: &mut MessageBus) -> u8 {
        let result = register.x();
        register.set_accumulator(result);
        register.calculate_nz_bits(result);

        return 2;
    }
}

#[cfg(test)]
mod tests {
    use super::Txa;
    use crate::cpu::mnemonics::Mnemonic;
    use crate::cpu::register::Register;
    use crate::memory::Memory;
    use crate::message_bus::MessageBus;

    #[test]
    fn test_implied() {
        let txa = Txa::new(0x8A);
        let mut memory = Memory::new();
        let mut register = Register::new();
        register.set_x(0x42);

        let mut message_bus = MessageBus::new(&mut memory);

        let cycles = txa.call(vec![0x00], &mut register, &mut message_bus);

        assert_eq!(register.a(), 0x42);
        assert_eq!(0b0011_0000, register.p());
        assert_eq!(cycles, 2);
    }

    #[test]
    fn test_implied_with_zero() {
        let txa = Txa::new(0x8A);
        let mut memory = Memory::new();
        let mut register = Register::new();
        register.set_x(0x00);

        let mut message_bus = MessageBus::new(&mut memory);

        let cycles = txa.call(vec![0x00], &mut register, &mut message_bus);

        assert_eq!(register.a(), 0x00);
        assert_eq!(0b0011_0010, register.p());
        assert_eq!(cycles, 2);
    }

    #[test]
    fn test_implied_with_negative() {
        let txa = Txa::new(0x8A);
        let mut memory = Memory::new();
        let mut register = Register::new();
        register.set_x(0xF2);

        let mut message_bus = MessageBus::new(&mut memory);

        let cycles = txa.call(vec![0x00], &mut register, &mut message_bus);

        assert_eq!(register.a(), 0xF2);
        assert_eq!(0b1011_0000, register.p());
        assert_eq!(cycles, 2);
    }

    #[test]
    #[should_panic]
    fn test_invalid_opcode() {
        let txa = Txa::new(0x00);
        let arguments = vec![0xFF];
        let mut memory = Memory::new();
        let mut message_bus = MessageBus::new(&mut memory);
        let mut register = Register::new();

        txa.call(arguments, &mut register, &mut message_bus);
    }
}
