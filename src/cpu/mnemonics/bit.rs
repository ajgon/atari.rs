/*
BIT  Test Bits in Memory with Accumulator

     bits 7 and 6 of operand are transfered to bit 7 and 6 of SR (N,V);
     the zeroflag is set to the result of operand AND accumulator.

     A AND M, M7 -> N, M6 -> V        N Z C I D V
                                     M7 + - - - M6

     addressing    assembler    opc  bytes  cyles
     --------------------------------------------
     zeropage      BIT oper      24    2     3
     absolute      BIT oper      2C    3     4
*/

use crate::cpu::addressing;
use crate::cpu::mnemonics::Mnemonic;
use crate::cpu::register::Register;
use crate::message_bus::MessageBus;

#[derive(Debug)]
pub struct Bit {
    mnemonic: String,
    opcode: u8
}

impl Bit {
    pub fn new(opcode: u8) -> Bit {
        return Bit { mnemonic: "BIT".to_string(), opcode: opcode };
    }
}

impl Mnemonic for Bit {
    fn determine_bytes(&self) -> usize {
        return match self.opcode {
            0x24 => 2,
            0x2C => 3,
            _ => panic!("Invalid opcode `0x{:x}` for mnemonic {}", self.opcode, self.mnemonic)
        }
    }

    fn call(&self, arguments: Vec<u8>, register: &mut Register, message_bus: &mut MessageBus) -> u8 {
        match self.opcode {
            0x24 => return self.call_zero_page(arguments, register, message_bus),
            0x2C => return self.call_absolute(arguments, register, message_bus),
            _ => panic!("Invalid opcode `0x{:x}` for mnemonic {}", self.opcode, self.mnemonic)
        }
    }

    fn call_zero_page(&self, arguments: Vec<u8>, register: &mut Register, message_bus: &mut MessageBus) -> u8 {
        let (_memory_address, memory_value, _boundary_crossed) = addressing::zero_page(arguments, message_bus);

        register.set_negative_bit(memory_value & 0x80 == 0x80);
        register.set_overflow_bit(memory_value & 0x40 == 0x40);
        register.set_zero_bit(memory_value & register.a() == 0);

        return 3;
    }

    fn call_absolute(&self, arguments: Vec<u8>, register: &mut Register, message_bus: &mut MessageBus) -> u8 {
        let (_memory_address, memory_value, _boundary_crossed) = addressing::absolute(arguments, message_bus);

        register.set_negative_bit(memory_value & 0x80 == 0x80);
        register.set_overflow_bit(memory_value & 0x40 == 0x40);
        register.set_zero_bit(memory_value & register.a() == 0);

        return 4;
    }
}

#[cfg(test)]
mod tests {
    use super::Bit;
    use crate::cpu::mnemonics::Mnemonic;
    use crate::cpu::register::Register;
    use crate::memory::Memory;
    use crate::message_bus::MessageBus;

    #[test]
    fn test_zero_page() {
        let bit = Bit::new(0x24);
        let arguments = vec![0x30];
        let mut memory = Memory::new();
        memory.write_byte(0x30, 0b0000_1111);

        let mut register = Register::new();
        register.set_accumulator(0b0001_0001);

        let mut message_bus = MessageBus::new(&mut memory);

        let cycles = bit.call(arguments, &mut register, &mut message_bus);

        assert_eq!(register.a(), 0b0001_0001);
        assert_eq!(register.p(), 0b0011_0000);
        assert_eq!(cycles, 3);
    }

    #[test]
    fn test_zero_page_with_negative_bit_set() {
        let bit = Bit::new(0x24);
        let arguments = vec![0x30];
        let mut memory = Memory::new();
        memory.write_byte(0x30, 0b1000_1111);

        let mut register = Register::new();
        register.set_accumulator(0b0001_0001);

        let mut message_bus = MessageBus::new(&mut memory);

        let cycles = bit.call(arguments, &mut register, &mut message_bus);

        assert_eq!(register.a(), 0b0001_0001);
        assert_eq!(register.p(), 0b1011_0000);
        assert_eq!(cycles, 3);
    }

