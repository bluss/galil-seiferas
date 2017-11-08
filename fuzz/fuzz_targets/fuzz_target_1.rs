#![no_main]
#[macro_use] extern crate libfuzzer_sys;
extern crate galil_seiferas;

fuzz_target!(|data: &[u8]| {
    if data.len() > 1 {
        // use first byte as split point
        let split_len = (data[0] as usize).min(data.len() - 1);
        let data = &data[1..];
        let (needle, hay) = data.split_at(split_len);
        assert_eq!(galil_seiferas::util::brute_force_fast(hay, needle), galil_seiferas::gs_find(hay, needle));
    }
});
