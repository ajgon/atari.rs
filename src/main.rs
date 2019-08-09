mod message_bus;
mod cpu;
mod memory;
mod atari;
use atari::Atari;
use std::io::prelude::*;
use std::fs::File;

fn main() {
    let mut file = File::open("examples/tolower.mem").unwrap();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer);

    let mut memory = memory::Memory::new().contents(buffer);
    let mut message_bus = message_bus::MessageBus::new(&mut memory);
    let mut cpu = cpu::Cpu::new(&mut message_bus);
    let mut atari = Atari::new(&mut cpu);
    atari.start();
    atari.work();

    let mut file = File::create("out.dump").unwrap();
    file.write_all(&memory.dump());
}
