extern crate synthrs;
extern crate rand;

use self::synthrs::synthesizer::{ make_samples };
use self::synthrs::wave::SineWave;

use synth::rand::Rng;

pub enum FunctionType {
    Linear
}

pub struct Tone {
    data: Vec<f64>,
    pitch: f64,
    duration: f64,
    sample_rate: u32
}

impl Tone {
    pub fn new(pitch: f64, duration: f64) -> Tone {
        Tone {
            data: make_samples(duration, 44_100, SineWave(pitch)),
            pitch,
            duration,
            sample_rate: 44_100
        }
    }

    pub fn get_sample_data(self) -> Vec<f64> {
        self.data
    }

    pub fn change_volume(&mut self, volume: f64) -> &mut Self {
        let factor = 1.0 + volume;

        for s in self.data.iter_mut() {
            *s *= factor;
        }

        self
    }

    pub fn combine(&mut self, another: Self) -> &mut Self {
        // using another block to end borrow before Self is returned
        {
            let zipped = self.data.iter_mut().zip(another.data.iter());

            for (s, another_s) in zipped {
                *s += another_s;
            }
        }

        self
    }

    pub fn fade_out(&mut self, fade_type: FunctionType, fade_start: f64, fade_duration: f64) -> &mut Self {
        // using another block to end borrow before Self is returned
        {
            let iter = self.data.iter_mut();
            let skip: usize = (self.sample_rate as f64 * fade_start) as usize;

            let mut fade_mul: f64 = 1.0;
            let fade_step: f64 = 1.0 / (fade_duration * self.sample_rate as f64);

            let iter = iter.skip(skip);
            for (_, s) in iter.enumerate() {
                *s *= fade_mul;

                let new_fade_mul = match fade_type {
                    FunctionType::Linear => fade_mul - fade_step
                };

                if new_fade_mul > 0.0 {
                    fade_mul = new_fade_mul;
                } else {
                    fade_mul = 0.0;
                }
            }
        }

        self
    }

    // add harmonics with random duration, fading, volume
    pub fn add_random_harmonics(&mut self, num_of_harmonics: u32) -> &mut Self {
        for i in 2..num_of_harmonics {
            let mut harmonic = Self::new(self.pitch * i as f64, self.duration);

            let volume = rand::thread_rng().gen_range(0.6, 0.95);
            let fade_time = rand::thread_rng().gen_range(0.2, self.duration - 0.2);

            harmonic
                .change_volume(-volume)
                .fade_out(FunctionType::Linear, fade_time, fade_time);

            self.combine(harmonic);
        }

        self
    }
}
