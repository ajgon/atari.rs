/*
PLP  Pull Processor Status from Stack

     pull SR                          N Z C I D V
                                      from stack

     addressing    assembler    opc  bytes  cyles
     --------------------------------------------
     implied       PLP           28    1     4
*/

use crate::cpu::addressing;
use crate::cpu::mnemonics::Mnemonic;
use crate::cpu::register::Register;
use crate::message_bus::MessageBus;

#[derive(Debug)]
pub struct Plp {
    mnemonic: String,
    opcode: u8
}

impl Plp {
    pub fn new(opcode: u8) -> Plp {
        return Plp { mnemonic: "PLP".to_string(), opcode: opcode };
    }
}

impl Mnemonic for Plp {
    fn determine_bytes(&self) -> usize {
        return match self.opcode {
            0x28 => 1,
            _ => panic!("Invalid opcode `0x{:x}` for mnemonic {}", self.opcode, self.mnemonic)
        }
    }

    fn call(&self, _arguments: Vec<u8>, register: &mut Register, _message_bus: &mut MessageBus) -> u8 {
        match self.opcode {
            0x28 => return self.call_implied(register, _message_bus),
            _ => panic!("Invalid opcode `0x{:x}` for mnemonic {}", self.opcode, self.mnemonic)
        }
    }

    fn call_implied(&self, register: &mut Register, _message_bus: &mut MessageBus) -> u8 {
        let result = addressing::stack_pull(_message_bus, register);
        register.set_p(result);

        return 4;
    }
}

#[cfg(test)]
mod tests {
    use super::Plp;
    use crate::cpu::mnemonics::Mnemonic;
    use crate::cpu::register::Register;
    use crate::memory::Memory;
    use crate::message_bus::MessageBus;

    #[test]
    fn test_implied() {
        let plp = Plp::new(0x28);
        let mut memory = Memory::new();
        memory.write_byte(0x01ff, 0b1001_0001);

        let mut register = Register::new();
        register.push_s();

        let mut message_bus = MessageBus::new(&mut memory);

        let cycles = plp.call(vec![0x00], &mut register, &mut message_bus);

        assert_eq!(0b1011_0001, register.p());
        assert_eq!(cycles, 4);
    }

    #[test]
    #[should_panic]
    fn test_invalid_opcode() {
        let plp = Plp::new(0x00);
        let arguments = vec![0xFF];
        let mut memory = Memory::new();
        let mut message_bus = MessageBus::new(&mut memory);
        let mut register = Register::new();

        plp.call(arguments, &mut register, &mut message_bus);
    }
}





