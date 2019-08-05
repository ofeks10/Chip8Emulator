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
    fn get_nibble(&self, nibble: u8) -> u8 {
        ((self.opcode >> (12 - (12 - nibble * 4))) & 0xF) as u8
    }

    fn get_n_value(&self) -> u8 {
        self.get_nibble(0)
    }

    fn get_x_value(&self) -> u8 {
        self.get_nibble(1)
    }

    fn get_y_value(&self) -> u8 {
        self.get_nibble(2)
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

            _ => { panic!("error");},
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