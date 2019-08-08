extern crate sdl2;

use sdl2::keyboard::Keycode;
use sdl2::event::Event;
use sdl2::EventPump;
use sdl2::Sdl;

pub struct Keyboard {
    sdl_event_pump: EventPump,
    pub keys_array: [bool; 16],
}

impl Keyboard {
    pub fn new(ctx: &Sdl) -> Keyboard {
        let pump = ctx.event_pump().unwrap();
        Keyboard { 
            sdl_event_pump: pump,
            keys_array: [false; 16],
        }
    }

    pub fn update_keys(&mut self) {
        for event in self.sdl_event_pump.poll_iter() {
            match event {
                Event::KeyDown {
		            keycode: Some(Keycode::Num1), .. 
                } => {
                    self.keys_array[0] = true;
                },

                Event::KeyDown {
		            keycode: Some(Keycode::Num2), .. 
                } => {
                    self.keys_array[1] = true;
                },

                Event::KeyDown {
		            keycode: Some(Keycode::Num3), .. 
                } => {
                    self.keys_array[2] = true;
                },

                Event::KeyDown {
		            keycode: Some(Keycode::Num4), .. 
                } => {
                    self.keys_array[3] = true;
                },

                Event::KeyDown {
		            keycode: Some(Keycode::Q), .. 
                } => {
                    self.keys_array[4] = true;
                },

                Event::KeyDown {
		            keycode: Some(Keycode::W), .. 
                } => {
                    self.keys_array[5] = true;
                },

                Event::KeyDown {
		            keycode: Some(Keycode::E), .. 
                } => {
                    self.keys_array[6] = true;
                },

                Event::KeyDown {
		            keycode: Some(Keycode::R), .. 
                } => {
                    self.keys_array[7] = true;
                },

                Event::KeyDown {
		            keycode: Some(Keycode::A), .. 
                } => {
                    self.keys_array[8] = true;
                },

                Event::KeyDown {
		            keycode: Some(Keycode::S), .. 
                } => {
                    self.keys_array[9] = true;
                },

                Event::KeyDown {
		            keycode: Some(Keycode::D), .. 
                } => {
                    self.keys_array[10] = true;
                },

                Event::KeyDown {
		            keycode: Some(Keycode::F), .. 
                } => {
                    self.keys_array[11] = true;
                },

                Event::KeyDown {
		            keycode: Some(Keycode::Z), .. 
                } => {
                    self.keys_array[12] = true;
                },

                Event::KeyDown {
		            keycode: Some(Keycode::X), .. 
                } => {
                    self.keys_array[13] = true;
                },

                Event::KeyDown {
		            keycode: Some(Keycode::C), .. 
                } => {
                    self.keys_array[14] = true;
                },

                Event::KeyDown {
		            keycode: Some(Keycode::V), .. 
                } => {
                    self.keys_array[15] = true;
                },

                Event::KeyUp {
		            keycode: Some(Keycode::Num1), .. 
                } => {
                    self.keys_array[0] = false;
                },

                Event::KeyUp {
		            keycode: Some(Keycode::Num2), .. 
                } => {
                    self.keys_array[1] = false;
                },

                Event::KeyUp {
		            keycode: Some(Keycode::Num3), .. 
                } => {
                    self.keys_array[2] = false;
                },

                Event::KeyUp {
		            keycode: Some(Keycode::Num4), .. 
                } => {
                    self.keys_array[3] = false;
                },

                Event::KeyUp {
		            keycode: Some(Keycode::Q), .. 
                } => {
                    self.keys_array[4] = false;
                },

                Event::KeyUp {
		            keycode: Some(Keycode::W), .. 
                } => {
                    self.keys_array[5] = false;
                },

                Event::KeyUp {
		            keycode: Some(Keycode::E), .. 
                } => {
                    self.keys_array[6] = false;
                },

                Event::KeyUp {
		            keycode: Some(Keycode::R), .. 
                } => {
                    self.keys_array[7] = false;
                },

                Event::KeyUp {
		            keycode: Some(Keycode::A), .. 
                } => {
                    self.keys_array[8] = false;
                },

                Event::KeyUp {
		            keycode: Some(Keycode::S), .. 
                } => {
                    self.keys_array[9] = false;
                },

                Event::KeyUp {
		            keycode: Some(Keycode::D), .. 
                } => {
                    self.keys_array[10] = false;
                },

                Event::KeyUp {
		            keycode: Some(Keycode::F), .. 
                } => {
                    self.keys_array[11] = false;
                },

                Event::KeyUp {
		            keycode: Some(Keycode::Z), .. 
                } => {
                    self.keys_array[12] = false;
                },

                Event::KeyUp {
		            keycode: Some(Keycode::X), .. 
                } => {
                    self.keys_array[13] = false;
                },

                Event::KeyUp {
		            keycode: Some(Keycode::C), .. 
                } => {
                    self.keys_array[14] = false;
                },

                Event::KeyUp {
		            keycode: Some(Keycode::V), .. 
                } => {
                    self.keys_array[15] = false;
                },


                Event::Quit { .. } => {
                    std::process::exit(0);
                }

                _ => {}
            }
        }

        println!("keyboard keys: {:?}", self.keys_array);
    }
}