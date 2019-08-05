/*
ADC  Add Memory to Accumulator with Carry

     A + M + C -> A, C                N Z C I D V
                                      + + + - - +

     addressing    assembler    opc  bytes  cyles
     --------------------------------------------
     immidiate     ADC #oper     69    2     2
     zeropage      ADC oper      65    2     3
     zeropage,X    ADC oper,X    75    2     4
     absolute      ADC oper      6D    3     4
     absolute,X    ADC oper,X    7D    3     4*
     absolute,Y    ADC oper,Y    79    3     4*
     (indirect,X)  ADC (oper,X)  61    2     6
     (indirect),Y  ADC (oper),Y  71    2     5*

*   16-bit address words are little endian, lo(w)-byte first, followed by the hi(gh)-byte.
(An assembler will use a human readable, big-endian notation as in $HHLL.)

*/

use crate::cpu::mnemonics::Mnemonic;
use crate::cpu::register::Register;
use crate::message_bus::MessageBus;
use crate::message_bus::MessageBusTarget;
use crate::message_bus::MessageBusMessage;
use crate::cpu::alu::add;

#[derive(Debug)]
pub struct Adc {
    mnemonic: String,
    opcode: u8
}

impl Adc {
    pub fn new(opcode: u8) -> Adc {
        return Adc { mnemonic: "ADC".to_string(), opcode: opcode };
    }
}

impl Mnemonic for Adc {
    fn determine_bytes_and_cycles(&self) -> (usize, u8) {
        // @todo implement cycles change when page boundary is crossed
        return match self.opcode {
            0x69 => (2, 2),
            0x65 => (2, 3),
            0x75 => (2, 4),
            0x6D => (3, 4),
            0x7D => (3, 4),
            0x79 => (3, 4),
            0x61 => (2, 6),
            0x71 => (2, 5),
            _ => panic!("Invalid opcode `0x{:x}` for mnemonic {}", self.opcode, self.mnemonic)
        }
    }

    // @todo return number of cycles used (including out of bounds)
    fn call(&self, arguments: Vec<u8>, register: &mut Register, message_bus: &MessageBus) {
        match self.opcode {
            0x69 => self.call_immidiate(arguments, register),
            0x65 => self.call_zero_page(arguments, register, &message_bus),
            0x75 => self.call_zero_page_x(arguments, register, &message_bus),
            0x6D => self.call_absolute(arguments, register, &message_bus),
            0x7D => self.call_absolute_x(arguments, register, &message_bus),
            0x79 => self.call_absolute_y(arguments, register, &message_bus),
            0x61 => self.call_indirect_x(arguments, register, &message_bus),
            0x71 => self.call_indirect_y(arguments, register, &message_bus),
            _ => panic!("Invalid opcode `0x{:x}` for mnemonic {}", self.opcode, self.mnemonic)
        }
    }

    fn call_immidiate(&self, arguments: Vec<u8>, register: &mut Register) {
        add(arguments[0], register);
    }

    fn call_zero_page(&self, arguments: Vec<u8>, register: &mut Register, message_bus: &MessageBus) {
        let memory_value = message_bus.send_message(MessageBusTarget::Memory, MessageBusMessage::Read, arguments[0] as u16);
        add(memory_value, register);
    }

    fn call_zero_page_x(&self, arguments: Vec<u8>, register: &mut Register, message_bus: &MessageBus) {
        let memory_address = arguments[0].overflowing_add(register.x()).0 as u16;
        let memory_value = message_bus.send_message(
            MessageBusTarget::Memory, MessageBusMessage::Read, memory_address
        );
        add(memory_value, register);
    }

    fn call_absolute(&self, arguments: Vec<u8>, register: &mut Register, message_bus: &MessageBus) {
        let memory_address: u16 = ((arguments[1] as u16) << 8) + arguments[0] as u16;
        let memory_value = message_bus.send_message(
            MessageBusTarget::Memory, MessageBusMessage::Read, memory_address
        );
        add(memory_value, register);
    }

    fn call_absolute_x(&self, arguments: Vec<u8>, register: &mut Register, message_bus: &MessageBus) {
        let memory_address: u16 = ((arguments[1] as u16) << 8) + arguments[0] as u16;
        let memory_address: u16 = memory_address.overflowing_add(register.x() as u16).0;
        let memory_value = message_bus.send_message(
            MessageBusTarget::Memory, MessageBusMessage::Read, memory_address
        );
        add(memory_value, register);
    }

    fn call_absolute_y(&self, arguments: Vec<u8>, register: &mut Register, message_bus: &MessageBus) {
        let memory_address: u16 = ((arguments[1] as u16) << 8) + arguments[0] as u16;
        let memory_address: u16 = memory_address.overflowing_add(register.y() as u16).0;
        let memory_value = message_bus.send_message(
            MessageBusTarget::Memory, MessageBusMessage::Read, memory_address
        );
        add(memory_value, register);
    }

