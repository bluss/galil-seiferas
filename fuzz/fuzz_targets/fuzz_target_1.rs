#![no_main]
#[macro_use] extern crate libfuzzer_sys;
extern crate galil_seiferas;

fuzz_target!(|data: &[u8]| {
    if data.len() > 2 {
        // use first 2 bytes as split point
        let (a, b) = (data[0] as usize, data[1] as usize);
        let (_, data) = data.split_at(2);
        let needle_split = ((a << 8) | b).min(data.len());
        let (needle, hay) = data.split_at(needle_split);
        assert_eq!(galil_seiferas::brute_force_search(hay, needle), galil_seiferas::gs_find(hay, needle));
    }
});
