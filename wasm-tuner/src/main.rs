extern crate wasm_tuner;

fn main() {}

use std::cmp::Ordering;

#[no_mangle]
pub fn get_pitch(buffer_ptr: *mut usize, buffer_len: usize, sample_rate: usize) -> f64 {
    let buffer: Vec<f64>;

    if sample_rate.cmp(&0) == Ordering::Equal {
        panic!("Sample rate can't be 0");
    }

    unsafe {
        buffer = Vec::from_raw_parts(buffer_ptr as *mut f32, buffer_len, buffer_len)
            .into_iter()
            .map(|e| e as f64)
            .collect();
    }

    match wasm_tuner::get_pitch(buffer, &sample_rate) {
        Some(val) => val,
        None => -1.0
    }
}
