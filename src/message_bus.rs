use crate::memory::Memory;

pub enum MessageBusTarget {
    Memory
}

pub enum MessageBusMessage {
    Read
    //Write
}

pub trait ProcessMessage {
    fn process_message(&self, message: MessageBusMessage, argument: u16) -> u8;
}

#[derive(Debug)]
pub struct MessageBus<'a> {
    memory: &'a Memory
}

impl<'a> MessageBus<'a> {
    pub fn new(memory: &Memory) -> MessageBus {
        return MessageBus {
            memory: memory
        };
    }

    pub fn send_message(&self, target: MessageBusTarget, message: MessageBusMessage, argument: u16) -> u8 {
        return match target {
            MessageBusTarget::Memory => self.memory.process_message(message, argument)
        };
    }
}

