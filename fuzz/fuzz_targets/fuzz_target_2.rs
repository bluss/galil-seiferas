#![no_main]
#[macro_use] extern crate libfuzzer_sys;
extern crate galil_seiferas;

fuzz_target!(|data: &[u8]| {
    let hay = data;
    let needle = b"aaabaaabaaabaaabbbbb";
    assert_eq!(galil_seiferas::util::brute_force_fast(hay, needle), galil_seiferas::gs_find(hay, needle));
});
