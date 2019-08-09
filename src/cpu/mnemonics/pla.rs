/*
PLA  Pull Accumulator from Stack

     pull A                           N Z C I D V
                                      + + - - - -

     addressing    assembler    opc  bytes  cyles
     --------------------------------------------
     implied       PLA           68    1     4
*/

use crate::cpu::addressing;
use crate::cpu::mnemonics::Mnemonic;
use crate::cpu::register::Register;
use crate::message_bus::MessageBus;

#[derive(Debug)]
pub struct Pla {
    mnemonic: String,
    opcode: u8
}

impl Pla {
    pub fn new(opcode: u8) -> Pla {
        return Pla { mnemonic: "PLA".to_string(), opcode: opcode };
    }
}

impl Mnemonic for Pla {
    fn determine_bytes(&self) -> usize {
        return match self.opcode {
            0x68 => 1,
            _ => panic!("Invalid opcode `0x{:x}` for mnemonic {}", self.opcode, self.mnemonic)
        }
    }

    fn call(&self, _arguments: Vec<u8>, register: &mut Register, _message_bus: &mut MessageBus) -> u8 {
        match self.opcode {
            0x68 => return self.call_implied(register, _message_bus),
            _ => panic!("Invalid opcode `0x{:x}` for mnemonic {}", self.opcode, self.mnemonic)
        }
    }

    fn call_implied(&self, register: &mut Register, _message_bus: &mut MessageBus) -> u8 {
        let result = addressing::stack_pull(_message_bus, register);
        register.set_accumulator(result);
        register.calculate_nz_bits(result);

        return 4;
    }
}

#[cfg(test)]
mod tests {
    use super::Pla;
    use crate::cpu::mnemonics::Mnemonic;
    use crate::cpu::register::Register;
    use crate::memory::Memory;
    use crate::message_bus::MessageBus;

    #[test]
    fn test_implied() {
        let pla = Pla::new(0x68);
        let mut memory = Memory::new();
        memory.write_byte(0x01ff, 0x42);

        let mut register = Register::new();
        register.push_s();

        let mut message_bus = MessageBus::new(&mut memory);

        let cycles = pla.call(vec![0x00], &mut register, &mut message_bus);

        assert_eq!(0x42, register.a());
        assert_eq!(0b0010_0000, register.p());
        assert_eq!(cycles, 4);
    }

    #[test]
    fn test_implied_with_zero() {
        let pla = Pla::new(0x68);
        let mut memory = Memory::new();
        memory.write_byte(0x01ff, 0x00);

        let mut register = Register::new();
        register.push_s();

        let mut message_bus = MessageBus::new(&mut memory);

        let cycles = pla.call(vec![0x00], &mut register, &mut message_bus);

        assert_eq!(0x00, register.a());
        assert_eq!(0b0010_0010, register.p());
        assert_eq!(cycles, 4);
    }

    #[test]
    fn test_implied_with_negative_bit() {
        let pla = Pla::new(0x68);
        let mut memory = Memory::new();
        memory.write_byte(0x01ff, 0xF2);

        let mut register = Register::new();
        register.push_s();

        let mut message_bus = MessageBus::new(&mut memory);

        let cycles = pla.call(vec![0x00], &mut register, &mut message_bus);

        assert_eq!(0xF2, register.a());
        assert_eq!(0b1010_0000, register.p());
        assert_eq!(cycles, 4);
    }

    #[test]
    #[should_panic]
    fn test_invalid_opcode() {
        let pla = Pla::new(0x00);
        let arguments = vec![0xFF];
        let mut memory = Memory::new();
        let mut message_bus = MessageBus::new(&mut memory);
        let mut register = Register::new();

        pla.call(arguments, &mut register, &mut message_bus);
    }
}




