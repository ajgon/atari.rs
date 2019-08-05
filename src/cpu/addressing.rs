use crate::cpu::register::Register;
use crate::message_bus::MessageBus;
use crate::message_bus::MessageBusTarget;
use crate::message_bus::MessageBusMessage;

pub fn zero_page(arguments: Vec<u8>, message_bus: &MessageBus) -> (u8, bool) {
    let memory_value = message_bus.send_message(MessageBusTarget::Memory, MessageBusMessage::Read, arguments[0] as u16);
    let boundary_crossed = false;

    return (memory_value, boundary_crossed);
}

pub fn zero_page_x(arguments: Vec<u8>, message_bus: &MessageBus, register: &Register) -> (u8, bool) {
    let memory_address = arguments[0].overflowing_add(register.x()).0 as u16;
    let memory_value = message_bus.send_message(
        MessageBusTarget::Memory, MessageBusMessage::Read, memory_address
    );
    let boundary_crossed = false;

    return (memory_value, boundary_crossed);
}

pub fn zero_page_y(arguments: Vec<u8>, message_bus: &MessageBus, register: &Register) -> (u8, bool) {
    let memory_address = arguments[0].overflowing_add(register.y()).0 as u16;
    let memory_value = message_bus.send_message(
        MessageBusTarget::Memory, MessageBusMessage::Read, memory_address
    );
    let boundary_crossed = false;

    return (memory_value, boundary_crossed);
}

pub fn absolute(arguments: Vec<u8>, message_bus: &MessageBus) -> (u8, bool) {
    let memory_address: u16 = ((arguments[1] as u16) << 8) + arguments[0] as u16;
    let memory_value = message_bus.send_message(
        MessageBusTarget::Memory, MessageBusMessage::Read, memory_address
    );
    let boundary_crossed = false;

    return (memory_value, boundary_crossed);
}

pub fn absolute_x(arguments: Vec<u8>, message_bus: &MessageBus, register: &Register) -> (u8, bool) {
    let base_memory_address: u16 = ((arguments[1] as u16) << 8) + arguments[0] as u16;
    let memory_address: u16 = base_memory_address.overflowing_add(register.x() as u16).0;
    let memory_value = message_bus.send_message(
        MessageBusTarget::Memory, MessageBusMessage::Read, memory_address
    );
    let boundary_crossed = base_memory_address & 0xff00 != memory_address & 0xff00;

    return (memory_value, boundary_crossed);
}

pub fn absolute_y(arguments: Vec<u8>, message_bus: &MessageBus, register: &Register) -> (u8, bool) {
    let base_memory_address: u16 = ((arguments[1] as u16) << 8) + arguments[0] as u16;
    let memory_address: u16 = base_memory_address.overflowing_add(register.y() as u16).0;
    let memory_value = message_bus.send_message(
        MessageBusTarget::Memory, MessageBusMessage::Read, memory_address
    );
    let boundary_crossed = base_memory_address & 0xff00 != memory_address & 0xff00;

    return (memory_value, boundary_crossed);
}

pub fn indirect_x(arguments: Vec<u8>, message_bus: &MessageBus, register: &Register) -> (u8, bool) {
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
    let boundary_crossed = false;

    return (memory_value, boundary_crossed);
}

pub fn indirect_y(arguments: Vec<u8>, message_bus: &MessageBus, register: &Register) -> (u8, bool) {
    let memory_address: u16 = arguments[0] as u16;
    let memory_value = message_bus.send_message(
        MessageBusTarget::Memory, MessageBusMessage::Read, memory_address
    );
    let base_new_memory_address: u16 = memory_value as u16;
    let memory_value = message_bus.send_message(
        MessageBusTarget::Memory, MessageBusMessage::Read, memory_address.overflowing_add(1).0
    );
    let base_new_memory_address: u16 = base_new_memory_address + ((memory_value as u16) << 8);
    let new_memory_address: u16 = base_new_memory_address.overflowing_add(register.y() as u16).0;
    let memory_value = message_bus.send_message(
        MessageBusTarget::Memory, MessageBusMessage::Read, new_memory_address
    );
    let boundary_crossed = base_new_memory_address & 0xff00 != new_memory_address & 0xff00;

    return (memory_value, boundary_crossed);
}

