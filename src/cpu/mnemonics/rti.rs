/*
RTI  Return from Interrupt

     pull SR, pull PC                 N Z C I D V
                                      from stack

     addressing    assembler    opc  bytes  cyles
     --------------------------------------------
     implied       RTI           40    1     6
*/

use crate::cpu::addressing;
use crate::cpu::mnemonics::Mnemonic;
use crate::cpu::register::Register;
use crate::message_bus::MessageBus;

#[derive(Debug)]
pub struct Rti {
    mnemonic: String,
    opcode: u8
}

impl Rti {
    pub fn new(opcode: u8) -> Rti {
        return Rti { mnemonic: "RTI".to_string(), opcode: opcode };
    }
}

impl Mnemonic for Rti {
    fn determine_bytes(&self) -> usize {
        return match self.opcode {
            0x40 => 1,
            _ => panic!("Invalid opcode `0x{:x}` for mnemonic {}", self.opcode, self.mnemonic)
        }
    }

    fn call(&self, _arguments: Vec<u8>, register: &mut Register, _message_bus: &mut MessageBus) -> u8 {
        match self.opcode {
            0x40 => return self.call_implied(register, _message_bus),
            _ => panic!("Invalid opcode `0x{:x}` for mnemonic {}", self.opcode, self.mnemonic)
        }
    }

    fn call_implied(&self, register: &mut Register, _message_bus: &mut MessageBus) -> u8 {
        let p = addressing::stack_pull(_message_bus, register);
        let pcl = addressing::stack_pull(_message_bus, register);
        let pch = addressing::stack_pull(_message_bus, register);

        register.set_p(p);
        register.set_break_bit(false);
        register.set_pc(((pch as u16) << 8) + pcl as u16);

        return 6;
    }
}

#[cfg(test)]
mod tests {
    use super::Rti;
    use crate::cpu::mnemonics::Mnemonic;
    use crate::cpu::register::Register;
    use crate::memory::Memory;
    use crate::message_bus::MessageBus;

    #[test]
    fn test_implied() {
        let rti = Rti::new(0x40);
        let mut memory = Memory::new();
        memory.write_byte(0x01FF, 0x06);
        memory.write_byte(0x01FE, 0x55);
        memory.write_byte(0x01FD, 0b1011_0011);

        let mut register = Register::new();
        register.push_s();
        register.push_s();
        register.push_s();

        let mut message_bus = MessageBus::new(&mut memory);

        let cycles = rti.call(vec![0x00], &mut register, &mut message_bus);

        assert_eq!(0b1010_0011, register.p());
        assert_eq!(0x0655, register.pc());
        assert_eq!(cycles, 6);
    }

    #[test]
    #[should_panic]
    fn test_invalid_opcode() {
        let rti = Rti::new(0x00);
        let arguments = vec![0xFF];
        let mut memory = Memory::new();
        let mut message_bus = MessageBus::new(&mut memory);
        let mut register = Register::new();

        rti.call(arguments, &mut register, &mut message_bus);
    }
}


