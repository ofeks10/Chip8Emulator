extern crate rand;

use crate::display::{Display, DISPLAY_WIDTH, DISPLAY_HEIGHT};
use crate::cartridge::Cartridge;
use crate::timer::Timer;
use crate::keyboard::Keyboard;
use crate::sound::Sound;
use rand::Rng;

const LETTERS_SPRITES: [u8; 80] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, 0x20, 0x60, 0x20,
    0x20, 0x70, 0xF0, 0x10, 0xF0, 0x80, 0xF0, 0xF0,
    0x10, 0xF0, 0x10, 0xF0, 0x90, 0x90, 0xF0, 0x10,
    0x10, 0xF0, 0x80, 0xF0, 0x10, 0xF0, 0xF0, 0x80,
    0xF0, 0x90, 0xF0, 0xF0, 0x10, 0x20, 0x40, 0x40,
    0xF0, 0x90, 0xF0, 0x90, 0xF0, 0xF0, 0x90, 0xF0,
    0x10, 0xF0, 0xF0, 0x90, 0xF0, 0x90, 0x90, 0xE0,
    0x90, 0xE0, 0x90, 0xE0, 0xF0, 0x80, 0x80, 0x80,
    0xF0, 0xE0, 0x90, 0x90, 0x90, 0xE0, 0xF0, 0x80,
    0xF0, 0x80, 0xF0, 0xF0, 0x80, 0xF0, 0x80, 0x80
];

const RAM_SIZE: usize = 0x1000;
const STACK_SIZE: usize = 16;
const START_OF_PROGRAM: usize = 0x200;

pub struct Cpu {
    v: [u8; 16],
    i: usize,
    pc: usize,
    sp: usize,
    stack: [usize; STACK_SIZE],
    memory: [u8; RAM_SIZE],
    display: Display,
    timer_for_sound: Timer,
    timer: Timer,
    keyboard: Keyboard,
    sound_device: Sound,
}

struct Opcode {
    opcode: u16,
}

impl Opcode {
    fn new(opcode: u16) -> Opcode {
        Opcode { opcode }
    }

    fn get_address(&self) -> usize {
        (self.opcode & 0x0FFF) as usize
    }

    fn get_lower_byte(&self) -> u8 {
        (self.opcode & 0x00FF) as u8
    }

    // For example, for the number 0x0FED:
    // nibble 0 will be 0xD
    // nibble 1 will be 0xE
    // nibble 2 will be 0xF
    fn get_nibble(&self, nibble: u8) -> usize {
        ((self.opcode >> (12 - (12 - nibble * 4))) & 0xF) as usize
    }

    fn get_n_value(&self) -> usize {
        self.get_nibble(0)
    }

    fn get_y_value(&self) -> usize {
        self.get_nibble(1)
    }

    fn get_x_value(&self) -> usize {
        self.get_nibble(2)
    }