    fn call_indirect_x(&self, arguments: Vec<u8>, register: &mut Register, message_bus: &MessageBus) {
        let memory_address: u16 = (arguments[0] as u16).overflowing_add(register.x() as u16).0;
        let memory_value = message_bus.send_message(
            MessageBusTarget::Memory, MessageBusMessage::Read, memory_address
        );
        let new_memory_address: u16 = memory_value as u16;
        let memory_value = message_bus.send_message(
            MessageBusTarget::Memory, MessageBusMessage::Read, memory_address.overflowing_add(1).0
        );
        let new_memory_address: u16 = new_memory_address + ((memory_value as u16) << 8);
        let memory_value = message_bus.send_message(
            MessageBusTarget::Memory, MessageBusMessage::Read, new_memory_address
        );
        add(memory_value, register);
    }

    fn call_indirect_y(&self, arguments: Vec<u8>, register: &mut Register, message_bus: &MessageBus) {
        let memory_address: u16 = arguments[0] as u16;
        let memory_value = message_bus.send_message(
            MessageBusTarget::Memory, MessageBusMessage::Read, memory_address
        );
        let new_memory_address: u16 = memory_value as u16;
        let memory_value = message_bus.send_message(
            MessageBusTarget::Memory, MessageBusMessage::Read, memory_address.overflowing_add(1).0
        );
        let new_memory_address: u16 = new_memory_address + ((memory_value as u16) << 8);
        let new_memory_address: u16 = new_memory_address.overflowing_add(register.y() as u16).0;
        let memory_value = message_bus.send_message(
            MessageBusTarget::Memory, MessageBusMessage::Read, new_memory_address
        );
        add(memory_value, register);
    }
}

#[cfg(test)]
mod tests {
    use super::Adc;
    use crate::cpu::mnemonics::Mnemonic;
    use crate::cpu::register::Register;
    use crate::memory::Memory;
    use crate::message_bus::MessageBus;

    #[test]
    fn test_immidiate() {
        let adc = Adc::new(0x69);
        let arguments = vec![0x42];
        let mut memory = Memory::new();
        let mut register = Register::new();
        register.set_accumulator(0x02);
        register.set_carry_bit(true);

        let mut message_bus = MessageBus::new(&memory);

        adc.call(arguments, &mut register, &message_bus);

        assert_eq!(register.a(), 0x45);
        assert_eq!(register.p(), 0b00110000);
    }

    #[test]
    fn test_zero_page() {
        let adc = Adc::new(0x65);
        let arguments = vec![0x30];
        let mut memory = Memory::new();
        memory.write_byte(0x30, 0x42);

        let mut register = Register::new();
        register.set_accumulator(0x02);
        register.set_carry_bit(true);

        let mut message_bus = MessageBus::new(&memory);

        adc.call(arguments, &mut register, &message_bus);

        assert_eq!(register.a(), 0x45);
        assert_eq!(register.p(), 0b00110000);
    }

    #[test]
    fn test_zero_page_x() {
        let adc = Adc::new(0x75);
        let arguments = vec![0x30];
        let mut memory = Memory::new();
        memory.write_byte(0x35, 0x42);

        let mut register = Register::new();
        register.set_accumulator(0x02);
        register.set_carry_bit(true);
        register.set_x(0x05);

        let mut message_bus = MessageBus::new(&memory);

        adc.call(arguments, &mut register, &message_bus);

        assert_eq!(register.a(), 0x45);
        assert_eq!(register.p(), 0b00110000);
    }

    #[test]
    fn test_zero_page_x_out_of_bounds() {
        let adc = Adc::new(0x75);
        let arguments = vec![0xff];
        let mut memory = Memory::new();
        memory.write_byte(0x35, 0x42);
        memory.write_byte(0x135, 0x27);

        let mut register = Register::new();
        register.set_accumulator(0x02);
        register.set_x(0x36);

        let mut message_bus = MessageBus::new(&memory);

        adc.call(arguments, &mut register, &message_bus);

        assert_eq!(register.a(), 0x44);
        assert_eq!(register.p(), 0b00110000);
    }

    #[test]
    fn test_absolute() {
        let adc = Adc::new(0x6D);
        let arguments = vec![0x3c, 0x5a];
        let mut memory = Memory::new();
        memory.write_byte(0x5a3c, 0x42);

        let mut register = Register::new();
        register.set_accumulator(0x02);
        register.set_carry_bit(true);

        let mut message_bus = MessageBus::new(&memory);

        adc.call(arguments, &mut register, &message_bus);

        assert_eq!(register.a(), 0x45);
        assert_eq!(register.p(), 0b00110000);
    }

