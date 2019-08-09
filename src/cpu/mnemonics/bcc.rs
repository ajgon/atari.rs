/*
BCC  Branch on Carry Clear

     branch on C = 0                  N Z C I D V
                                      - - - - - -

     addressing    assembler    opc  bytes  cyles
     --------------------------------------------
     relative      BCC oper      90    2     2**
*/

use crate::cpu::mnemonics::Mnemonic;
use crate::cpu::register::Register;
use crate::message_bus::MessageBus;

#[derive(Debug)]
pub struct Bcc {
    mnemonic: String,
    opcode: u8
}

impl Bcc {
    pub fn new(opcode: u8) -> Bcc {
        return Bcc { mnemonic: "BCC".to_string(), opcode: opcode };
    }
}

impl Mnemonic for Bcc {
    fn determine_bytes(&self) -> usize {
        return match self.opcode {
            0x90 => 2,
            _ => panic!("Invalid opcode `0x{:x}` for mnemonic {}", self.opcode, self.mnemonic)
        }
    }

    fn call(&self, arguments: Vec<u8>, register: &mut Register, _message_bus: &mut MessageBus) -> u8 {
        match self.opcode {
            0x90 => return self.call_relative(arguments, register),
            _ => panic!("Invalid opcode `0x{:x}` for mnemonic {}", self.opcode, self.mnemonic)
        }
    }

    fn call_relative(&self, arguments: Vec<u8>, register: &mut Register) -> u8 {
        if register.carry_bit() {
            return 2;
        }

        let previous_pc_value = register.pc();
        let rel: i16 = if arguments[0] > 0x7f { arguments[0] as i16 - 0x100 } else { arguments[0] as i16 };
        register.set_pc((register.pc() as i16 + rel) as u16);

        return if previous_pc_value & 0xFF00 == register.pc() & 0xFF00 { 3 } else { 4 };
    }
}

#[cfg(test)]
mod tests {
    use super::Bcc;
    use crate::cpu::mnemonics::Mnemonic;
    use crate::cpu::register::Register;
    use crate::memory::Memory;
    use crate::message_bus::MessageBus;

    #[test]
    fn test_relative() {
        let bcc = Bcc::new(0x90);
        let arguments = vec![0x02];
        let mut memory = Memory::new();
        let mut register = Register::new();

        let mut message_bus = MessageBus::new(&mut memory);

        let cycles = bcc.call(arguments, &mut register, &mut message_bus);

        assert_eq!(0x0602, register.pc());
        assert_eq!(cycles, 3);
    }

    #[test]
    fn test_relative_negative() {
        let bcc = Bcc::new(0x90);
        let arguments = vec![0xF0];
        let mut memory = Memory::new();
        let mut register = Register::new();

        let mut message_bus = MessageBus::new(&mut memory);

        let cycles = bcc.call(arguments, &mut register, &mut message_bus);

        assert_eq!(0x05F0, register.pc());
        assert_eq!(cycles, 4);
    }

    #[test]
    fn test_relative_out_of_bonds() {
        let bcc = Bcc::new(0x90);
        let arguments = vec![0x7f];
        let mut memory = Memory::new();
        let mut register = Register::new();
        register.increment_pc_by(0x86);

        let mut message_bus = MessageBus::new(&mut memory);

        let cycles = bcc.call(arguments, &mut register, &mut message_bus);

        assert_eq!(0x0705, register.pc());
        assert_eq!(cycles, 4);
    }

    #[test]
    fn test_relative_with_carry_bit_set() {
        let bcc = Bcc::new(0x90);
        let arguments = vec![0x02];
        let mut memory = Memory::new();
        let mut register = Register::new();
        register.set_carry_bit(true);

        let mut message_bus = MessageBus::new(&mut memory);

        let cycles = bcc.call(arguments, &mut register, &mut message_bus);

        assert_eq!(0x0600, register.pc());
        assert_eq!(cycles, 2);
    }

    #[test]
    #[should_panic]
    fn test_invalid_opcode() {
        let bcc = Bcc::new(0x00);
        let arguments = vec![0xFF];
        let mut memory = Memory::new();
        let mut message_bus = MessageBus::new(&mut memory);
        let mut register = Register::new();

        bcc.call(arguments, &mut register, &mut message_bus);
    }
}


