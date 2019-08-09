/*
ORA  OR Memory with Accumulator

     A OR M -> A                      N Z C I D V
                                      + + - - - -

     addressing    assembler    opc  bytes  cyles
     --------------------------------------------
     immidiate     ORA #oper     09    2     2
     zeropage      ORA oper      05    2     3
     zeropage,X    ORA oper,X    15    2     4
     absolute      ORA oper      0D    3     4
     absolute,X    ORA oper,X    1D    3     4*
     absolute,Y    ORA oper,Y    19    3     4*
     (indirect,X)  ORA (oper,X)  01    2     6
     (indirect),Y  ORA (oper),Y  11    2     5*
*/
use crate::cpu::addressing;
use crate::cpu::mnemonics::Mnemonic;
use crate::cpu::register::Register;
use crate::message_bus::MessageBus;
use crate::cpu::alu;

#[derive(Debug)]
pub struct Ora {
    mnemonic: String,
    opcode: u8
}

impl Ora {
    pub fn new(opcode: u8) -> Ora {
        return Ora { mnemonic: "ORA".to_string(), opcode: opcode };
    }
}

impl Mnemonic for Ora {
    fn determine_bytes(&self) -> usize {
        return match self.opcode {
            0x09 => 2,
            0x05 => 2,
            0x15 => 2,
            0x0D => 3,
            0x1D => 3,
            0x19 => 3,
            0x01 => 2,
            0x11 => 2,
            _ => panic!("Invalid opcode `0x{:x}` for mnemonic {}", self.opcode, self.mnemonic)
        }
    }

    fn call(&self, arguments: Vec<u8>, register: &mut Register, message_bus: &mut MessageBus) -> u8 {
        match self.opcode {
            0x09 => return self.call_immidiate(arguments, register),
            0x05 => return self.call_zero_page(arguments, register, message_bus),
            0x15 => return self.call_zero_page_x(arguments, register, message_bus),
            0x0D => return self.call_absolute(arguments, register, message_bus),
            0x1D => return self.call_absolute_x(arguments, register, message_bus),
            0x19 => return self.call_absolute_y(arguments, register, message_bus),
            0x01 => return self.call_indirect_x(arguments, register, message_bus),
            0x11 => return self.call_indirect_y(arguments, register, message_bus),
            _ => panic!("Invalid opcode `0x{:x}` for mnemonic {}", self.opcode, self.mnemonic)
        }
    }

    fn call_immidiate(&self, arguments: Vec<u8>, register: &mut Register) -> u8 {
        let result = alu::or(register.a(), arguments[0], register);
        register.set_accumulator(result);

        return 2;
    }

    fn call_zero_page(&self, arguments: Vec<u8>, register: &mut Register, message_bus: &mut MessageBus) -> u8 {
        let (_memory_address, memory_value, _boundary_crossed) = addressing::zero_page(arguments, message_bus);

        let result = alu::or(register.a(), memory_value, register);
        register.set_accumulator(result);

        return 3;
    }

    fn call_zero_page_x(&self, arguments: Vec<u8>, register: &mut Register, message_bus: &mut MessageBus) -> u8 {
        let (_memory_address, memory_value, _boundary_crossed) = addressing::zero_page_x(arguments, message_bus, register);

        let result = alu::or(register.a(), memory_value, register);
        register.set_accumulator(result);

        return 4;
    }

    fn call_absolute(&self, arguments: Vec<u8>, register: &mut Register, message_bus: &mut MessageBus) -> u8 {
        let (_memory_address, memory_value, _boundary_crossed) = addressing::absolute(arguments, message_bus);

        let result = alu::or(register.a(), memory_value, register);
        register.set_accumulator(result);

        return 4;
    }

    fn call_absolute_x(&self, arguments: Vec<u8>, register: &mut Register, message_bus: &mut MessageBus) -> u8 {
        let (_memory_address, memory_value, boundary_crossed) = addressing::absolute_x(arguments, message_bus, register);

        let result = alu::or(register.a(), memory_value, register);
        register.set_accumulator(result);

        return if boundary_crossed { 5u8 } else { 4u8 };
    }

    fn call_absolute_y(&self, arguments: Vec<u8>, register: &mut Register, message_bus: &mut MessageBus) -> u8 {
        let (_memory_address, memory_value, boundary_crossed) = addressing::absolute_y(arguments, message_bus, register);

        let result = alu::or(register.a(), memory_value, register);
        register.set_accumulator(result);

        return if boundary_crossed { 5u8 } else { 4u8 };
    }

