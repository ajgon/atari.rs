/*
BMI  Branch on Result Minus

     branch on N = 1                  N Z C I D V
                                      - - - - - -

     addressing    assembler    opc  bytes  cyles
     --------------------------------------------
     relative      BMI oper      30    2     2**
*/

use crate::cpu::mnemonics::Mnemonic;
use crate::cpu::register::Register;
use crate::message_bus::MessageBus;

#[derive(Debug)]
pub struct Bmi {
    mnemonic: String,
    opcode: u8
}

impl Bmi {
    pub fn new(opcode: u8) -> Bmi {
        return Bmi { mnemonic: "BMI".to_string(), opcode: opcode };
    }
}

impl Mnemonic for Bmi {
    fn determine_bytes(&self) -> usize {
        return match self.opcode {
            0x30 => 2,
            _ => panic!("Invalid opcode `0x{:x}` for mnemonic {}", self.opcode, self.mnemonic)
        }
    }

    fn call(&self, arguments: Vec<u8>, register: &mut Register, _message_bus: &MessageBus) -> u8 {
        match self.opcode {
            0x30 => return self.call_relative(arguments, register),
            _ => panic!("Invalid opcode `0x{:x}` for mnemonic {}", self.opcode, self.mnemonic)
        }
    }

    fn call_relative(&self, arguments: Vec<u8>, register: &mut Register) -> u8 {
        if !register.negative_bit() {
            return 2;
        }

        let previous_pc_value = register.pc();
        register.increment_pc_by(arguments[0] as u16);

        return if previous_pc_value & 0xFF00 == register.pc() & 0xFF00 { 3 } else { 4 };
    }
}

#[cfg(test)]
mod tests {
    use super::Bmi;
    use crate::cpu::mnemonics::Mnemonic;
    use crate::cpu::register::Register;
    use crate::memory::Memory;
    use crate::message_bus::MessageBus;

    #[test]
    fn test_relative() {
        let bmi = Bmi::new(0x30);
        let arguments = vec![0x02];
        let memory = Memory::new();
        let mut register = Register::new();
        register.set_negative_bit(true);

        let message_bus = MessageBus::new(&memory);

        let cycles = bmi.call(arguments, &mut register, &message_bus);

        assert_eq!(0x0602, register.pc());
        assert_eq!(cycles, 3);
    }

    #[test]
    fn test_relative_out_of_bonds() {
        let bmi = Bmi::new(0x30);
        let arguments = vec![0x85];
        let memory = Memory::new();
        let mut register = Register::new();
        register.set_negative_bit(true);
        register.increment_pc_by(0x80);

        let message_bus = MessageBus::new(&memory);

        let cycles = bmi.call(arguments, &mut register, &message_bus);

        assert_eq!(0x0705, register.pc());
        assert_eq!(cycles, 4);
    }

    #[test]
    fn test_relative_with_negative_bit_unset() {
        let bmi = Bmi::new(0x30);
        let arguments = vec![0x02];
        let memory = Memory::new();
        let mut register = Register::new();

        let message_bus = MessageBus::new(&memory);

        let cycles = bmi.call(arguments, &mut register, &message_bus);

        assert_eq!(0x0600, register.pc());
        assert_eq!(cycles, 2);
    }

    #[test]
    #[should_panic]
    fn test_invalid_opcode() {
        let bmi = Bmi::new(0x00);
        let arguments = vec![0xFF];
        let memory = Memory::new();
        let message_bus = MessageBus::new(&memory);
        let mut register = Register::new();

        bmi.call(arguments, &mut register, &message_bus);
    }
}

