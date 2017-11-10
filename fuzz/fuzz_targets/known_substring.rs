#![no_main]
#[macro_use] extern crate libfuzzer_sys;
extern crate galil_seiferas;

use std::cmp::{min, max};

fuzz_target!(|data: &[u8]| {
    if data.len() > 4 {
        let len = data.len() - 4;
        // use first 4 bytes as delimiters
        let (a, b) = (data[0] as usize, data[1] as usize);
        let first = ((a << 8) | b) % len;
        let (a, b) = (data[2] as usize, data[3] as usize);
        let second = ((a << 8) | b) % len;
        let (first, second) = (min(first, second), max(first, second));
        let (_, data) = data.split_at(4);
        let needle = &data[first..second];
        let find_result = galil_seiferas::gs_find(data, needle);
        if let Some(i) = find_result {
            assert!(i <= first, "i={} must be leq first={}", i, first);
        } else {
            panic!("Expected match at first={}", first);
        }
    }
});