    fn call_indirect_x(&self, arguments: Vec<u8>, register: &mut Register, message_bus: &mut MessageBus) -> u8 {
        let (_memory_address, memory_value, _boundary_crossed) = addressing::indirect_x(arguments, message_bus, register);

        let result = alu::or(register.a(), memory_value, register);
        register.set_accumulator(result);

        return 6;
    }

    fn call_indirect_y(&self, arguments: Vec<u8>, register: &mut Register, message_bus: &mut MessageBus) -> u8 {
        let (_memory_address, memory_value, boundary_crossed) = addressing::indirect_y(arguments, message_bus, register);

        let result = alu::or(register.a(), memory_value, register);
        register.set_accumulator(result);

        return if boundary_crossed { 6u8 } else { 5u8 };
    }
}

#[cfg(test)]
mod tests {
    use super::Ora;
    use crate::cpu::mnemonics::Mnemonic;
    use crate::cpu::register::Register;
    use crate::memory::Memory;
    use crate::message_bus::MessageBus;

    #[test]
    fn test_immidiate() {
        let ora = Ora::new(0x09);
        let arguments = vec![0b1010_0101];
        let mut memory = Memory::new();
        let mut register = Register::new();
        register.set_accumulator(0b0101_0101);

        let mut message_bus = MessageBus::new(&mut memory);

        let cycles = ora.call(arguments, &mut register, &mut message_bus);

        assert_eq!(register.a(), 0b1111_0101);
        assert_eq!(register.p(), 0b1010_0000);
        assert_eq!(cycles, 2);
    }

    #[test]
    fn test_zero_page() {
        let ora = Ora::new(0x05);
        let arguments = vec![0x30];
        let mut memory = Memory::new();
        memory.write_byte(0x30, 0b1010_0101);

        let mut register = Register::new();
        register.set_accumulator(0b0101_0101);

        let mut message_bus = MessageBus::new(&mut memory);

        let cycles = ora.call(arguments, &mut register, &mut message_bus);

        assert_eq!(register.a(), 0b1111_0101);
        assert_eq!(register.p(), 0b1010_0000);
        assert_eq!(cycles, 3);
    }

    #[test]
    fn test_zero_page_x() {
        let ora = Ora::new(0x15);
        let arguments = vec![0x30];
        let mut memory = Memory::new();
        memory.write_byte(0x35, 0b1010_0101);

        let mut register = Register::new();
        register.set_accumulator(0b0101_0101);
        register.set_x(0x05);

        let mut message_bus = MessageBus::new(&mut memory);

        let cycles = ora.call(arguments, &mut register, &mut message_bus);

        assert_eq!(register.a(), 0b1111_0101);
        assert_eq!(register.p(), 0b1010_0000);
        assert_eq!(cycles, 4);
    }

    #[test]
    fn test_absolute() {
        let ora = Ora::new(0x0D);
        let arguments = vec![0x3c, 0x5a];
        let mut memory = Memory::new();
        memory.write_byte(0x5a3c, 0b1010_0101);

        let mut register = Register::new();
        register.set_accumulator(0b0101_0101);

        let mut message_bus = MessageBus::new(&mut memory);

        let cycles = ora.call(arguments, &mut register, &mut message_bus);

        assert_eq!(register.a(), 0b1111_0101);
        assert_eq!(register.p(), 0b1010_0000);
        assert_eq!(cycles, 4);
    }

    #[test]
    fn test_absolute_x() {
        let ora = Ora::new(0x1D);
        let arguments = vec![0x3c, 0x5a];
        let mut memory = Memory::new();
        memory.write_byte(0x5a4c, 0b1010_0101);

        let mut register = Register::new();
        register.set_accumulator(0b0101_0101);
        register.set_x(0x10);
        register.set_y(0x20);

        let mut message_bus = MessageBus::new(&mut memory);

        let cycles = ora.call(arguments, &mut register, &mut message_bus);

        assert_eq!(register.a(), 0b1111_0101);
        assert_eq!(register.p(), 0b1010_0000);
        assert_eq!(cycles, 4);
    }

