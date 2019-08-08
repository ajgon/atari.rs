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
mod bvc;
mod bvs;
mod clc;
mod cld;
mod cli;
mod clv;
mod cmp;
mod cpx;
mod cpy;
mod dec;
mod dex;
mod dey;
mod eor;
mod inc;
mod inx;
mod iny;
mod jmp;
mod jsr;
mod lda;
mod ldx;
mod ldy;
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
use bvc::Bvc;
use bvs::Bvs;
use clc::Clc;
use cld::Cld;
use cli::Cli;
use clv::Clv;
use cmp::Cmp;
use cpx::Cpx;
use cpy::Cpy;
use dec::Dec;
use dex::Dex;
use dey::Dey;
use eor::Eor;
use inc::Inc;
use inx::Inx;
use iny::Iny;
use jmp::Jmp;
use jsr::Jsr;
use lda::Lda;
use ldx::Ldx;
use ldy::Ldy;
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
    fn call_zero_page_y(&self, arguments: Vec<u8>, register: &mut Register, message_bus: &mut MessageBus) -> u8 { 0 }
    fn call_absolute(&self, arguments: Vec<u8>, register: &mut Register, message_bus: &mut MessageBus) -> u8 { 0 }
    fn call_absolute_x(&self, arguments: Vec<u8>, register: &mut Register, message_bus: &mut MessageBus) -> u8 { 0 }
    fn call_absolute_y(&self, arguments: Vec<u8>, register: &mut Register, message_bus: &mut MessageBus) -> u8 { 0 }
    fn call_indirect(&self, arguments: Vec<u8>, register: &mut Register, message_bus: &mut MessageBus) -> u8 { 0 }
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
            0x50 => Box::new(Bvc::new(opcode)),
            0x70 => Box::new(Bvs::new(opcode)),
            0x18 => Box::new(Clc::new(opcode)),
            0xD8 => Box::new(Cld::new(opcode)),
            0x58 => Box::new(Cli::new(opcode)),
            0xB8 => Box::new(Clv::new(opcode)),
            0xC9 | 0xC5 | 0xD5 | 0xCD | 0xDD | 0xD9 | 0xC1 | 0xD1 => Box::new(Cmp::new(opcode)),
            0xE0 | 0xE4 | 0xEC => Box::new(Cpx::new(opcode)),
            0xC0 | 0xC4 | 0xCC => Box::new(Cpy::new(opcode)),
            0xC6 | 0xD6 | 0xCE | 0xDE => Box::new(Dec::new(opcode)),
            0xCA => Box::new(Dex::new(opcode)),
            0x88 => Box::new(Dey::new(opcode)),
            0x49 | 0x45 | 0x55 | 0x4d | 0x5d | 0x59 | 0x41 | 0x51 => Box::new(Eor::new(opcode)),
            0xE6 | 0xF6 | 0xEE | 0xFE => Box::new(Inc::new(opcode)),
            0xE8 => Box::new(Inx::new(opcode)),
            0xC8 => Box::new(Iny::new(opcode)),
            0x4C | 0x6C => Box::new(Jmp::new(opcode)),
            0x20 => Box::new(Jsr::new(opcode)),
            0xA9 | 0xA5 | 0xB5 | 0xAD | 0xBD | 0xB9 | 0xA1 | 0xB1 => Box::new(Lda::new(opcode)),
            0xA2 | 0xA6 | 0xB6 | 0xAE | 0xBE => Box::new(Ldx::new(opcode)),
            0xA0 | 0xA4 | 0xB4 | 0xAC | 0xBC => Box::new(Ldy::new(opcode)),
            _ => panic!("Unknown opcode numnber: 0x#{:x}", opcode)
        }
    }
}
