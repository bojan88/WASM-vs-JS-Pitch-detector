#![feature(test)]
#[cfg(test)]
mod tests;
#[cfg(test)]
pub mod synth;

use std::f64;

const LOWER_PITCH_CUTOFF: f64 = 20.0;
const SMALL_CUTOFF: f64 = 0.5;
const CUTOFF: f64 = 0.93;

pub fn get_pitch(buffer: Vec<f64>, sample_rate: &usize) -> Option<f64> {
    let nsdf = normalized_square_difference(buffer);
    let max_positions = peak_picking(&nsdf);
    let mut estimates: Vec<(f64, f64)> = Vec::new();

    let mut highest_amplitude = f64::MIN;

    for i in max_positions {
        highest_amplitude = f64::max(highest_amplitude, nsdf[i]);
        if nsdf[i] > SMALL_CUTOFF {
            let est = parabolic_interpolation(&nsdf, i);
            estimates.push(est);
            highest_amplitude = f64::max(highest_amplitude, est.1);
        }
    }

    if estimates.is_empty() {
        return None;
    }

    let actual_cutoff = CUTOFF * highest_amplitude;
    let mut period: f64 = 0.0;

    for e in estimates {
        if e.1 >= actual_cutoff {
            period = e.0;
            break;
        }
    }

    let pitch_est: f64 = *sample_rate as f64 / period;

    if pitch_est > LOWER_PITCH_CUTOFF {
        Some(pitch_est)
    } else {
        None
    }
}

fn peak_picking(nsdf: &Vec<f64>) -> Vec<usize> {
    let mut max_positions: Vec<usize> = Vec::new();
    let mut pos = 0;
    let mut cur_max_pos = 0;
    let len = nsdf.len();

    while pos < (len - 1) / 3 && nsdf[pos] > 0.0 {
        pos += 1;
    }
    while pos < len - 1 && nsdf[pos] <= 0.0 {
        pos += 1;
    }

    if pos == 0 {
        pos = 1;
    }

    while pos < len - 1 {
        if nsdf[pos] > nsdf[pos - 1] && nsdf[pos] >= nsdf[pos + 1] {
            if cur_max_pos == 0 {
                cur_max_pos = pos;
            } else if nsdf[pos] > nsdf[cur_max_pos] {
                cur_max_pos = pos;
            }
        }
        pos += 1;
        if pos < len - 1 && nsdf[pos] <= 0.0 {
            if cur_max_pos > 0 {
                max_positions.push(cur_max_pos);
                cur_max_pos = 0;
            }
            while pos < len - 1 && nsdf[pos] <= 0.0 {
                pos += 1;
            }
        }
    }
    if cur_max_pos > 0 {
        max_positions.push(cur_max_pos);
    }

    max_positions
}

fn normalized_square_difference(buffer: Vec<f64>) -> Vec<f64> {
    let len = buffer.len();
    let mut nsdf: Vec<f64> = vec![0.0; len];

    for tau in 0..len {
        let mut acf: f64 = 0.0;
        let mut divisor_m: f64 = 0.0;

        for i in 0..(len - tau) {
            acf += buffer[i] * buffer[i + tau];
            let el1 = buffer[i];
            let p1 = el1.powi(2);
            let el2 = buffer[i + tau];
            let p2 = el2.powi(2);
            divisor_m += p1 + p2;
        }

        nsdf[tau] = 2.0 * acf / divisor_m;
    }

    nsdf
}

fn parabolic_interpolation(nsdf: &Vec<f64>, tau: usize) -> (f64, f64) {
    let nsdfa = nsdf[tau - 1];
    let nsdfb = nsdf[tau];
    let nsdfc = nsdf[tau + 1];
    let b_val = tau as f64;
    let bottom = nsdfc + nsdfa - 2.0 * nsdfb;

    if bottom == 0.0 {
        (b_val, nsdfb)
    } else {
        let delta = nsdfa - nsdfc;
        (
            b_val + delta / (2.0 * bottom),
            nsdfb - delta * delta / (8.0 * bottom),
        )
    }
}
