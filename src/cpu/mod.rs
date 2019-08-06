mod mnemonics;
mod register;
mod addressing;
mod alu;

use mnemonics::Mnemonics;
use register::Register;
use crate::message_bus::MessageBus;

#[derive(Debug)]
pub struct Cpu<'a> {
    message_bus: &'a MessageBus<'a>,
    current_instruction: Vec<u8>,
    register: register::Register,
    mnemonics: Mnemonics
}

impl<'a> Cpu<'a> {
    pub fn new(message_bus: &'a MessageBus<'a>) -> Cpu<'a> {
        return Cpu {
            message_bus: message_bus,
            current_instruction: Vec::new(),
            register: Register::new(),
            mnemonics: Mnemonics::new()
        };
    }

    pub fn process_byte(&mut self, byte: u8) {
        let opcode = if self.current_instruction.len() > 0 { self.current_instruction[0] } else { byte };
        let mnemonic = self.mnemonics.resolve_mnemonic_from_opcode(opcode);
        let bytes = mnemonic.determine_bytes();

        self.current_instruction.push(byte);

        if self.current_instruction.len() == bytes {
            self.process_instruction();
            self.current_instruction.clear();
        }
    }

    fn process_instruction(&mut self) {
        let mnemonic = self.mnemonics.resolve_mnemonic_from_opcode(self.current_instruction[0]);
        let parameters = &self.current_instruction[1..];

        mnemonic.call(parameters.to_vec(), &mut self.register, self.message_bus);
    }


}
