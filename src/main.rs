mod cpu;
mod memory;
mod atari;
use atari::Atari;

fn main() {
    let mut atari = Atari::new();
    atari.load_into_memory("\x69\x2A\x69\x45");

    //println!("{:?}", motherboard());
}