    fn execute_opcode(&self, cpu: &mut Cpu) {
        match self.opcode & 0xF000 {
            0x0000 => self.handle_0x0000_opcode(cpu),
            0x1000 => { // JP addr
                cpu.pc = self.get_address()
            },

            0x2000 => { // CALL addr
                cpu.stack[cpu.sp] = cpu.pc;
                cpu.sp += 1;
                cpu.pc = self.get_address();
            },

            0x3000 => { // SE Vx, byte -> if Vx == byte, incrase pc by 2
                if self.get_lower_byte() == cpu.v[self.get_x_value()] {
                    cpu.pc += std::mem::size_of::<Opcode>();
                }
            },

            0x4000 => { //SNE Vx, byte -> if Vx != byte, incrase pc by 2
                if self.get_lower_byte() != cpu.v[self.get_x_value()] {
                    cpu.pc += std::mem::size_of::<Opcode>();
                }
            },

            0x5000 => { // SE Vx, Vy
                if cpu.v[self.get_y_value()] == cpu.v[self.get_x_value()] {
                    cpu.pc += std::mem::size_of::<Opcode>();
                }
            },

            0x6000 => { // LD Vx, byte
                cpu.v[self.get_x_value()] = self.get_lower_byte();
            },

            0x7000 => { // ADD Vx, byte
                let vx = cpu.v[self.get_x_value()] as u16;
                let val = self.get_lower_byte() as u16;
                let result = vx + val;
                cpu.v[self.get_x_value()] = result as u8;
            },

            0x8000 => self.handle_0x8000_opcode(cpu),
            0x9000 => { //SNE Vx, Vy
                if cpu.v[self.get_y_value()] != cpu.v[self.get_x_value()] {
                    cpu.pc += std::mem::size_of::<Opcode>();
                }
            },

            0xA000 => { // LD I, addr
                cpu.i = self.get_address();
            },

            0xB000 => { // JP V0, addr
                cpu.pc = self.get_address() + cpu.v[0] as usize;
            },

            0xC000 => { // RND Vx, byte -> Vx = random(0->255) & byte 
                let mut rng = rand::thread_rng();
                cpu.v[self.get_x_value()] = rng.gen_range(0, 255) & self.get_lower_byte();
            },

            0xD000 => { // Display stuff
                let x = self.get_x_value();
                let y = self.get_y_value();
                let n = self.get_n_value();
                cpu.v[0xF] = 0;
                for byte in 0..n {
                    let y = (cpu.v[y] as usize + byte) % DISPLAY_HEIGHT;
                    for bit in 0..8 {
                        let x = (cpu.v[x] as usize + bit) % DISPLAY_WIDTH;
                        let color = (cpu.memory[cpu.i + byte] >> (7 - bit)) & 1;
                        cpu.v[0xF] |= color & cpu.display.vram[y][x];
                        cpu.display.vram[y][x] ^= color;
                    }
                }

                cpu.display.should_render = true;
            }

            0xE000 => self.handle_0xe000_opcode(cpu),
            0xF000 => self.handle_0xf000_opcode(cpu),

            _ => panic!("Invalid opcode {:x}", self.opcode),
        }
    }

    fn handle_0x0000_opcode(&self, cpu: &mut Cpu) {
        match self.opcode & 0x000F {
            0x0 => {
                // Clear display
                cpu.display.clear();
            },

            0xE => {
                cpu.sp -= 1;
                cpu.pc = cpu.stack[cpu.sp];
                cpu.stack[cpu.sp] = 0;
            },

            _ => panic!("Invalid opcode {:?}", self.opcode),
        }
    }

    fn handle_0x8000_opcode(&self, cpu: &mut Cpu) {
        let x = self.get_x_value();
        let y = self.get_y_value();
        match self.opcode & 0x000F {
            0x0 => cpu.v[x] = cpu.v[y],
            0x1 => cpu.v[x] |= cpu.v[y],
            0x2 => cpu.v[x] &= cpu.v[y],
            0x3 => cpu.v[x] ^= cpu.v[y],
            0x4 => {
                let res = cpu.v[x] as u16 + cpu.v[y] as u16;
                cpu.v[0xF] = if res > 0xFF { 1 } else { 0 };
                cpu.v[x] = (res & 0x00FF) as u8
            },
            0x5 => {
                cpu.v[0xF] = if cpu.v[x] > cpu.v[y] { 1 } else { 0 };
                cpu.v[x] = cpu.v[x].wrapping_sub(cpu.v[y])
            },
            0x6 => {
                cpu.v[0xF] = cpu.v[x] & 1;
                cpu.v[x] >>= 1;
            },
            0x7 => {
                cpu.v[0xF] = if cpu.v[x] < cpu.v[y] { 1 } else { 0 };
                cpu.v[x] = cpu.v[y].wrapping_sub(cpu.v[x])
            },
            0xE => {
                cpu.v[0xF] = cpu.v[x] >> 7;
                cpu.v[x] <<= 1;
            },
            _ => panic!("Invalid opcode {:?}", self.opcode),
        }
    }

