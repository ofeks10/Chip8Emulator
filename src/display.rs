extern crate sdl2;
use sdl2::pixels;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

pub const DISPLAY_WIDTH: usize = 64;
pub const DISPLAY_HEIGHT: usize = 32;
const PIXEL_DENSITY: usize = 15;

pub struct Display {
    pub vram: [[u8; DISPLAY_WIDTH]; DISPLAY_HEIGHT],
    pub should_render: bool,
    driver: DisplayDriver,
}

struct DisplayDriver {
    canvas: Canvas<Window>,
}

impl DisplayDriver {
    fn new() -> DisplayDriver {
        let ctx = sdl2::init().unwrap();
        let mut canvas = ctx.video().unwrap()
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

    fn render_display(&mut self, pixels: &[[u8; DISPLAY_WIDTH]; DISPLAY_HEIGHT]) {
        for (y, row) in pixels.iter().enumerate() {
            for (x, &col) in row.iter().enumerate() {
                let x = x * PIXEL_DENSITY;
                let y = y * PIXEL_DENSITY;

                let draw_color = if col == 1 {
                    pixels::Color::RGB(255, 255, 255)
                } else {
                    pixels::Color::RGB(0, 0, 0)
                };

                self.canvas.set_draw_color(draw_color);
                self.canvas.fill_rect(Rect::new(x as i32, y as i32, PIXEL_DENSITY as u32, PIXEL_DENSITY as u32))
                .expect("Could not draw to screen");
            }
        }

        self.canvas.present();
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

    pub fn clear(&mut self) {
        self.vram = [[0; DISPLAY_WIDTH]; DISPLAY_HEIGHT];
        self.driver.render_display(&self.vram);
    }

    pub fn render(&mut self) {
        self.driver.render_display(&self.vram);
        self.should_render = false;
    }
}