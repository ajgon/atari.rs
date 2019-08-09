mod message_bus;
mod cpu;
mod memory;
mod atari;
use atari::Atari;

fn main() {
    let mut memory = memory::Memory::new();
    let mut message_bus = message_bus::MessageBus::new(&mut memory);
    let mut cpu = cpu::Cpu::new(&mut message_bus);

    let mut atari = Atari::new(&mut cpu);
    atari.start();
    //atari.load_into_memory("\x69\x2A\x69\x45");

    //println!("{:?}", motherboard());
}
