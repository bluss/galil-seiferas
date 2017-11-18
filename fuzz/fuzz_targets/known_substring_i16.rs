#![no_main]
#[macro_use] extern crate libfuzzer_sys;
extern crate galil_seiferas;

use std::cmp::{min, max};
use std::slice::from_raw_parts;

type PodType = i16;

fuzz_target!(|data: &[u8]| {
    let mut data = data;
    // align
    if data.as_ptr() as usize & 1 != 0 {
        data = &data[1..];
    }
    let data = unsafe { from_raw_parts(data.as_ptr() as *const PodType, data.len() / 2) };
    if data.len() > 2 {
        let len = data.len() - 2;
        // use first 4 bytes as delimiters
        let first = data[0] as usize % len;
        let second = data[1] as usize % len;
        let (first, second) = (min(first, second), max(first, second));
        let (_, data) = data.split_at(2);
        let needle = &data[first..second];
        let find_result = galil_seiferas::gs_find(data, needle);
        if let Some(i) = find_result {
            assert!(i <= first, "i={} must be leq first={}", i, first);
        } else {
            panic!("Expected match at first={}\ndata={:?}, needle={:?}", first, data, needle);
        }
    }
});
