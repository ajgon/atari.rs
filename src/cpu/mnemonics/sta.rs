/*
STA  Store Accumulator in Memory

     A -> M                           N Z C I D V
                                      - - - - - -

     addressing    assembler    opc  bytes  cyles
     --------------------------------------------
     zeropage      STA oper      85    2     3
     zeropage,X    STA oper,X    95    2     4
     absolute      STA oper      8D    3     4
     absolute,X    STA oper,X    9D    3     5
     absolute,Y    STA oper,Y    99    3     5
     (indirect,X)  STA (oper,X)  81    2     6
     (indirect),Y  STA (oper),Y  91    2     6
*/

use crate::cpu::addressing;
use crate::cpu::mnemonics::Mnemonic;
use crate::cpu::register::Register;
use crate::message_bus::MessageBus;
use crate::message_bus::MessageBusTarget;
use crate::message_bus::MessageBusMessage;

#[derive(Debug)]
pub struct Sta {
    mnemonic: String,
    opcode: u8
}

impl Sta {
    pub fn new(opcode: u8) -> Sta {
        return Sta { mnemonic: "STA".to_string(), opcode: opcode };
    }
}

impl Mnemonic for Sta {
    fn determine_bytes(&self) -> usize {
        return match self.opcode {
            0x85 => 2,
            0x95 => 2,
            0x8D => 3,
            0x9D => 3,
            0x99 => 3,
            0x81 => 2,
            0x91 => 2,
            _ => panic!("Invalid opcode `0x{:x}` for mnemonic {}", self.opcode, self.mnemonic)
        }
    }

    fn call(&self, arguments: Vec<u8>, register: &mut Register, message_bus: &mut MessageBus) -> u8 {
        match self.opcode {
            0x85 => return self.call_zero_page(arguments, register, message_bus),
            0x95 => return self.call_zero_page_x(arguments, register, message_bus),
            0x8D => return self.call_absolute(arguments, register, message_bus),
            0x9D => return self.call_absolute_x(arguments, register, message_bus),
            0x99 => return self.call_absolute_y(arguments, register, message_bus),
            0x81 => return self.call_indirect_x(arguments, register, message_bus),
            0x91 => return self.call_indirect_y(arguments, register, message_bus),
            _ => panic!("Invalid opcode `0x{:x}` for mnemonic {}", self.opcode, self.mnemonic)
        }
    }

    fn call_zero_page(&self, arguments: Vec<u8>, register: &mut Register, message_bus: &mut MessageBus) -> u8 {
        let (memory_address, _memory_value, _boundary_crossed) = addressing::zero_page(arguments, message_bus);

        message_bus.send_message(
            MessageBusTarget::Memory, MessageBusMessage::Write, vec![memory_address, register.a() as u16]
        );

        return 3;
    }

    fn call_zero_page_x(&self, arguments: Vec<u8>, register: &mut Register, message_bus: &mut MessageBus) -> u8 {
        let (memory_address, _memory_value, _boundary_crossed) = addressing::zero_page_x(arguments, message_bus, register);

        message_bus.send_message(
            MessageBusTarget::Memory, MessageBusMessage::Write, vec![memory_address, register.a() as u16]
        );

        return 4;
    }

    fn call_absolute(&self, arguments: Vec<u8>, register: &mut Register, message_bus: &mut MessageBus) -> u8 {
        let (memory_address, _memory_value, _boundary_crossed) = addressing::absolute(arguments, message_bus);

        message_bus.send_message(
            MessageBusTarget::Memory, MessageBusMessage::Write, vec![memory_address, register.a() as u16]
        );

        return 4;
    }

    fn call_absolute_x(&self, arguments: Vec<u8>, register: &mut Register, message_bus: &mut MessageBus) -> u8 {
        let (memory_address, _memory_value, _boundary_crossed) = addressing::absolute_x(arguments, message_bus, register);
        message_bus.send_message(
            MessageBusTarget::Memory, MessageBusMessage::Write, vec![memory_address, register.a() as u16]
        );


        return 5;
    }

    fn call_absolute_y(&self, arguments: Vec<u8>, register: &mut Register, message_bus: &mut MessageBus) -> u8 {
        let (memory_address, _memory_value, _boundary_crossed) = addressing::absolute_y(arguments, message_bus, register);
        message_bus.send_message(
            MessageBusTarget::Memory, MessageBusMessage::Write, vec![memory_address, register.a() as u16]
        );


        return 5;
    }

    fn call_indirect_x(&self, arguments: Vec<u8>, register: &mut Register, message_bus: &mut MessageBus) -> u8 {
        let (memory_address, _memory_value, _boundary_crossed) = addressing::indirect_x(arguments, message_bus, register);
        message_bus.send_message(
            MessageBusTarget::Memory, MessageBusMessage::Write, vec![memory_address, register.a() as u16]
        );


        return 6;
    }

