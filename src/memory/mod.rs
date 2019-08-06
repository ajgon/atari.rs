use crate::message_bus::ProcessMessage;
use crate::message_bus::MessageBusMessage;

#[derive(Debug)]
pub struct Memory {
    contents: Vec<u8>
}

impl Memory {
    pub fn new() -> Memory {
        return Memory { contents: vec![0; 65536] }
    }

    pub fn read_byte(&self, address: u16) -> u8 {
        return self.contents[address as usize];
    }

    pub fn write_byte(&mut self, address: u16, value: u8) {
        self.contents[address as usize] = value;
    }
}

impl ProcessMessage for Memory {
    fn process_message(&self, _message: MessageBusMessage, argument: u16) -> u8 {
        return self.read_byte(argument);
    }
}

#[cfg(test)]
mod tests {
    use super::Memory;

    #[test]
    fn test_reader() {
        assert_eq!(Memory::new().read_byte(44), 0);
    }

    #[test]
    fn test_writer() {
        let mut memory = Memory::new();
        memory.write_byte(42, 99);

        assert_eq!(memory.read_byte(42), 99);
    }
}


