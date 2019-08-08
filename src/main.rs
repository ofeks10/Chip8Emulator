mod cartridge;
mod cpu;
mod display;

use std::{thread, time};

fn main() {
    let cartridge = cartridge::Cartridge::new("/home/ofeks10/Chip8Games/pong.ch8");
    let mut cpu = cpu::Cpu::new(&cartridge);

    loop {
        cpu.tick();
        thread::sleep(time::Duration::from_millis(2));
    }
}