    fn call_indirect_y(&self, arguments: Vec<u8>, register: &mut Register, message_bus: &mut MessageBus) -> u8 {
        let (memory_address, _memory_value, _boundary_crossed) = addressing::indirect_y(arguments, message_bus, register);

        message_bus.send_message(
            MessageBusTarget::Memory, MessageBusMessage::Write, vec![memory_address, register.a() as u16]
        );

        return 6;
    }
}

#[cfg(test)]
mod tests {
    use super::Sta;
    use crate::cpu::mnemonics::Mnemonic;
    use crate::cpu::register::Register;
    use crate::memory::Memory;
    use crate::message_bus::MessageBus;

    #[test]
    fn test_zero_page() {
        let sta = Sta::new(0x85);
        let arguments = vec![0x30];
        let mut memory = Memory::new();
        let mut register = Register::new();
        register.set_accumulator(0x42);

        let mut message_bus = MessageBus::new(&mut memory);

        let cycles = sta.call(arguments, &mut register, &mut message_bus);

        assert_eq!(memory.read_byte(0x30), 0x42);
        assert_eq!(register.p(), 0b0010_0000);
        assert_eq!(cycles, 3);
    }

    #[test]
    fn test_zero_page_x() {
        let sta = Sta::new(0x95);
        let arguments = vec![0x30];
        let mut memory = Memory::new();
        let mut register = Register::new();
        register.set_accumulator(0x42);
        register.set_x(0x05);

        let mut message_bus = MessageBus::new(&mut memory);

        let cycles = sta.call(arguments, &mut register, &mut message_bus);

        assert_eq!(memory.read_byte(0x35), 0x42);
        assert_eq!(register.p(), 0b0010_0000);
        assert_eq!(cycles, 4);
    }

    #[test]
    fn test_absolute() {
        let sta = Sta::new(0x8D);
        let arguments = vec![0x3c, 0x5a];
        let mut memory = Memory::new();
        let mut register = Register::new();
        register.set_accumulator(0x42);

        let mut message_bus = MessageBus::new(&mut memory);

        let cycles = sta.call(arguments, &mut register, &mut message_bus);

        assert_eq!(memory.read_byte(0x5a3c), 0x42);
        assert_eq!(register.p(), 0b0010_0000);
        assert_eq!(cycles, 4);
    }

    #[test]
    fn test_absolute_x() {
        let sta = Sta::new(0x9D);
        let arguments = vec![0x3c, 0x5a];
        let mut memory = Memory::new();
        let mut register = Register::new();
        register.set_accumulator(0x42);
        register.set_x(0x10);
        register.set_y(0x20);

        let mut message_bus = MessageBus::new(&mut memory);

        let cycles = sta.call(arguments, &mut register, &mut message_bus);

        assert_eq!(memory.read_byte(0x5a4c), 0x42);
        assert_eq!(register.p(), 0b0010_0000);
        assert_eq!(cycles, 5);
    }

    #[test]
    fn test_absolute_y() {
        let sta = Sta::new(0x99);
        let arguments = vec![0x3c, 0x5a];
        let mut memory = Memory::new();
        let mut register = Register::new();
        register.set_accumulator(0x42);
        register.set_x(0x20);
        register.set_y(0x10);

        let mut message_bus = MessageBus::new(&mut memory);

        let cycles = sta.call(arguments, &mut register, &mut message_bus);

        assert_eq!(memory.read_byte(0x5a4c), 0x42);
        assert_eq!(register.p(), 0b0010_0000);
        assert_eq!(cycles, 5);
    }

    #[test]
    fn test_indirect_x() {
        let sta = Sta::new(0x81);
        let arguments = vec![0x44];
        let mut memory = Memory::new();
        memory.write_byte(0x77, 0x05);
        memory.write_byte(0x78, 0x01);

        let mut register = Register::new();
        register.set_accumulator(0x42);
        register.set_x(0x33);

        let mut message_bus = MessageBus::new(&mut memory);

        let cycles = sta.call(arguments, &mut register, &mut message_bus);

        assert_eq!(memory.read_byte(0x105), 0x42);
        assert_eq!(register.p(), 0b0010_0000);
        assert_eq!(cycles, 6);
    }

    #[test]
    fn test_indirect_y() {
        let sta = Sta::new(0x91);
        let arguments = vec![0x77];
        let mut memory = Memory::new();
        memory.write_byte(0x77, 0x05);
        memory.write_byte(0x78, 0x01);

        let mut register = Register::new();
        register.set_accumulator(0x42);
        register.set_y(0x04);

        let mut message_bus = MessageBus::new(&mut memory);

        let cycles = sta.call(arguments, &mut register, &mut message_bus);

        assert_eq!(memory.read_byte(0x0109), 0x42);
        assert_eq!(register.p(), 0b0010_0000);
        assert_eq!(cycles, 6);
    }

    #[test]
    #[should_panic]
    fn test_invalid_opcode() {
        let sta = Sta::new(0x00);
        let arguments = vec![0xFF];
        let mut memory = Memory::new();
        let mut message_bus = MessageBus::new(&mut memory);
        let mut register = Register::new();

        sta.call(arguments, &mut register, &mut message_bus);
    }
}


