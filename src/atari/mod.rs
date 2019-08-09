use super::cpu::Cpu;
//use super::memory::Memory;

#[derive(Debug)]
pub struct Atari<'a> {
    cpu: &'a mut Cpu<'a>
}

impl<'a> Atari<'a> {
    pub fn new(cpu: &'a mut Cpu<'a>) -> Atari<'a> {
        return Atari {
            cpu: cpu
        };
    }

    pub fn start(&mut self) {
        self.cpu.cold_reset();
    }

    pub fn work(&mut self) {
        while self.cpu.step() {}
    }

    //pub fn load_into_memory(&mut self, data: &str) {
        //for (i, byte) in data.bytes().enumerate() {
            //self.memory.write_byte(i, byte);
        //}

        //self.cpu.process_byte(0x69);
        //self.cpu.process_byte(0x2A);
        //self.cpu.process_byte(0x69);
        //self.cpu.process_byte(0x45);
    //}
}
