/*
JSR  Jump to New Location Saving Return Address

     push (PC+2),                     N Z C I D V
     (PC+1) -> PCL                    - - - - - -
     (PC+2) -> PCH

     addressing    assembler    opc  bytes  cyles
     --------------------------------------------
     absolute      JSR oper      20    3     6
*/

use crate::cpu::addressing;
use crate::cpu::mnemonics::Mnemonic;
use crate::cpu::register::Register;
use crate::message_bus::MessageBus;

#[derive(Debug)]
pub struct Jsr {
    mnemonic: String,
    opcode: u8
}

impl Jsr {
    pub fn new(opcode: u8) -> Jsr {
        return Jsr { mnemonic: "JSR".to_string(), opcode: opcode };
    }
}

impl Mnemonic for Jsr {
    fn determine_bytes(&self) -> usize {
        return match self.opcode {
            0x20 => 3,
            _ => panic!("Invalid opcode `0x{:x}` for mnemonic {}", self.opcode, self.mnemonic)
        }
    }

    fn call(&self, arguments: Vec<u8>, register: &mut Register, message_bus: &mut MessageBus) -> u8 {
        match self.opcode {
            0x20 => return self.call_absolute(arguments, register, message_bus),
            _ => panic!("Invalid opcode `0x{:x}` for mnemonic {}", self.opcode, self.mnemonic)
        }
    }

    fn call_absolute(&self, arguments: Vec<u8>, register: &mut Register, message_bus: &mut MessageBus) -> u8 {
        let (memory_address, _memory_value, _boundary_crossed) = addressing::absolute(arguments, message_bus);
        register.increment_pc_by(2);
        addressing::stack_push(((register.pc() & 0xFF00) >> 8) as u8, message_bus, register);
        addressing::stack_push((register.pc() & 0x00FF) as u8, message_bus, register);

        register.set_pc(memory_address);

        return 6;
    }
}

#[cfg(test)]
mod tests {
    use super::Jsr;
    use crate::cpu::mnemonics::Mnemonic;
    use crate::cpu::register::Register;
    use crate::memory::Memory;
    use crate::message_bus::MessageBus;

    #[test]
    fn test_absolute() {
        let jsr = Jsr::new(0x20);
        let arguments = vec![0x50, 0x06];
        let mut memory = Memory::new();
        let mut register = Register::new();
        let mut message_bus = MessageBus::new(&mut memory);

        let cycles = jsr.call(arguments, &mut register, &mut message_bus);

        assert_eq!(memory.read_byte(0x1ff), 0x06);
        assert_eq!(memory.read_byte(0x1fe), 0x02);
        assert_eq!(register.pc(), 0x0650);
        assert_eq!(register.p(), 0b0010_0000);
        assert_eq!(cycles, 6);
    }

    #[test]
    #[should_panic]
    fn test_invalid_opcode() {
        let jsr = Jsr::new(0x00);
        let arguments = vec![0xFF];
        let mut memory = Memory::new();
        let mut message_bus = MessageBus::new(&mut memory);
        let mut register = Register::new();

        jsr.call(arguments, &mut register, &mut message_bus);
    }
}



