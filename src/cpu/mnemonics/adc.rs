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

use crate::cpu::addressing;
use crate::cpu::mnemonics::Mnemonic;
use crate::cpu::register::Register;
use crate::message_bus::MessageBus;
use crate::cpu::alu;

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
    fn determine_bytes(&self) -> usize {
        return match self.opcode {
            0x69 => 2,
            0x65 => 2,
            0x75 => 2,
            0x6D => 3,
            0x7D => 3,
            0x79 => 3,
            0x61 => 2,
            0x71 => 2,
            _ => panic!("Invalid opcode `0x{:x}` for mnemonic {}", self.opcode, self.mnemonic)
        }
    }

    fn call(&self, arguments: Vec<u8>, register: &mut Register, message_bus: &MessageBus) -> u8 {
        match self.opcode {
            0x69 => return self.call_immidiate(arguments, register),
            0x65 => return self.call_zero_page(arguments, register, &message_bus),
            0x75 => return self.call_zero_page_x(arguments, register, &message_bus),
            0x6D => return self.call_absolute(arguments, register, &message_bus),
            0x7D => return self.call_absolute_x(arguments, register, &message_bus),
            0x79 => return self.call_absolute_y(arguments, register, &message_bus),
            0x61 => return self.call_indirect_x(arguments, register, &message_bus),
            0x71 => return self.call_indirect_y(arguments, register, &message_bus),
            _ => panic!("Invalid opcode `0x{:x}` for mnemonic {}", self.opcode, self.mnemonic)
        }
    }

    fn call_immidiate(&self, arguments: Vec<u8>, register: &mut Register) -> u8 {
        alu::add(arguments[0], register);
        return 2;
    }

    fn call_zero_page(&self, arguments: Vec<u8>, register: &mut Register, message_bus: &MessageBus) -> u8 {
        let (memory_value, _boundary_crossed) = addressing::zero_page(arguments, message_bus);

        alu::add(memory_value, register);
        return 3;
    }

    fn call_zero_page_x(&self, arguments: Vec<u8>, register: &mut Register, message_bus: &MessageBus) -> u8 {
        let (memory_value, _boundary_crossed) = addressing::zero_page_x(arguments, message_bus, register);

        alu::add(memory_value, register);
        return 4;
    }

    fn call_absolute(&self, arguments: Vec<u8>, register: &mut Register, message_bus: &MessageBus) -> u8 {
        let (memory_value, _boundary_crossed) = addressing::absolute(arguments, message_bus);

        alu::add(memory_value, register);
        return 4;
    }

    fn call_absolute_x(&self, arguments: Vec<u8>, register: &mut Register, message_bus: &MessageBus) -> u8 {
        let (memory_value, boundary_crossed) = addressing::absolute_x(arguments, message_bus, register);

        alu::add(memory_value, register);
        return if boundary_crossed { 5u8 } else { 4u8 };
    }

    fn call_absolute_y(&self, arguments: Vec<u8>, register: &mut Register, message_bus: &MessageBus) -> u8 {
        let (memory_value, boundary_crossed) = addressing::absolute_y(arguments, message_bus, register);

        alu::add(memory_value, register);
        return if boundary_crossed { 5u8 } else { 4u8 };
    }

    fn call_indirect_x(&self, arguments: Vec<u8>, register: &mut Register, message_bus: &MessageBus) -> u8 {
        let (memory_value, _boundary_crossed) = addressing::indirect_x(arguments, message_bus, register);

        alu::add(memory_value, register);
        return 6;
    }

    fn call_indirect_y(&self, arguments: Vec<u8>, register: &mut Register, message_bus: &MessageBus) -> u8 {
        let (memory_value, boundary_crossed) = addressing::indirect_y(arguments, message_bus, register);

        alu::add(memory_value, register);
        return if boundary_crossed { 6u8 } else { 5u8 };
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
        let memory = Memory::new();
        let mut register = Register::new();
        register.set_accumulator(0x02);
        register.set_carry_bit(true);

        let message_bus = MessageBus::new(&memory);

        let cycles = adc.call(arguments, &mut register, &message_bus);

        assert_eq!(register.a(), 0x45);
        assert_eq!(register.p(), 0b00110000);
        assert_eq!(cycles, 2);
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

        let message_bus = MessageBus::new(&memory);

        let cycles = adc.call(arguments, &mut register, &message_bus);

        assert_eq!(register.a(), 0x45);
        assert_eq!(register.p(), 0b00110000);
        assert_eq!(cycles, 3);
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

        let message_bus = MessageBus::new(&memory);

        let cycles = adc.call(arguments, &mut register, &message_bus);

        assert_eq!(register.a(), 0x45);
        assert_eq!(register.p(), 0b00110000);
        assert_eq!(cycles, 4);
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

        let message_bus = MessageBus::new(&memory);

        let cycles = adc.call(arguments, &mut register, &message_bus);

        assert_eq!(register.a(), 0x45);
        assert_eq!(register.p(), 0b00110000);
        assert_eq!(cycles, 4);
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

        let message_bus = MessageBus::new(&memory);

        let cycles = adc.call(arguments, &mut register, &message_bus);

        assert_eq!(register.a(), 0x45);
        assert_eq!(register.p(), 0b00110000);
        assert_eq!(cycles, 4);
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

        let message_bus = MessageBus::new(&memory);

        let cycles = adc.call(arguments, &mut register, &message_bus);

        assert_eq!(register.a(), 0x45);
        assert_eq!(register.p(), 0b00110000);
        assert_eq!(cycles, 5);
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

        let message_bus = MessageBus::new(&memory);

        let cycles = adc.call(arguments, &mut register, &message_bus);

        assert_eq!(register.a(), 0x45);
        assert_eq!(register.p(), 0b00110000);
        assert_eq!(cycles, 4);
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

        let message_bus = MessageBus::new(&memory);

        let cycles = adc.call(arguments, &mut register, &message_bus);

        assert_eq!(register.a(), 0x45);
        assert_eq!(register.p(), 0b00110000);
        assert_eq!(cycles, 5);
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

        let message_bus = MessageBus::new(&memory);

        let cycles = adc.call(arguments, &mut register, &message_bus);

        assert_eq!(register.a(), 0x18);
        assert_eq!(register.p(), 0b00110000);
        assert_eq!(cycles, 6);
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

        let message_bus = MessageBus::new(&memory);

        let cycles = adc.call(arguments, &mut register, &message_bus);

        assert_eq!(register.a(), 0x18);
        assert_eq!(register.p(), 0b00110000);
        assert_eq!(cycles, 5);
    }

    #[test]
    fn test_indirect_y_out_of_bonds() {
        let adc = Adc::new(0x71);
        let arguments = vec![0x77];
        let mut memory = Memory::new();
        memory.write_byte(0x77, 0xff);
        memory.write_byte(0x78, 0x01);
        memory.write_byte(0x0205, 0x07);

        let mut register = Register::new();
        register.set_accumulator(0x11);
        register.set_y(0x06);

        let message_bus = MessageBus::new(&memory);

        let cycles = adc.call(arguments, &mut register, &message_bus);

        assert_eq!(register.a(), 0x18);
        assert_eq!(register.p(), 0b00110000);
        assert_eq!(cycles, 6);
    }

    #[test]
    #[should_panic]
    fn test_invalid_opcode() {
        let adc = Adc::new(0x00);
        let arguments = vec![0xFF];
        let memory = Memory::new();
        let message_bus = MessageBus::new(&memory);
        let mut register = Register::new();

        adc.call(arguments, &mut register, &message_bus);
    }
}