    #[test]
    fn test_zero_page_with_overflow_bit_set() {
        let bit = Bit::new(0x24);
        let arguments = vec![0x30];
        let mut memory = Memory::new();
        memory.write_byte(0x30, 0b0100_1111);

        let mut register = Register::new();
        register.set_accumulator(0b0001_0001);

        let mut message_bus = MessageBus::new(&mut memory);

        let cycles = bit.call(arguments, &mut register, &mut message_bus);

        assert_eq!(register.a(), 0b0001_0001);
        assert_eq!(register.p(), 0b0111_0000);
        assert_eq!(cycles, 3);
    }

    #[test]
    fn test_zero_page_with_zero_bit_set() {
        let bit = Bit::new(0x24);
        let arguments = vec![0x30];
        let mut memory = Memory::new();
        memory.write_byte(0x30, 0b0000_1110);

        let mut register = Register::new();
        register.set_accumulator(0b0001_0001);

        let mut message_bus = MessageBus::new(&mut memory);

        let cycles = bit.call(arguments, &mut register, &mut message_bus);

        assert_eq!(register.a(), 0b0001_0001);
        assert_eq!(register.p(), 0b0011_0010);
        assert_eq!(cycles, 3);
    }

    #[test]
    fn test_absolute() {
        let bit = Bit::new(0x2C);
        let arguments = vec![0x3c, 0x5a];
        let mut memory = Memory::new();
        memory.write_byte(0x5a3c, 0b0000_1111);

        let mut register = Register::new();
        register.set_accumulator(0b0001_0001);

        let mut message_bus = MessageBus::new(&mut memory);

        let cycles = bit.call(arguments, &mut register, &mut message_bus);

        assert_eq!(register.a(), 0b0001_0001);
        assert_eq!(register.p(), 0b0011_0000);
        assert_eq!(cycles, 4);
    }

    #[test]
    fn test_absolute_with_negative_bit_set() {
        let bit = Bit::new(0x2C);
        let arguments = vec![0x3c, 0x5a];
        let mut memory = Memory::new();
        memory.write_byte(0x5a3c, 0b1000_1111);

        let mut register = Register::new();
        register.set_accumulator(0b0001_0001);

        let mut message_bus = MessageBus::new(&mut memory);

        let cycles = bit.call(arguments, &mut register, &mut message_bus);

        assert_eq!(register.a(), 0b0001_0001);
        assert_eq!(register.p(), 0b1011_0000);
        assert_eq!(cycles, 4);
    }

    #[test]
    fn test_absolute_with_overflow_bit_set() {
        let bit = Bit::new(0x2C);
        let arguments = vec![0x3c, 0x5a];
        let mut memory = Memory::new();
        memory.write_byte(0x5a3c, 0b0100_1111);

        let mut register = Register::new();
        register.set_accumulator(0b0001_0001);

        let mut message_bus = MessageBus::new(&mut memory);

        let cycles = bit.call(arguments, &mut register, &mut message_bus);

        assert_eq!(register.a(), 0b0001_0001);
        assert_eq!(register.p(), 0b0111_0000);
        assert_eq!(cycles, 4);
    }

    #[test]
    fn test_absolute_with_zero_bit_set() {
        let bit = Bit::new(0x2C);
        let arguments = vec![0x3c, 0x5a];
        let mut memory = Memory::new();
        memory.write_byte(0x5a3c, 0b0000_1110);

        let mut register = Register::new();
        register.set_accumulator(0b0001_0001);

        let mut message_bus = MessageBus::new(&mut memory);

        let cycles = bit.call(arguments, &mut register, &mut message_bus);

        assert_eq!(register.a(), 0b0001_0001);
        assert_eq!(register.p(), 0b0011_0010);
        assert_eq!(cycles, 4);
    }

    #[test]
    #[should_panic]
    fn test_invalid_opcode() {
        let bit = Bit::new(0x00);
        let arguments = vec![0xFF];
        let mut memory = Memory::new();
        let mut message_bus = MessageBus::new(&mut memory);
        let mut register = Register::new();

        bit.call(arguments, &mut register, &mut message_bus);
    }
}