#[cfg(test)]
mod tests {
    use super::zero_page;
    use super::zero_page_x;
    use super::zero_page_y;
    use super::absolute;
    use super::absolute_x;
    use super::absolute_y;
    use super::indirect_x;
    use super::indirect_y;

    use crate::cpu::mnemonics::Mnemonic;
    use crate::cpu::register::Register;
    use crate::memory::Memory;
    use crate::message_bus::MessageBus;

    #[test]
    fn test_zero_page() {
        let arguments = vec![0x30];
        let mut memory = Memory::new();
        memory.write_byte(0x30, 0x42);

        let mut message_bus = MessageBus::new(&memory);

        let value = zero_page(arguments, &message_bus);

        assert_eq!(value, (0x42, false))
    }

    #[test]
    fn test_zero_page_x() {
        let arguments = vec![0x30];
        let mut memory = Memory::new();
        memory.write_byte(0x35, 0x42);

        let mut register = Register::new();
        register.set_x(0x05);

        let mut message_bus = MessageBus::new(&memory);

        let value = zero_page_x(arguments, &message_bus, &register);

        assert_eq!(value, (0x42, false))
    }

    #[test]
    fn test_addrss_zero_page_x_out_of_bounds() {
        let arguments = vec![0xff];
        let mut memory = Memory::new();
        memory.write_byte(0x35, 0x42);
        memory.write_byte(0x135, 0x27);

        let mut register = Register::new();
        register.set_x(0x36);

        let mut message_bus = MessageBus::new(&memory);

        let value = zero_page_x(arguments, &message_bus, &register);

        assert_eq!(value, (0x42, false))
    }

    #[test]
    fn test_zero_page_y() {
        let arguments = vec![0x30];
        let mut memory = Memory::new();
        memory.write_byte(0x35, 0x42);

        let mut register = Register::new();
        register.set_y(0x05);

        let mut message_bus = MessageBus::new(&memory);

        let value = zero_page_y(arguments, &message_bus, &register);

        assert_eq!(value, (0x42, false))
    }

    #[test]
    fn test_addrss_zero_page_y_out_of_bounds() {
        let arguments = vec![0xff];
        let mut memory = Memory::new();
        memory.write_byte(0x35, 0x42);
        memory.write_byte(0x135, 0x27);

        let mut register = Register::new();
        register.set_y(0x36);

        let mut message_bus = MessageBus::new(&memory);

        let value = zero_page_y(arguments, &message_bus, &register);

        assert_eq!(value, (0x42, false))
    }

    #[test]
    fn test_absolute() {
        let arguments = vec![0x3c, 0x5a];
        let mut memory = Memory::new();
        memory.write_byte(0x5a3c, 0x42);

        let mut message_bus = MessageBus::new(&memory);

        let value = absolute(arguments, &message_bus);

        assert_eq!(value, (0x42, false))
    }

    #[test]
    fn test_absolute_x() {
        let arguments = vec![0x3c, 0x5a];
        let mut memory = Memory::new();
        memory.write_byte(0x5a4c, 0x42);

        let mut register = Register::new();
        register.set_x(0x10);

        let mut message_bus = MessageBus::new(&memory);

        let value = absolute_x(arguments, &message_bus, &register);

        assert_eq!(value, (0x42, false))
    }

    #[test]
    fn test_absolute_x_page_boundary_crossed() {
        let arguments = vec![0xfc, 0x5a];
        let mut memory = Memory::new();
        memory.write_byte(0x5b0c, 0x42);

        let mut register = Register::new();
        register.set_x(0x10);

        let mut message_bus = MessageBus::new(&memory);

        let value = absolute_x(arguments, &message_bus, &register);

        assert_eq!(value, (0x42, true))
    }