    #[test]
    fn test_absolute_x_out_of_bounds() {
        let ora = Ora::new(0x1D);
        let arguments = vec![0xff, 0xff];
        let mut memory = Memory::new();
        memory.write_byte(0x5a, 0b1010_0101);

        let mut register = Register::new();
        register.set_accumulator(0b0101_0101);
        register.set_x(0x5b);
        register.set_y(0x20);

        let mut message_bus = MessageBus::new(&mut memory);

        let cycles = ora.call(arguments, &mut register, &mut message_bus);

        assert_eq!(register.a(), 0b1111_0101);
        assert_eq!(register.p(), 0b1010_0000);
        assert_eq!(cycles, 5);
    }

    #[test]
    fn test_absolute_y() {
        let ora = Ora::new(0x19);
        let arguments = vec![0x3c, 0x5a];
        let mut memory = Memory::new();
        memory.write_byte(0x5a4c, 0b1010_0101);

        let mut register = Register::new();
        register.set_accumulator(0b0101_0101);
        register.set_x(0x20);
        register.set_y(0x10);

        let mut message_bus = MessageBus::new(&mut memory);

        let cycles = ora.call(arguments, &mut register, &mut message_bus);

        assert_eq!(register.a(), 0b1111_0101);
        assert_eq!(register.p(), 0b1010_0000);
        assert_eq!(cycles, 4);
    }

    #[test]
    fn test_absolute_y_out_of_bounds() {
        let ora = Ora::new(0x19);
        let arguments = vec![0xff, 0xff];
        let mut memory = Memory::new();
        memory.write_byte(0x5a, 0b1010_0101);

        let mut register = Register::new();
        register.set_accumulator(0b0101_0101);
        register.set_x(0x20);
        register.set_y(0x5b);

        let mut message_bus = MessageBus::new(&mut memory);

        let cycles = ora.call(arguments, &mut register, &mut message_bus);

        assert_eq!(register.a(), 0b1111_0101);
        assert_eq!(register.p(), 0b1010_0000);
        assert_eq!(cycles, 5);
    }

    #[test]
    fn test_indirect_x() {
        let ora = Ora::new(0x01);
        let arguments = vec![0x44];
        let mut memory = Memory::new();
        memory.write_byte(0x77, 0x05);
        memory.write_byte(0x78, 0x01);
        memory.write_byte(0x0105, 0b1010_0101);

        let mut register = Register::new();
        register.set_accumulator(0b0101_0101);
        register.set_x(0x33);

        let mut message_bus = MessageBus::new(&mut memory);

        let cycles = ora.call(arguments, &mut register, &mut message_bus);

        assert_eq!(register.a(), 0b1111_0101);
        assert_eq!(register.p(), 0b1010_0000);
        assert_eq!(cycles, 6);
    }

    #[test]
    fn test_indirect_y() {
        let ora = Ora::new(0x11);
        let arguments = vec![0x77];
        let mut memory = Memory::new();
        memory.write_byte(0x77, 0x05);
        memory.write_byte(0x78, 0x01);
        memory.write_byte(0x0109, 0b1010_0101);

        let mut register = Register::new();
        register.set_accumulator(0b0101_0101);
        register.set_y(0x04);

        let mut message_bus = MessageBus::new(&mut memory);

        let cycles = ora.call(arguments, &mut register, &mut message_bus);

        assert_eq!(register.a(), 0b1111_0101);
        assert_eq!(register.p(), 0b1010_0000);
        assert_eq!(cycles, 5);
    }

    #[test]
    fn test_indirect_y_out_of_bonds() {
        let ora = Ora::new(0x11);
        let arguments = vec![0x77];
        let mut memory = Memory::new();
        memory.write_byte(0x77, 0xff);
        memory.write_byte(0x78, 0x01);
        memory.write_byte(0x0205, 0b1010_0101);

        let mut register = Register::new();
        register.set_accumulator(0b0101_0101);
        register.set_y(0x06);

        let mut message_bus = MessageBus::new(&mut memory);

        let cycles = ora.call(arguments, &mut register, &mut message_bus);

        assert_eq!(register.a(), 0b1111_0101);
        assert_eq!(register.p(), 0b1010_0000);
        assert_eq!(cycles, 6);
    }

    #[test]
    #[should_panic]
    fn test_invalid_opcode() {
        let ora = Ora::new(0x00);
        let arguments = vec![0xFF];
        let mut memory = Memory::new();
        let mut message_bus = MessageBus::new(&mut memory);
        let mut register = Register::new();

        ora.call(arguments, &mut register, &mut message_bus);
    }
}


