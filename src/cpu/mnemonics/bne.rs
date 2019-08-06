/*
BNE  Branch on Result not Zero

     branch on Z = 0                  N Z C I D V
                                      - - - - - -

     addressing    assembler    opc  bytes  cyles
     --------------------------------------------
     relative      BNE oper      D0    2     2**
*/

use crate::cpu::mnemonics::Mnemonic;
use crate::cpu::register::Register;
use crate::message_bus::MessageBus;

#[derive(Debug)]
pub struct Bne {
    mnemonic: String,
    opcode: u8
}

impl Bne {
    pub fn new(opcode: u8) -> Bne {
        return Bne { mnemonic: "BNE".to_string(), opcode: opcode };
    }
}

impl Mnemonic for Bne {
    fn determine_bytes(&self) -> usize {
        return match self.opcode {
            0xD0 => 2,
            _ => panic!("Invalid opcode `0x{:x}` for mnemonic {}", self.opcode, self.mnemonic)
        }
    }

    fn call(&self, arguments: Vec<u8>, register: &mut Register, _message_bus: &mut MessageBus) -> u8 {
        match self.opcode {
            0xD0 => return self.call_relative(arguments, register),
            _ => panic!("Invalid opcode `0x{:x}` for mnemonic {}", self.opcode, self.mnemonic)
        }
    }

    fn call_relative(&self, arguments: Vec<u8>, register: &mut Register) -> u8 {
        if register.zero_bit() {
            return 2;
        }

        let previous_pc_value = register.pc();
        register.increment_pc_by(arguments[0] as u16);

        return if previous_pc_value & 0xFF00 == register.pc() & 0xFF00 { 3 } else { 4 };
    }
}

#[cfg(test)]
mod tests {
    use super::Bne;
    use crate::cpu::mnemonics::Mnemonic;
    use crate::cpu::register::Register;
    use crate::memory::Memory;
    use crate::message_bus::MessageBus;

    #[test]
    fn test_relative() {
        let bne = Bne::new(0xD0);
        let arguments = vec![0x02];
        let mut memory = Memory::new();
        let mut register = Register::new();

        let mut message_bus = MessageBus::new(&mut memory);

        let cycles = bne.call(arguments, &mut register, &mut message_bus);

        assert_eq!(0x0602, register.pc());
        assert_eq!(cycles, 3);
    }

    #[test]
    fn test_relative_out_of_bonds() {
        let bne = Bne::new(0xD0);
        let arguments = vec![0x85];
        let mut memory = Memory::new();
        let mut register = Register::new();
        register.increment_pc_by(0x80);

        let mut message_bus = MessageBus::new(&mut memory);

        let cycles = bne.call(arguments, &mut register, &mut message_bus);

        assert_eq!(0x0705, register.pc());
        assert_eq!(cycles, 4);
    }

    #[test]
    fn test_relative_with_zero_bit_set() {
        let bne = Bne::new(0xD0);
        let arguments = vec![0x02];
        let mut memory = Memory::new();
        let mut register = Register::new();
        register.set_zero_bit(true);

        let mut message_bus = MessageBus::new(&mut memory);

        let cycles = bne.call(arguments, &mut register, &mut message_bus);

        assert_eq!(0x0600, register.pc());
        assert_eq!(cycles, 2);
    }

    #[test]
    #[should_panic]
    fn test_invalid_opcode() {
        let bne = Bne::new(0x00);
        let arguments = vec![0xFF];
        let mut memory = Memory::new();
        let mut message_bus = MessageBus::new(&mut memory);
        let mut register = Register::new();

        bne.call(arguments, &mut register, &mut message_bus);
    }
}