    #[test]
    fn test_absolute_x_out_of_bounds() {
        let arguments = vec![0xff, 0xff];
        let mut memory = Memory::new();
        memory.write_byte(0x5a, 0x42);

        let mut register = Register::new();
        register.set_x(0x5b);

        let mut message_bus = MessageBus::new(&memory);

        let value = absolute_x(arguments, &message_bus, &register);

        assert_eq!(value, (0x42, true))
    }

    #[test]
    fn test_absolute_y() {
        let arguments = vec![0x3c, 0x5a];
        let mut memory = Memory::new();
        memory.write_byte(0x5a4c, 0x42);

        let mut register = Register::new();
        register.set_y(0x10);

        let mut message_bus = MessageBus::new(&memory);

        let value = absolute_y(arguments, &message_bus, &register);

        assert_eq!(value, (0x42, false))
    }

    #[test]
    fn test_absolute_y_page_boundary_crossed() {
        let arguments = vec![0xfc, 0x5a];
        let mut memory = Memory::new();
        memory.write_byte(0x5b0c, 0x42);

        let mut register = Register::new();
        register.set_y(0x10);

        let mut message_bus = MessageBus::new(&memory);

        let value = absolute_y(arguments, &message_bus, &register);

        assert_eq!(value, (0x42, true))
    }

    #[test]
    fn test_absolute_y_out_of_bounds() {
        let arguments = vec![0xff, 0xff];
        let mut memory = Memory::new();
        memory.write_byte(0x5a, 0x42);

        let mut register = Register::new();
        register.set_y(0x5b);

        let mut message_bus = MessageBus::new(&memory);

        let value = absolute_y(arguments, &message_bus, &register);

        assert_eq!(value, (0x42, true))
    }

    #[test]
    fn test_indirect_x() {
        let arguments = vec![0x44];
        let mut memory = Memory::new();
        memory.write_byte(0x77, 0x05);
        memory.write_byte(0x78, 0x01);
        memory.write_byte(0x0105, 0x42);

        let mut register = Register::new();
        register.set_x(0x33);

        let mut message_bus = MessageBus::new(&memory);

        let value = indirect_x(arguments, &message_bus, &register);

        assert_eq!(value, (0x42, false))
    }

    #[test]
    fn test_indirect_x_out_of_zeropage() {
        let arguments = vec![0xCC];
        let mut memory = Memory::new();
        memory.write_byte(0xff, 0x05);
        memory.write_byte(0x100, 0x01);
        memory.write_byte(0x0105, 0x42);

        let mut register = Register::new();
        register.set_x(0x33);

        let mut message_bus = MessageBus::new(&memory);

        let value = indirect_x(arguments, &message_bus, &register);

        assert_eq!(value, (0x42, false))
    }

    #[test]
    fn test_indirect_y() {
        let arguments = vec![0x77];
        let mut memory = Memory::new();
        memory.write_byte(0x77, 0x05);
        memory.write_byte(0x78, 0x01);
        memory.write_byte(0x0109, 0x42);

        let mut register = Register::new();
        register.set_y(0x04);

        let mut message_bus = MessageBus::new(&memory);

        let value = indirect_y(arguments, &message_bus, &register);

        assert_eq!(value, (0x42, false))
    }

    #[test]
    fn test_indirect_y_out_of_bonds() {
        let arguments = vec![0x77];
        let mut memory = Memory::new();
        memory.write_byte(0x77, 0xff);
        memory.write_byte(0x78, 0x01);
        memory.write_byte(0x205, 0x42);

        let mut register = Register::new();
        register.set_y(0x06);

        let mut message_bus = MessageBus::new(&memory);

        let value = indirect_y(arguments, &message_bus, &register);

        assert_eq!(value, (0x42, true))
    }

    #[test]
    fn test_indirect_y_out_of_zeropage() {
        let arguments = vec![0xFF];
        let mut memory = Memory::new();
        memory.write_byte(0xFF, 0x05);
        memory.write_byte(0x100, 0x01);
        memory.write_byte(0x0109, 0x42);

        let mut register = Register::new();
        register.set_y(0x04);

        let mut message_bus = MessageBus::new(&memory);

        let value = indirect_y(arguments, &message_bus, &register);

        assert_eq!(value, (0x42, false))
    }
}

