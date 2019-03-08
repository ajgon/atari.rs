use super::cpu::Cpu;
use super::memory::Memory;

#[derive(Debug)]
pub struct Atari {
    cpu: Cpu,
    memory: Memory
}

impl Atari {
    pub fn new() -> Atari {
        return Atari { cpu: Cpu::new(), memory: Memory::new() }
    }
    pub fn load_into_memory(&mut self, data: &str) {
        for (i, byte) in data.bytes().enumerate() {
            self.memory.write_byte(i, byte);
        }

        self.cpu.process_byte(0x69);
        self.cpu.process_byte(0x2A);
        self.cpu.process_byte(0x69);
        self.cpu.process_byte(0x45);
    }
}
