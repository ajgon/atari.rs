use crate::memory::Memory;

pub enum MessageBusTarget {
    Memory
}

pub enum MessageBusMessage {
    Read,
    Write
}

pub trait ProcessMessage {
    fn process_message(&mut self, message: MessageBusMessage, arguments: Vec<u16>) -> u8;
}

#[derive(Debug)]
pub struct MessageBus<'a> {
    memory: &'a mut Memory
}

impl<'a> MessageBus<'a> {
    pub fn new(memory: &mut Memory) -> MessageBus {
        return MessageBus {
            memory: memory
        };
    }

    pub fn send_message(&mut self, target: MessageBusTarget, message: MessageBusMessage, arguments: Vec<u16>) -> u8 {
        return match target {
            MessageBusTarget::Memory => self.memory.process_message(message, arguments)
        };
    }
}

