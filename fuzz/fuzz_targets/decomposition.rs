#![no_main]
#[macro_use] extern crate libfuzzer_sys;
extern crate galil_seiferas;

fuzz_target!(|data: &[u8]| {
    galil_seiferas::gs_find_by(data, data, |a, b| (a & 0xf) == (b & 0xf));
});
