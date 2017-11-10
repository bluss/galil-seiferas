#![no_main]
#[macro_use] extern crate libfuzzer_sys;
extern crate galil_seiferas;

fuzz_target!(|data: &[u8]| {
    // data is ascii only
    if data.len() > 2 {
        // use first 2 bytes as split point
        let (a, b) = (data[0] as usize, data[1] as usize);
        let (_, data) = data.split_at(2);
        let needle_split = ((a << 7) | b).min(data.len());
        if let Ok(data) = std::str::from_utf8(data) {
            let (needle, hay) = data.split_at(needle_split);
            assert_eq!(str::find(hay, needle), galil_seiferas::gs_find(hay.as_bytes(), needle.as_bytes()));
        }
    }
});
