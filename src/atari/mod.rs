use super::cpu::Cpu;
//use super::memory::Memory;
use std::time::{Duration, Instant};

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
        let now = Instant::now();
        let mut elapsed = now.elapsed().as_secs();

        while self.cpu.step() {
            let new_elapsed = now.elapsed().as_secs();

            if (new_elapsed != elapsed) {
                elapsed = new_elapsed;
                println!("Used cycles: {}", self.cpu.cycles);
            }
        }
        println!("Used cycles: {}", self.cpu.cycles);
    }
}
