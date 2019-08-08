/*
JMP  Jump to New Location

     (PC+1) -> PCL                    N Z C I D V
     (PC+2) -> PCH                    - - - - - -

     addressing    assembler    opc  bytes  cyles
     --------------------------------------------
     absolute      JMP oper      4C    3     3
     indirect      JMP (oper)    6C    3     5
*/

use crate::cpu::addressing;
use crate::cpu::mnemonics::Mnemonic;
use crate::cpu::register::Register;
use crate::message_bus::MessageBus;

#[derive(Debug)]
pub struct Jmp {
    mnemonic: String,
    opcode: u8
}

impl Jmp {
    pub fn new(opcode: u8) -> Jmp {
        return Jmp { mnemonic: "JMP".to_string(), opcode: opcode };
    }
}

impl Mnemonic for Jmp {
    fn determine_bytes(&self) -> usize {
        return match self.opcode {
            0x4C => 3,
            0x6C => 3,
            _ => panic!("Invalid opcode `0x{:x}` for mnemonic {}", self.opcode, self.mnemonic)
        }
    }

    fn call(&self, arguments: Vec<u8>, register: &mut Register, message_bus: &mut MessageBus) -> u8 {
        match self.opcode {
            0x4C => return self.call_absolute(arguments, register, message_bus),
            0x6C => return self.call_indirect(arguments, register, message_bus),
            _ => panic!("Invalid opcode `0x{:x}` for mnemonic {}", self.opcode, self.mnemonic)
        }
    }

    fn call_absolute(&self, arguments: Vec<u8>, register: &mut Register, message_bus: &mut MessageBus) -> u8 {
        let (memory_address, _memory_value, _boundary_crossed) = addressing::absolute(arguments, message_bus);

        register.set_pc(memory_address);

        return 3;
    }

    fn call_indirect(&self, arguments: Vec<u8>, register: &mut Register, message_bus: &mut MessageBus) -> u8 {
        let high_address_byte = arguments[1];
        let low_address_byte = arguments[0];
        let (_memory_address, low_pc_value, _boundary_crossed) = addressing::absolute(vec![low_address_byte, high_address_byte], message_bus);

        // 6502 has a well known bug in JMP. When fetching indirectly new PC address, only low byte
        // of the address is increased (and overflowing) without affecting the high byte. Thus:
        // JMP $3000 - will fetch PCL from $3000, and PCH from $3001 (which is okay)
        // but
        // JMP $30FF - will fetch PCL from $30FF, but PCH will be fetched from $3000 not $3100
        // (only low byte overflows, without affecting the high one).
        let low_address_byte = low_address_byte.overflowing_add(1).0;
        let (_memory_address, high_pc_value, _boundary_crossed) = addressing::absolute(vec![low_address_byte, high_address_byte], message_bus);
        let memory_address = ((high_pc_value as u16) << 8) + low_pc_value as u16;

        register.set_pc(memory_address);

        return 5;
    }
}

#[cfg(test)]
mod tests {
    use super::Jmp;
    use crate::cpu::mnemonics::Mnemonic;
    use crate::cpu::register::Register;
    use crate::memory::Memory;
    use crate::message_bus::MessageBus;

    #[test]
    fn test_absolute() {
        let jmp = Jmp::new(0x4C);
        let arguments = vec![0x50, 0x06];
        let mut memory = Memory::new();
        let mut register = Register::new();
        let mut message_bus = MessageBus::new(&mut memory);

        let cycles = jmp.call(arguments, &mut register, &mut message_bus);

        assert_eq!(register.pc(), 0x0650);
        assert_eq!(register.p(), 0b0011_0000);
        assert_eq!(cycles, 3);
    }

    #[test]
    fn test_indirect() {
        let jmp = Jmp::new(0x6C);
        let arguments = vec![0x00, 0x30];
        let mut memory = Memory::new();
        memory.write_byte(0x3000, 0x50);
        memory.write_byte(0x3001, 0x06);
        let mut register = Register::new();
        let mut message_bus = MessageBus::new(&mut memory);

        let cycles = jmp.call(arguments, &mut register, &mut message_bus);

        assert_eq!(register.pc(), 0x0650);
        assert_eq!(register.p(), 0b0011_0000);
        assert_eq!(cycles, 5);
    }

    #[test]
    fn test_indirect_out_of_bonds() {
        let jmp = Jmp::new(0x6C);
        let arguments = vec![0xFF, 0x30];
        let mut memory = Memory::new();
        memory.write_byte(0x3000, 0x06);
        memory.write_byte(0x30FF, 0x50);
        memory.write_byte(0x3100, 0x42);
        let mut register = Register::new();
        let mut message_bus = MessageBus::new(&mut memory);

        let cycles = jmp.call(arguments, &mut register, &mut message_bus);

        assert_eq!(register.pc(), 0x0650);
        assert_eq!(register.p(), 0b0011_0000);
        assert_eq!(cycles, 5);
    }

    #[test]
    #[should_panic]
    fn test_invalid_opcode() {
        let jmp = Jmp::new(0x00);
        let arguments = vec![0xFF];
        let mut memory = Memory::new();
        let mut message_bus = MessageBus::new(&mut memory);
        let mut register = Register::new();

        jmp.call(arguments, &mut register, &mut message_bus);
    }
}


