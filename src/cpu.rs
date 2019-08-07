extern crate rand;

use rand::Rng;

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
    // Add timer, display and sound here
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

    fn handle_0x0000_opcode(&self, cpu: &mut Cpu) {
        match self.opcode & 0x000F {
            0x0 => {
                // Clear display
            },

            0xE => { //
                cpu.pc = cpu.stack[cpu.sp];
                cpu.stack[cpu.sp] = 0;
                cpu.sp -= 1;
            },

            _ => panic!("Invalid opcode {:?}", self.opcode),
        }
    }

    fn execute_opcode(&self, cpu: &mut Cpu) {
        match self.opcode & 0xF000 {
            //0x0000 => self.handle_0x0000_opcode(cpu),
            0x1000 => { // JP addr
                cpu.pc = self.get_address()
            },

            0x2000 => { // CALL addr
                cpu.stack[cpu.pc] = cpu.pc + std::mem::size_of::<Opcode>();
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
                cpu.v[self.get_x_value()] += self.get_lower_byte();
            },

            //0x8000 => self.handle_0x8000_opcode(cpu),
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

            0xC000 => { // RND Vx, byte -> Vx = random(0,255) & byte 
                let mut rng = rand::thread_rng();
                cpu.v[self.get_x_value()] = rng.gen_range(0, 255) & self.get_lower_byte();
            },

            // 0xD000 => { // Display stuff
            // }

            //0xE000 => self.handle_0xe000_opcode(cpu),
            //0xF000 => self.handle_0xf000_opcode(cpu),

            _ => panic!("Invalid opcode {:?}", self.opcode),
        }
    }
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            v: [0; 0x10],
            i: 0,
            pc: 0,
            sp: 0,
            stack: [0; STACK_SIZE],
            memory: [0; RAM_SIZE]
        }
    }
}