    fn handle_0xe000_opcode(&self, cpu: &mut Cpu) {
        match self.opcode & 0x00FF {
            0x9E => {
                if cpu.keyboard.keys_array[self.get_x_value()] == true {
                    cpu.pc += std::mem::size_of::<Opcode>();
                }
            },

            0xA1 => { // SKNP Vx
                if cpu.keyboard.keys_array[self.get_x_value()] == false {
                    cpu.pc += std::mem::size_of::<Opcode>();
                }
            },

            _ => panic!("Invalid opcode {:x}", self.opcode),
        }
    }

    fn handle_0xf000_opcode(&self, cpu: &mut Cpu) {
        match self.opcode & 0xFF {
            0x07 => {
                cpu.v[self.get_x_value()] = cpu.timer.timer_value
            },

            0x0A => {
                loop {  
                    cpu.keyboard.update_keys();
                    for i in 0..cpu.keyboard.keys_array.len() {
                        if cpu.keyboard.keys_array[i] == true {
                            return;
                        }
                    }
                }
            }, 

            0x15 => {
                cpu.timer.timer_value = cpu.v[self.get_x_value()]
            }, 

            0x18 => {
                cpu.timer_for_sound.timer_value = cpu.v[self.get_x_value()]
            },
            
            0x1E => {
                cpu.i += cpu.v[self.get_x_value()] as usize;
                cpu.v[0xF] = if cpu.i > 0xF00 { 1 } else { 0 };
            },

            0x29 => {
                cpu.i = (cpu.v[self.get_x_value()] as usize) * 5
            },

            0x33 => {
                cpu.memory[cpu.i] = cpu.v[self.get_x_value()] / 100;
                cpu.memory[cpu.i + 1] = (cpu.v[self.get_x_value()] % 100) / 10;
                cpu.memory[cpu.i + 2] = cpu.v[self.get_x_value()] % 10;
            },

            0x55 => {
                for reg_index in 0..self.get_x_value() + 1 {
                    cpu.memory[cpu.i + reg_index] = cpu.v[reg_index];
                }
            },

            0x65 => {
                for reg_index in 0..self.get_x_value() + 1 {
                    cpu.v[reg_index] = cpu.memory[cpu.i + reg_index];
                }
            },

            _ => panic!("Invalid opcode {:x}", self.opcode),
        }
    }
}

impl Cpu {
    pub fn new(cartridge: &Cartridge) -> Cpu {
        let ctx = sdl2::init().unwrap();
        let mut cpu = Cpu {
            v: [0; 0x10],
            i: 0,
            pc: START_OF_PROGRAM,
            sp: 0,
            stack: [0; STACK_SIZE],
            memory: [0; RAM_SIZE],
            display: Display::new(&ctx),
            timer_for_sound: Timer::new_without_callback(),
            timer: Timer::new_without_callback(),
            keyboard: Keyboard::new(&ctx),
            sound_device: Sound::new(&ctx),
        };

        for i in 0..cartridge.size {
            cpu.memory[START_OF_PROGRAM + i] = cartridge.contents[i];
        }

        for i in 0..LETTERS_SPRITES.len() {
            cpu.memory[i] = LETTERS_SPRITES[i];
        }

        cpu
    }

    fn get_next_opcode(&mut self) -> Opcode {
        let opcode: u16 = (self.memory[self.pc] as u16) << 8 | (self.memory[self.pc + 1] as u16);
        let op = Opcode::new(opcode);
        self.pc += 2;
        op
    }

    fn run_next_opcode(&mut self) {
        let opcode: Opcode = self.get_next_opcode();
        self.print_everything();
        opcode.execute_opcode(self);
    }

    fn print_everything(&mut self) {
        println!("Registers:\n{:?}", self.v);
        println!("PC:\t{:?}", self.pc);
        println!("SP:\t{:?}", self.sp);
        println!("I:\t{:?}", self.i);
    }

    pub fn tick(&mut self) {
        self.keyboard.update_keys();
        self.run_next_opcode();

        self.timer.tick();

        if self.timer_for_sound.timer_value > 0 {
            self.sound_device.start_beep();
        } else { 
            self.sound_device.stop_beep();
        }

        self.timer_for_sound.tick();

        if self.display.should_render {
            self.display.render();
        }
    }
}