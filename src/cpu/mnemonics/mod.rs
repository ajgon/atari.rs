mod adc;
use adc::Adc;
use crate::cpu::register::Register;

pub trait Mnemonic {
    fn determine_bytes_and_cycles(&self) -> (usize, u8);
    fn call(&self, arguments: Vec<u8>, register: &mut Register);
    fn call_immidiate(&self, arguments: Vec<u8>, register: &mut Register);
}

#[derive(Debug)]
pub struct Mnemonics {
}

impl Mnemonics {
    pub fn new() -> Mnemonics {
        return Mnemonics {}
    }

    pub fn resolve_mnemonic_from_opcode(&self, opcode: u8) -> Box<Mnemonic> {
        return match opcode {
            0x69 => Box::new(Adc::new(opcode)),
            _ => panic!("Unknown opcode numnber: 0x#{:x}", opcode)
        }
    }
}
