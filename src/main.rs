mod cartridge;
mod cpu;

fn main() {
    let cartridge = cartridge::Cartridge::new("/home/ofeks10/Chip8Games/pong.ch8");
    let mut cpu = cpu::Cpu::new(&cartridge);

    println!("Hello, world!");
}
