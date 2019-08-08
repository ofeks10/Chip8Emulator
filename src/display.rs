extern crate sdl2;
use sdl2::pixels;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

const DISPLAY_WIDTH: usize = 64;
const DISPLAY_HEIGHT: usize = 32;
const PIXEL_DENSITY: usize = 10;

pub struct Display {
    vram: [[u8; DISPLAY_HEIGHT]; DISPLAY_WIDTH],
    should_render: bool,
    driver: DisplayDriver,
}

struct DisplayDriver {
    canvas: Canvas<Window>,
}

impl DisplayDriver {
    fn new() -> DisplayDriver {
        let ctx = sdl2::init().unwrap();
        let canvas = ctx.video().unwrap()
			.window("Ofek's Chip8 Emulator", (DISPLAY_WIDTH * PIXEL_DENSITY) as u32, (DISPLAY_HEIGHT * PIXEL_DENSITY) as u32)
			.position_centered()
			.opengl()
			.build()
			.unwrap()
			.into_canvas()
			.build()
			.unwrap();

        canvas.set_draw_color(pixels::Color::RGB(0, 0, 0));
        canvas.clear();
        canvas.present();

        DisplayDriver {
            canvas
        }
    }
}

impl Display {
    pub fn new() -> Display {
        Display {
            vram: [[0; DISPLAY_WIDTH]; DISPLAY_HEIGHT],
            should_render: false,
            driver: DisplayDriver::new(),
        }
    }
}