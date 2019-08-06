mod adc;
mod and;
mod asl;
mod bcc;
mod bcs;
mod beq;
mod bit;
mod bmi;
mod bne;
mod bpl;
mod brk;
use adc::Adc;
use and::And;
use asl::Asl;
use bcc::Bcc;
use bcs::Bcs;
use beq::Beq;
use bit::Bit;
use bmi::Bmi;
use bne::Bne;
use bpl::Bpl;
use brk::Brk;
use crate::cpu::register::Register;
use crate::message_bus::MessageBus;

#[allow(unused_variables)]
pub trait Mnemonic {
    fn determine_bytes(&self) -> usize;
    fn call(&self, arguments: Vec<u8>, register: &mut Register, message_bus: &mut MessageBus) -> u8;
    fn call_implied(&self, register: &mut Register, message_bus: &mut MessageBus) -> u8 { 0 }
    fn call_accumulator(&self, register: &mut Register) -> u8 { 0 }
    fn call_relative(&self, arguments: Vec<u8>, register: &mut Register) -> u8 { 0 }
    fn call_immidiate(&self, arguments: Vec<u8>, register: &mut Register) -> u8 { 0 }
    fn call_zero_page(&self, arguments: Vec<u8>, register: &mut Register, message_bus: &mut MessageBus) -> u8 { 0 }
    fn call_zero_page_x(&self, arguments: Vec<u8>, register: &mut Register, message_bus: &mut MessageBus) -> u8 { 0 }
    fn call_absolute(&self, arguments: Vec<u8>, register: &mut Register, message_bus: &mut MessageBus) -> u8 { 0 }
    fn call_absolute_x(&self, arguments: Vec<u8>, register: &mut Register, message_bus: &mut MessageBus) -> u8 { 0 }
    fn call_absolute_y(&self, arguments: Vec<u8>, register: &mut Register, message_bus: &mut MessageBus) -> u8 { 0 }
    fn call_indirect_x(&self, arguments: Vec<u8>, register: &mut Register, message_bus: &mut MessageBus) -> u8 { 0 }
    fn call_indirect_y(&self, arguments: Vec<u8>, register: &mut Register, message_bus: &mut MessageBus) -> u8 { 0 }
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
            0x0A | 0x06 | 0x16 | 0x0E | 0x1E => Box::new(Asl::new(opcode)),
            0x90 => Box::new(Bcc::new(opcode)),
            0xB0 => Box::new(Bcs::new(opcode)),
            0xF0 => Box::new(Beq::new(opcode)),
            0x24 | 0x2C => Box::new(Bit::new(opcode)),
            0x30 => Box::new(Bmi::new(opcode)),
            0xD0 => Box::new(Bne::new(opcode)),
            0x10 => Box::new(Bpl::new(opcode)),
            0x00 => Box::new(Brk::new(opcode)),
            _ => panic!("Unknown opcode numnber: 0x#{:x}", opcode)
        }
    }
}
