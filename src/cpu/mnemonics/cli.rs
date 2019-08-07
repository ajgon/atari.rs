/*
CLI  Clear Interrupt Disable Bit

     0 -> I                           N Z C I D V
                                      - - - 0 - -

     addressing    assembler    opc  bytes  cyles
     --------------------------------------------
     implied       CLI           58    1     2
*/

use crate::cpu::mnemonics::Mnemonic;
use crate::cpu::register::Register;
use crate::message_bus::MessageBus;

#[derive(Debug)]
pub struct Cli {
    mnemonic: String,
    opcode: u8
}

impl Cli {
    pub fn new(opcode: u8) -> Cli {
        return Cli { mnemonic: "CLI".to_string(), opcode: opcode };
    }
}

impl Mnemonic for Cli {
    fn determine_bytes(&self) -> usize {
        return match self.opcode {
            0x58 => 1,
            _ => panic!("Invalid opcode `0x{:x}` for mnemonic {}", self.opcode, self.mnemonic)
        }
    }

    fn call(&self, _arguments: Vec<u8>, register: &mut Register, _message_bus: &mut MessageBus) -> u8 {
        match self.opcode {
            0x58 => return self.call_implied(register, _message_bus),
            _ => panic!("Invalid opcode `0x{:x}` for mnemonic {}", self.opcode, self.mnemonic)
        }
    }

    fn call_implied(&self, register: &mut Register, _message_bus: &mut MessageBus) -> u8 {
        register.set_interrupt_bit(false);
        return 2;
    }
}

#[cfg(test)]
mod tests {
    use super::Cli;
    use crate::cpu::mnemonics::Mnemonic;
    use crate::cpu::register::Register;
    use crate::memory::Memory;
    use crate::message_bus::MessageBus;

    #[test]
    fn test_implied() {
        let cli = Cli::new(0x58);
        let mut memory = Memory::new();
        let mut register = Register::new();
        register.set_interrupt_bit(true);

        let mut message_bus = MessageBus::new(&mut memory);

        let cycles = cli.call(vec![0x00], &mut register, &mut message_bus);

        assert_eq!(0b0011_0000, register.p());
        assert_eq!(cycles, 2);
    }

    #[test]
    #[should_panic]
    fn test_invalid_opcode() {
        let cli = Cli::new(0x00);
        let arguments = vec![0xFF];
        let mut memory = Memory::new();
        let mut message_bus = MessageBus::new(&mut memory);
        let mut register = Register::new();

        cli.call(arguments, &mut register, &mut message_bus);
    }
}



