mod adc;
mod and;
use adc::Adc;
use and::And;
use crate::cpu::register::Register;
use crate::message_bus::MessageBus;

pub trait Mnemonic {
    fn determine_bytes(&self) -> usize;
    fn call(&self, arguments: Vec<u8>, register: &mut Register, message_bus: &MessageBus) -> u8;
    fn call_immidiate(&self, arguments: Vec<u8>, register: &mut Register) -> u8;
    fn call_zero_page(&self, arguments: Vec<u8>, register: &mut Register, message_bus: &MessageBus) -> u8;
    fn call_zero_page_x(&self, arguments: Vec<u8>, register: &mut Register, message_bus: &MessageBus) -> u8;
    fn call_absolute(&self, arguments: Vec<u8>, register: &mut Register, message_bus: &MessageBus) -> u8;
    fn call_absolute_x(&self, arguments: Vec<u8>, register: &mut Register, message_bus: &MessageBus) -> u8;
    fn call_absolute_y(&self, arguments: Vec<u8>, register: &mut Register, message_bus: &MessageBus) -> u8;
    fn call_indirect_x(&self, arguments: Vec<u8>, register: &mut Register, message_bus: &MessageBus) -> u8;
    fn call_indirect_y(&self, arguments: Vec<u8>, register: &mut Register, message_bus: &MessageBus) -> u8;
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
            0x69 | 0x65 | 0x75 | 0x6d | 0x7d | 0x79 | 0x61 | 0x71 => Box::new(Adc::new(opcode)),
            0x29 | 0x25 | 0x35 | 0x2d | 0x3d | 0x39 | 0x21 | 0x31 => Box::new(And::new(opcode)),
            _ => panic!("Unknown opcode numnber: 0x#{:x}", opcode)
        }
    }
}
