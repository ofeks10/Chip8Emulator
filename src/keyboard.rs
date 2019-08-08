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
		            keycode: Some(Keycode::Down), .. 
                } => {
                    self.keys_array[0] = true;
                },

                Event::KeyDown {
		            keycode: Some(Keycode::Up), .. 
                } => {
                    self.keys_array[1] = true;
                },

                Event::KeyUp {
                    keycode: Some(Keycode::Down), ..
                } => {
                    self.keys_array[0] = false;
                },

                Event::KeyUp {
                    keycode: Some(Keycode::Up), ..
                } => {
                    self.keys_array[1] = false;
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