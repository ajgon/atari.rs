/*
CPX  Compare Memory and Index X

     X - M                            N Z C I D V
                                      + + + - - -

     addressing    assembler    opc  bytes  cyles
     --------------------------------------------
     immidiate     CPX #oper     E0    2     2
     zeropage      CPX oper      E4    2     3
     absolute      CPX oper      EC    3     4
*/

use crate::cpu::addressing;
use crate::cpu::mnemonics::Mnemonic;
use crate::cpu::register::Register;
use crate::message_bus::MessageBus;
use crate::cpu::alu;

#[derive(Debug)]
pub struct Cpx {
    mnemonic: String,
    opcode: u8
}

impl Cpx {
    pub fn new(opcode: u8) -> Cpx {
        return Cpx { mnemonic: "CPX".to_string(), opcode: opcode };
    }
}

impl Mnemonic for Cpx {
    fn determine_bytes(&self) -> usize {
        return match self.opcode {
            0xE0 => 2,
            0xE4 => 2,
            0xEC => 3,
            _ => panic!("Invalid opcode `0x{:x}` for mnemonic {}", self.opcode, self.mnemonic)
        }
    }

    fn call(&self, arguments: Vec<u8>, register: &mut Register, message_bus: &mut MessageBus) -> u8 {
        match self.opcode {
            0xE0 => return self.call_immidiate(arguments, register),
            0xE4 => return self.call_zero_page(arguments, register, message_bus),
            0xEC => return self.call_absolute(arguments, register, message_bus),
            _ => panic!("Invalid opcode `0x{:x}` for mnemonic {}", self.opcode, self.mnemonic)
        }
    }

    fn call_immidiate(&self, arguments: Vec<u8>, register: &mut Register) -> u8 {
        let accumulator_value = register.a();

        register.set_accumulator(register.x());
        register.set_carry_bit(true);
        alu::subtract(register.a(), arguments[0], register);
        register.set_accumulator(accumulator_value);

        return 2;
    }

    fn call_zero_page(&self, arguments: Vec<u8>, register: &mut Register, message_bus: &mut MessageBus) -> u8 {
        let (_memory_address, memory_value, _boundary_crossed) = addressing::zero_page(arguments, message_bus);
        let accumulator_value = register.a();

        register.set_accumulator(register.x());
        register.set_carry_bit(true);
        alu::subtract(register.a(), memory_value, register);
        register.set_accumulator(accumulator_value);

        return 3;
    }

    fn call_absolute(&self, arguments: Vec<u8>, register: &mut Register, message_bus: &mut MessageBus) -> u8 {
        let (_memory_address, memory_value, _boundary_crossed) = addressing::absolute(arguments, message_bus);
        let accumulator_value = register.a();

        register.set_accumulator(register.x());
        register.set_carry_bit(true);
        alu::subtract(register.a(), memory_value, register);
        register.set_accumulator(accumulator_value);

        return 4;
    }
}

#[cfg(test)]
mod tests {
    use super::Cpx;
    use crate::cpu::mnemonics::Mnemonic;
    use crate::cpu::register::Register;
    use crate::memory::Memory;
    use crate::message_bus::MessageBus;

    #[test]
    fn test_immidiate_argument_equals() {
        let cpx = Cpx::new(0xE0);
        let arguments = vec![0x42];
        let mut memory = Memory::new();
        let mut register = Register::new();
        register.set_accumulator(0x99);
        register.set_x(0x42);

        let mut message_bus = MessageBus::new(&mut memory);

        let cycles = cpx.call(arguments, &mut register, &mut message_bus);

        assert_eq!(register.a(), 0x99);
        assert_eq!(register.x(), 0x42);
        assert_eq!(register.p(), 0b00110011);
        assert_eq!(cycles, 2);
    }

