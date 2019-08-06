/*
BPL  Branch on Result Plus

     branch on N = 0                  N Z C I D V
                                      - - - - - -

     addressing    assembler    opc  bytes  cyles
     --------------------------------------------
     relative      BPL oper      10    2     2**
*/

use crate::cpu::mnemonics::Mnemonic;
use crate::cpu::register::Register;
use crate::message_bus::MessageBus;

#[derive(Debug)]
pub struct Bpl {
    mnemonic: String,
    opcode: u8
}

impl Bpl {
    pub fn new(opcode: u8) -> Bpl {
        return Bpl { mnemonic: "BPL".to_string(), opcode: opcode };
    }
}

impl Mnemonic for Bpl {
    fn determine_bytes(&self) -> usize {
        return match self.opcode {
            0x10 => 2,
            _ => panic!("Invalid opcode `0x{:x}` for mnemonic {}", self.opcode, self.mnemonic)
        }
    }

    fn call(&self, arguments: Vec<u8>, register: &mut Register, _message_bus: &MessageBus) -> u8 {
        match self.opcode {
            0x10 => return self.call_relative(arguments, register),
            _ => panic!("Invalid opcode `0x{:x}` for mnemonic {}", self.opcode, self.mnemonic)
        }
    }

    fn call_relative(&self, arguments: Vec<u8>, register: &mut Register) -> u8 {
        if register.negative_bit() {
            return 2;
        }

        let previous_pc_value = register.pc();
        register.increment_pc_by(arguments[0] as u16);

        return if previous_pc_value & 0xFF00 == register.pc() & 0xFF00 { 3 } else { 4 };
    }
}

#[cfg(test)]
mod tests {
    use super::Bpl;
    use crate::cpu::mnemonics::Mnemonic;
    use crate::cpu::register::Register;
    use crate::memory::Memory;
    use crate::message_bus::MessageBus;

    #[test]
    fn test_relative() {
        let bpl = Bpl::new(0x10);
        let arguments = vec![0x02];
        let memory = Memory::new();
        let mut register = Register::new();

        let message_bus = MessageBus::new(&memory);

        let cycles = bpl.call(arguments, &mut register, &message_bus);

        assert_eq!(0x0602, register.pc());
        assert_eq!(cycles, 3);
    }

    #[test]
    fn test_relative_out_of_bonds() {
        let bpl = Bpl::new(0x10);
        let arguments = vec![0x85];
        let memory = Memory::new();
        let mut register = Register::new();
        register.increment_pc_by(0x80);

        let message_bus = MessageBus::new(&memory);

        let cycles = bpl.call(arguments, &mut register, &message_bus);

        assert_eq!(0x0705, register.pc());
        assert_eq!(cycles, 4);
    }

    #[test]
    fn test_relative_with_negative_bit_set() {
        let bpl = Bpl::new(0x10);
        let arguments = vec![0x02];
        let memory = Memory::new();
        let mut register = Register::new();
        register.set_negative_bit(true);

        let message_bus = MessageBus::new(&memory);

        let cycles = bpl.call(arguments, &mut register, &message_bus);

        assert_eq!(0x0600, register.pc());
        assert_eq!(cycles, 2);
    }

    #[test]
    #[should_panic]
    fn test_invalid_opcode() {
        let bpl = Bpl::new(0x00);
        let arguments = vec![0xFF];
        let memory = Memory::new();
        let message_bus = MessageBus::new(&memory);
        let mut register = Register::new();

        bpl.call(arguments, &mut register, &message_bus);
    }
}


