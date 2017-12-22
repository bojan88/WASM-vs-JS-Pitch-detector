extern crate test;

use ::synth::{Tone, FunctionType};
use self::test::Bencher;

#[test]
fn test_pitch_miss_for_different_frequencies() {
    let pitches = vec![100.0, 200.0, 300.0, 400.0, 500.0, 600.0, 700.0, 800.0, 900.0, 1000.0];
    let threshold = 0.3;

    for pitch in pitches {
        let mut tone = Tone::new(pitch, 1.0);
        tone
            .fade_out(FunctionType::Linear, 0.3, 0.25)
            .add_random_harmonics(10);

        let samples = tone.get_sample_data()[0..1024].to_vec();
        let found_pitch = ::get_pitch(samples, &44_100).unwrap();

        let pitch_diff = pitch - found_pitch;

        if pitch_diff.abs() > threshold {
            println!("Failing on {}Hz, with difference of {:.5}", pitch, pitch_diff.abs());
        }

        assert!(pitch_diff.abs() < threshold);
    }
}

#[bench]
fn bench_nsd_1024(b: &mut Bencher) {
    let pitch = 440.0;
    let duration = 1.0;

    let mut tone = Tone::new(pitch, duration);
    tone
        .fade_out(FunctionType::Linear, 0.3, 0.25)
        .add_random_harmonics(10);

    let samples = tone.get_sample_data();

    b.iter(|| {
        match ::normalized_square_difference(samples[0..1024].to_vec()) {
            _ => ()
        }
    });
}

#[bench]
fn bench_nsd_2048(b: &mut Bencher) {
    let pitch = 440.0;
    let duration = 1.0;

    let mut tone = Tone::new(pitch, duration);
    tone
        .fade_out(FunctionType::Linear, 0.3, 0.25)
        .add_random_harmonics(10);

    let samples = tone.get_sample_data();

    b.iter(|| {
        match ::normalized_square_difference(samples[0..2048].to_vec()) {
            _ => ()
        }
    });
}

#[bench]
fn bench_nsd_4096(b: &mut Bencher) {
    let pitch = 440.0;
    let duration = 1.0;

    let mut tone = Tone::new(pitch, duration);
    tone
        .fade_out(FunctionType::Linear, 0.3, 0.25)
        .add_random_harmonics(10);

    let samples = tone.get_sample_data();

    b.iter(|| {
        match ::normalized_square_difference(samples[0..4096].to_vec()) {
            _ => ()
        }
    });
}

#[bench]
fn bench_nsd_8192(b: &mut Bencher) {
    let pitch = 440.0;
    let duration = 1.0;

    let mut tone = Tone::new(pitch, duration);
    tone
        .fade_out(FunctionType::Linear, 0.3, 0.25)
        .add_random_harmonics(10);

    let samples = tone.get_sample_data();

    b.iter(|| {
        match ::normalized_square_difference(samples[0..8192].to_vec()) {
            _ => ()
        }
    });
}
