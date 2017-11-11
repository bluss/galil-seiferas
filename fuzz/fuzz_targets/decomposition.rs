#![no_main]
#[macro_use] extern crate libfuzzer_sys;
extern crate galil_seiferas;

fuzz_target!(|data: &[u8]| {
    if let Some((_, tail)) = data.split_first() {
        galil_seiferas::gs_find(data, tail);
    }
});