    #[test]
    fn test_immidiate_argument_greater() {
        let cpx = Cpx::new(0xE0);
        let arguments = vec![0x62];
        let mut memory = Memory::new();
        let mut register = Register::new();
        register.set_accumulator(0x99);
        register.set_x(0x42);

        let mut message_bus = MessageBus::new(&mut memory);

        let cycles = cpx.call(arguments, &mut register, &mut message_bus);

        assert_eq!(register.a(), 0x99);
        assert_eq!(register.x(), 0x42);
        assert_eq!(register.p(), 0b10110000);
        assert_eq!(cycles, 2);
    }

    #[test]
    fn test_immidiate_argument_greater_negative() {
        let cpx = Cpx::new(0xE0);
        let arguments = vec![0xF2];
        let mut memory = Memory::new();
        let mut register = Register::new();
        register.set_accumulator(0x99);
        register.set_x(0x42);

        let mut message_bus = MessageBus::new(&mut memory);

        let cycles = cpx.call(arguments, &mut register, &mut message_bus);

        assert_eq!(register.a(), 0x99);
        assert_eq!(register.x(), 0x42);
        assert_eq!(register.p(), 0b00110000);
        assert_eq!(cycles, 2);
    }

    #[test]
    fn test_immidiate_argument_less() {
        let cpx = Cpx::new(0xE0);
        let arguments = vec![0x22];
        let mut memory = Memory::new();
        let mut register = Register::new();
        register.set_accumulator(0x99);
        register.set_x(0x42);

        let mut message_bus = MessageBus::new(&mut memory);

        let cycles = cpx.call(arguments, &mut register, &mut message_bus);

        assert_eq!(register.a(), 0x99);
        assert_eq!(register.x(), 0x42);
        assert_eq!(register.p(), 0b00110001);
        assert_eq!(cycles, 2);
    }

    #[test]
    fn test_immidiate_argument_less_negative() {
        let cpx = Cpx::new(0xE0);
        let arguments = vec![0xC2];
        let mut memory = Memory::new();
        let mut register = Register::new();
        register.set_accumulator(0x99);
        register.set_x(0xF2);

        let mut message_bus = MessageBus::new(&mut memory);

        let cycles = cpx.call(arguments, &mut register, &mut message_bus);

        assert_eq!(register.a(), 0x99);
        assert_eq!(register.x(), 0xF2);
        assert_eq!(register.p(), 0b00110001);
        assert_eq!(cycles, 2);
    }

    #[test]
    fn test_zero_page() {
        let cpx = Cpx::new(0xE4);
        let arguments = vec![0x30];
        let mut memory = Memory::new();
        memory.write_byte(0x30, 0xF2);

        let mut register = Register::new();
        register.set_accumulator(0x99);
        register.set_x(0x42);

        let mut message_bus = MessageBus::new(&mut memory);

        let cycles = cpx.call(arguments, &mut register, &mut message_bus);

        assert_eq!(register.a(), 0x99);
        assert_eq!(register.x(), 0x42);
        assert_eq!(register.p(), 0b00110000);
        assert_eq!(cycles, 3);
    }

    #[test]
    fn test_absolute() {
        let cpx = Cpx::new(0xEC);
        let arguments = vec![0x3c, 0x5a];
        let mut memory = Memory::new();
        memory.write_byte(0x5a3c, 0xF2);

        let mut register = Register::new();
        register.set_accumulator(0x99);
        register.set_x(0x42);

        let mut message_bus = MessageBus::new(&mut memory);

        let cycles = cpx.call(arguments, &mut register, &mut message_bus);

        assert_eq!(register.a(), 0x99);
        assert_eq!(register.x(), 0x42);
        assert_eq!(register.p(), 0b00110000);
        assert_eq!(cycles, 4);
    }

    #[test]
    #[should_panic]
    fn test_invalid_opcode() {
        let cpx = Cpx::new(0x00);
        let arguments = vec![0xFF];
        let mut memory = Memory::new();
        let mut message_bus = MessageBus::new(&mut memory);
        let mut register = Register::new();

        cpx.call(arguments, &mut register, &mut message_bus);
    }
}

