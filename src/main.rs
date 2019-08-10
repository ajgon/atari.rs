mod cpu;
use cpu::Cpu;
use std::io::prelude::*;
use std::fs::File;
use std::time::{Duration, Instant};

fn main() {
    let mut memory = [0; 65536];

    let mut file = File::open("examples/test.mem").unwrap();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer);
    for i in (0..65536) {
        memory[i] = buffer[i];
    }

    let mut cpu = Cpu::new(&mut memory);

    //cpu.debug();
    cpu.cold_reset();
    let now = Instant::now();
    let mut elapsed = now.elapsed().as_secs();

    while cpu.step() {
        let new_elapsed = now.elapsed().as_secs();

        if (new_elapsed != elapsed) {
            elapsed = new_elapsed;
            println!("Used cycles: {}", cpu.cycles);
        }
    }
    println!("Used cycles: {}", cpu.cycles);
}
