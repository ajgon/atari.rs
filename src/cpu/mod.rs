mod mnemonics;
mod register;
mod addressing;
mod alu;

use mnemonics::Mnemonics;
use register::Register;
use crate::message_bus::MessageBus;

#[derive(Debug)]
pub struct Cpu<'a> {
    message_bus: &'a mut MessageBus<'a>,
    register: register::Register,
    mnemonics: Mnemonics,
    cycles: u64
}

impl<'a> Cpu<'a> {
    pub fn new(message_bus: &'a mut MessageBus<'a>) -> Cpu<'a> {
        return Cpu {
            message_bus: message_bus,
            register: Register::new(),
            mnemonics: Mnemonics::new(),
            cycles: 0
        };
    }

    pub fn cold_reset(&mut self) {
        let (_memory_address, pch, _boundary_crossed) = addressing::absolute(vec![0xfd, 0xff], self.message_bus);
        let (_memory_address, pcl, _boundary_crossed) = addressing::absolute(vec![0xfc, 0xff], self.message_bus);

        self.register.set_accumulator(0x00);
        self.register.set_x(0x00);
        self.register.set_y(0x00);
        self.register.set_p(0b0010_0100); // Interrupt flag
        self.register.set_pc(((pch as u16) << 8) + pcl as u16);
    }

    pub fn warm_reset(&mut self) {
        let (_memory_address, pch, _boundary_crossed) = addressing::absolute(vec![0xfd, 0xff], self.message_bus);
        let (_memory_address, pcl, _boundary_crossed) = addressing::absolute(vec![0xfc, 0xff], self.message_bus);

        self.register.set_interrupt_bit(true);
        self.register.set_pc(((pch as u16) << 8) + pcl as u16);
    }

    pub fn step(&mut self) {
        let opcode = self.read_byte();
        let mnemonic = self.mnemonics.resolve_mnemonic_from_opcode(opcode);
        let mnemonic_length = mnemonic.determine_bytes();
        let mut mnemonic_data: Vec<u8> = Vec::new();

        for _ in 0..(mnemonic_length - 1) {
            mnemonic_data.push(self.read_byte());
        }

        self.cycles += mnemonic.call(mnemonic_data, &mut self.register, &mut self.message_bus) as u64;
    }

    fn read_byte(&mut self) -> u8 {
        let (_memory_address, memory_value, _boundary_crossed) = addressing::absolute(vec![self.register.pc() as u8, (self.register.pc() >> 8) as u8], &mut self.message_bus);
        self.register.increment_pc();

        return memory_value;
    }
}
#[cfg(test)]
mod tests {
    use super::Cpu;

    use crate::message_bus::MessageBus;
    use crate::memory::Memory;

    #[test]
    fn test_cold_reset() {
        let mut memory = Memory::new();
        memory.write_byte(0xfffc, 0x01);
        memory.write_byte(0xfffd, 0x06);
        let mut message_bus = MessageBus::new(&mut memory);
        let mut cpu = Cpu::new(&mut message_bus);

        cpu.register.set_accumulator(0x42);
        cpu.register.set_x(0x33);
        cpu.register.set_y(0x22);
        cpu.register.set_p(0b1000_0000);
        cpu.register.set_pc(0x1234);
        cpu.cold_reset();

        assert_eq!(cpu.register.a(), 0x00);
        assert_eq!(cpu.register.x(), 0x00);
        assert_eq!(cpu.register.y(), 0x00);
        assert_eq!(cpu.register.p(), 0b0010_0100);
        assert_eq!(cpu.register.pc(), 0x0601);
    }

    #[test]
    fn test_warm_reset() {
        let mut memory = Memory::new();
        memory.write_byte(0xfffc, 0x01);
        memory.write_byte(0xfffd, 0x06);
        let mut message_bus = MessageBus::new(&mut memory);
        let mut cpu = Cpu::new(&mut message_bus);

        cpu.register.set_accumulator(0x42);
        cpu.register.set_x(0x33);
        cpu.register.set_y(0x22);
        cpu.register.set_p(0b1000_0000);
        cpu.register.set_pc(0x1234);
        cpu.warm_reset();

        assert_eq!(cpu.register.a(), 0x42);
        assert_eq!(cpu.register.x(), 0x33);
        assert_eq!(cpu.register.y(), 0x22);
        assert_eq!(cpu.register.p(), 0b1010_0100);
        assert_eq!(cpu.register.pc(), 0x0601);
    }

    #[test]
    fn test_step() {
        let mut memory = Memory::new();
        // LDA #$33
        memory.write_byte(0x600, 0xa9);
        memory.write_byte(0x601, 0x33);
        // LDX #$44
        memory.write_byte(0x602, 0xa2);
        memory.write_byte(0x603, 0x44);
        // LDY #$55
        memory.write_byte(0x604, 0xa0);
        memory.write_byte(0x605, 0x55);
        // PC start
        memory.write_byte(0xfffc, 0x00);
        memory.write_byte(0xfffd, 0x06);
        let mut message_bus = MessageBus::new(&mut memory);
        let mut cpu = Cpu::new(&mut message_bus);
        cpu.cold_reset();
        cpu.step();
        cpu.step();
        cpu.step();

        assert_eq!(cpu.cycles, 6);
        assert_eq!(cpu.register.a(), 0x33);
        assert_eq!(cpu.register.x(), 0x44);
        assert_eq!(cpu.register.y(), 0x55);
    }
}
