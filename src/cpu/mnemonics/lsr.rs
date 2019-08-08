/*
LSR  Shift One Bit Right (Memory or Accumulator)

     0 -> [76543210] -> C             N Z C I D V
                                      0 + + - - -

     addressing    assembler    opc  bytes  cyles
     --------------------------------------------
     accumulator   LSR A         4A    1     2
     zeropage      LSR oper      46    2     5
     zeropage,X    LSR oper,X    56    2     6
     absolute      LSR oper      4E    3     6
     absolute,X    LSR oper,X    5E    3     7
*/
use crate::cpu::addressing;
use crate::cpu::mnemonics::Mnemonic;
use crate::cpu::register::Register;
use crate::message_bus::MessageBus;
use crate::cpu::alu;

#[derive(Debug)]
pub struct Lsr {
    mnemonic: String,
    opcode: u8
}

impl Lsr {
    pub fn new(opcode: u8) -> Lsr {
        return Lsr { mnemonic: "LSR".to_string(), opcode: opcode };
    }
}

impl Mnemonic for Lsr {
    fn determine_bytes(&self) -> usize {
        return match self.opcode {
            0x4A => 1,
            0x46 => 2,
            0x56 => 2,
            0x4E => 3,
            0x5E => 3,
            _ => panic!("Invalid opcode `0x{:x}` for mnemonic {}", self.opcode, self.mnemonic)
        }
    }

    fn call(&self, arguments: Vec<u8>, register: &mut Register, message_bus: &mut MessageBus) -> u8 {
        match self.opcode {
            0x4A => return self.call_accumulator(register),
            0x46 => return self.call_zero_page(arguments, register, message_bus),
            0x56 => return self.call_zero_page_x(arguments, register, message_bus),
            0x4E => return self.call_absolute(arguments, register, message_bus),
            0x5E => return self.call_absolute_x(arguments, register, message_bus),
            _ => panic!("Invalid opcode `0x{:x}` for mnemonic {}", self.opcode, self.mnemonic)
        }
    }

    fn call_accumulator(&self, register: &mut Register) -> u8 {
        let result = alu::shift_right(register.a(), register);
        register.set_accumulator(result);

        return 2;
    }

    fn call_zero_page(&self, arguments: Vec<u8>, register: &mut Register, message_bus: &mut MessageBus) -> u8 {
        let (_memory_address, memory_value, _boundary_crossed) = addressing::zero_page(arguments, message_bus);

        let result = alu::shift_right(memory_value, register);
        register.set_accumulator(result);

        return 5;
    }

    fn call_zero_page_x(&self, arguments: Vec<u8>, register: &mut Register, message_bus: &mut MessageBus) -> u8 {
        let (_memory_address, memory_value, _boundary_crossed) = addressing::zero_page_x(arguments, message_bus, register);

        let result = alu::shift_right(memory_value, register);
        register.set_accumulator(result);

        return 6;
    }

    fn call_absolute(&self, arguments: Vec<u8>, register: &mut Register, message_bus: &mut MessageBus) -> u8 {
        let (_memory_address, memory_value, _boundary_crossed) = addressing::absolute(arguments, message_bus);

        let result = alu::shift_right(memory_value, register);
        register.set_accumulator(result);

        return 6;
    }

    fn call_absolute_x(&self, arguments: Vec<u8>, register: &mut Register, message_bus: &mut MessageBus) -> u8 {
        let (_memory_address, memory_value, _boundary_crossed) = addressing::absolute_x(arguments, message_bus, register);

        let result = alu::shift_right(memory_value, register);
        register.set_accumulator(result);

        return 7;
    }
}

#[cfg(test)]
mod tests {
    use super::Lsr;
    use crate::cpu::mnemonics::Mnemonic;
    use crate::cpu::register::Register;
    use crate::memory::Memory;
    use crate::message_bus::MessageBus;

    #[test]
    fn test_accumulator() {
        let lsr = Lsr::new(0x4A);
        let mut memory = Memory::new();
        let mut register = Register::new();
        register.set_accumulator(0b0010_1100);

        let mut message_bus = MessageBus::new(&mut memory);

        let cycles = lsr.call(vec![register.a()], &mut register, &mut message_bus);

        assert_eq!(register.a(), 0b0001_0110);
        assert_eq!(register.p(), 0b0011_0000);
        assert_eq!(cycles, 2);
    }

    #[test]
    fn test_zero_page() {
        let lsr = Lsr::new(0x46);
        let arguments = vec![0x30];
        let mut memory = Memory::new();
        memory.write_byte(0x30, 0b0010_1100);

        let mut register = Register::new();

        let mut message_bus = MessageBus::new(&mut memory);

        let cycles = lsr.call(arguments, &mut register, &mut message_bus);

        assert_eq!(register.a(), 0b0001_0110);
        assert_eq!(register.p(), 0b0011_0000);
        assert_eq!(cycles, 5);
    }

    #[test]
    fn test_zero_page_x() {
        let lsr = Lsr::new(0x56);
        let arguments = vec![0x30];
        let mut memory = Memory::new();
        memory.write_byte(0x35, 0b0010_1100);

        let mut register = Register::new();
        register.set_x(0x05);

        let mut message_bus = MessageBus::new(&mut memory);

        let cycles = lsr.call(arguments, &mut register, &mut message_bus);

        assert_eq!(register.a(), 0b0001_0110);
        assert_eq!(register.p(), 0b0011_0000);
        assert_eq!(cycles, 6);
    }

    #[test]
    fn test_absolute() {
        let lsr = Lsr::new(0x4E);
        let arguments = vec![0x3c, 0x5a];
        let mut memory = Memory::new();
        memory.write_byte(0x5a3c, 0b0010_1100);

        let mut register = Register::new();

        let mut message_bus = MessageBus::new(&mut memory);

        let cycles = lsr.call(arguments, &mut register, &mut message_bus);

        assert_eq!(register.a(), 0b0001_0110);
        assert_eq!(register.p(), 0b0011_0000);
        assert_eq!(cycles, 6);
    }

    #[test]
    fn test_absolute_x() {
        let lsr = Lsr::new(0x5E);
        let arguments = vec![0x3c, 0x5a];
        let mut memory = Memory::new();
        memory.write_byte(0x5a4c, 0b0010_1100);

        let mut register = Register::new();
        register.set_x(0x10);
        register.set_y(0x20);

        let mut message_bus = MessageBus::new(&mut memory);

        let cycles = lsr.call(arguments, &mut register, &mut message_bus);

        assert_eq!(register.a(), 0b0001_0110);
        assert_eq!(register.p(), 0b0011_0000);
        assert_eq!(cycles, 7);
    }

    #[test]
    #[should_panic]
    fn test_invalid_opcode() {
        let lsr = Lsr::new(0x00);
        let arguments = vec![0xFF];
        let mut memory = Memory::new();
        let mut message_bus = MessageBus::new(&mut memory);
        let mut register = Register::new();

        lsr.call(arguments, &mut register, &mut message_bus);
    }
}



