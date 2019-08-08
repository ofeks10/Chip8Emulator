use sdl2::audio;
use sdl2::Sdl;

struct SquareWave {
    phase_inc: f32,
    phase: f32,
    volume: f32,
}

pub struct Sound {
    sound_device: audio::AudioDevice<SquareWave>,
}

impl audio::AudioCallback for SquareWave {
    type Channel = f32;

    fn callback(&mut self, out: &mut [f32]) {
        // Generate a square wave
        for x in out.iter_mut() {
            *x = self.volume * if self.phase < 0.5 { 1.0 } else { -1.0 };
            self.phase = (self.phase + self.phase_inc) % 1.0;
        }
    }
}


impl Sound {
    pub fn new(ctx: &Sdl) -> Sound {
        let sdl_audio = ctx.audio().unwrap();

        let specifications = audio::AudioSpecDesired {
            freq: Some(44100), // Hertz
            channels: Some(1),
            samples: None,
        };

        let audio_device = sdl_audio.open_playback(None, &specifications, |spec| {
            println!("{:?}", spec);

            SquareWave {
                phase_inc: 240.0 / spec.freq as f32,
                phase: 0.0,
                volume: 0.25,
            }
        }).unwrap();

        Sound {
            sound_device: audio_device,
        }
    }

    pub fn start_beep(&self) {
        self.sound_device.resume();
    }

    pub fn stop_beep(&self) {
        self.sound_device.pause();
    }
}