    #[test]
    fn test_absolute_x() {
        let adc = Adc::new(0x7D);
        let arguments = vec![0x3c, 0x5a];
        let mut memory = Memory::new();
        memory.write_byte(0x5a4c, 0x42);

        let mut register = Register::new();
        register.set_accumulator(0x02);
        register.set_carry_bit(true);
        register.set_x(0x10);
        register.set_y(0x20);

        let mut message_bus = MessageBus::new(&memory);

        adc.call(arguments, &mut register, &message_bus);

        assert_eq!(register.a(), 0x45);
        assert_eq!(register.p(), 0b00110000);
    }

    #[test]
    fn test_absolute_x_out_of_bounds() {
        let adc = Adc::new(0x7D);
        let arguments = vec![0xff, 0xff];
        let mut memory = Memory::new();
        memory.write_byte(0x5a, 0x42);

        let mut register = Register::new();
        register.set_accumulator(0x02);
        register.set_carry_bit(true);
        register.set_x(0x5b);
        register.set_y(0x20);

        let mut message_bus = MessageBus::new(&memory);

        adc.call(arguments, &mut register, &message_bus);

        assert_eq!(register.a(), 0x45);
        assert_eq!(register.p(), 0b00110000);
    }

    #[test]
    fn test_absolute_y() {
        let adc = Adc::new(0x79);
        let arguments = vec![0x3c, 0x5a];
        let mut memory = Memory::new();
        memory.write_byte(0x5a4c, 0x42);

        let mut register = Register::new();
        register.set_accumulator(0x02);
        register.set_carry_bit(true);
        register.set_x(0x20);
        register.set_y(0x10);

        let mut message_bus = MessageBus::new(&memory);

        adc.call(arguments, &mut register, &message_bus);

        assert_eq!(register.a(), 0x45);
        assert_eq!(register.p(), 0b00110000);
    }

    #[test]
    fn test_absolute_y_out_of_bounds() {
        let adc = Adc::new(0x79);
        let arguments = vec![0xff, 0xff];
        let mut memory = Memory::new();
        memory.write_byte(0x5a, 0x42);

        let mut register = Register::new();
        register.set_accumulator(0x02);
        register.set_carry_bit(true);
        register.set_x(0x20);
        register.set_y(0x5b);

        let mut message_bus = MessageBus::new(&memory);

        adc.call(arguments, &mut register, &message_bus);

        assert_eq!(register.a(), 0x45);
        assert_eq!(register.p(), 0b00110000);
    }

    #[test]
    fn test_indirect_x() {
        let adc = Adc::new(0x61);
        let arguments = vec![0x44];
        let mut memory = Memory::new();
        memory.write_byte(0x77, 0x05);
        memory.write_byte(0x78, 0x01);
        memory.write_byte(0x0105, 0x07);

        let mut register = Register::new();
        register.set_accumulator(0x11);
        register.set_x(0x33);

        let mut message_bus = MessageBus::new(&memory);

        adc.call(arguments, &mut register, &message_bus);

        assert_eq!(register.a(), 0x18);
        assert_eq!(register.p(), 0x30);
    }

    #[test]
    fn test_indirect_x_out_of_zeropage() {
        let adc = Adc::new(0x61);
        let arguments = vec![0xCC];
        let mut memory = Memory::new();
        memory.write_byte(0xff, 0x05);
        memory.write_byte(0x100, 0x01);
        memory.write_byte(0x0105, 0x07);

        let mut register = Register::new();
        register.set_accumulator(0x11);
        register.set_x(0x33);

        let mut message_bus = MessageBus::new(&memory);

        adc.call(arguments, &mut register, &message_bus);

        assert_eq!(register.a(), 0x18);
        assert_eq!(register.p(), 0x30);
    }

    #[test]
    fn test_indirect_y() {
        let adc = Adc::new(0x71);
        let arguments = vec![0x77];
        let mut memory = Memory::new();
        memory.write_byte(0x77, 0x05);
        memory.write_byte(0x78, 0x01);
        memory.write_byte(0x0109, 0x07);

        let mut register = Register::new();
        register.set_accumulator(0x11);
        register.set_y(0x04);

        let mut message_bus = MessageBus::new(&memory);

        adc.call(arguments, &mut register, &message_bus);

        assert_eq!(register.a(), 0x18);
        assert_eq!(register.p(), 0x30);
    }

    #[test]
    fn test_indirect_y_out_of_zeropage() {
        let adc = Adc::new(0x71);
        let arguments = vec![0xFF];
        let mut memory = Memory::new();
        memory.write_byte(0xFF, 0x05);
        memory.write_byte(0x100, 0x01);
        memory.write_byte(0x0109, 0x07);

        let mut register = Register::new();
        register.set_accumulator(0x11);
        register.set_y(0x04);

        let mut message_bus = MessageBus::new(&memory);

        adc.call(arguments, &mut register, &message_bus);

        assert_eq!(register.a(), 0x18);
        assert_eq!(register.p(), 0x30);
    }
}

