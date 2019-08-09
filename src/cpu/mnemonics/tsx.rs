/*
TSX  Transfer Stack Pointer to Index X

     SP -> X                          N Z C I D V
                                      + + - - - -

     addressing    assembler    opc  bytes  cyles
     --------------------------------------------
     implied       TSX           BA    1     2
*/

use crate::cpu::mnemonics::Mnemonic;
use crate::cpu::register::Register;
use crate::message_bus::MessageBus;

#[derive(Debug)]
pub struct Tsx {
    mnemonic: String,
    opcode: u8
}

impl Tsx {
    pub fn new(opcode: u8) -> Tsx {
        return Tsx { mnemonic: "TSX".to_string(), opcode: opcode };
    }
}

impl Mnemonic for Tsx {
    fn determine_bytes(&self) -> usize {
        return match self.opcode {
            0xBA => 1,
            _ => panic!("Invalid opcode `0x{:x}` for mnemonic {}", self.opcode, self.mnemonic)
        }
    }

    fn call(&self, _arguments: Vec<u8>, register: &mut Register, _message_bus: &mut MessageBus) -> u8 {
        match self.opcode {
            0xBA => return self.call_implied(register, _message_bus),
            _ => panic!("Invalid opcode `0x{:x}` for mnemonic {}", self.opcode, self.mnemonic)
        }
    }

    fn call_implied(&self, register: &mut Register, _message_bus: &mut MessageBus) -> u8 {
        let result = register.s();
        register.set_x(result);
        register.calculate_nz_bits(result);

        return 2;
    }
}

#[cfg(test)]
mod tests {
    use super::Tsx;
    use crate::cpu::mnemonics::Mnemonic;
    use crate::cpu::register::Register;
    use crate::memory::Memory;
    use crate::message_bus::MessageBus;

    #[test]
    fn test_implied() {
        let tsx = Tsx::new(0xBA);
        let mut memory = Memory::new();
        let mut register = Register::new();
        register.pull_s();
        register.pull_s();

        let mut message_bus = MessageBus::new(&mut memory);

        let cycles = tsx.call(vec![0x00], &mut register, &mut message_bus);

        assert_eq!(register.x(), 0x01);
        assert_eq!(0b0011_0000, register.p());
        assert_eq!(cycles, 2);
    }

    #[test]
    fn test_implied_with_zero() {
        let tsx = Tsx::new(0xBA);
        let mut memory = Memory::new();
        let mut register = Register::new();
        register.pull_s();

        let mut message_bus = MessageBus::new(&mut memory);

        let cycles = tsx.call(vec![0x00], &mut register, &mut message_bus);

        assert_eq!(register.x(), 0x00);
        assert_eq!(0b0011_0010, register.p());
        assert_eq!(cycles, 2);
    }

    #[test]
    fn test_implied_with_negative() {
        let tsx = Tsx::new(0xBA);
        let mut memory = Memory::new();
        let mut register = Register::new();
        register.push_s();

        let mut message_bus = MessageBus::new(&mut memory);

        let cycles = tsx.call(vec![0x00], &mut register, &mut message_bus);

        assert_eq!(register.x(), 0xFE);
        assert_eq!(0b1011_0000, register.p());
        assert_eq!(cycles, 2);
    }

    #[test]
    #[should_panic]
    fn test_invalid_opcode() {
        let tsx = Tsx::new(0x00);
        let arguments = vec![0xFF];
        let mut memory = Memory::new();
        let mut message_bus = MessageBus::new(&mut memory);
        let mut register = Register::new();

        tsx.call(arguments, &mut register, &mut message_bus);
    }
}


