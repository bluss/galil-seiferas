#![no_main]
#[macro_use] extern crate libfuzzer_sys;
extern crate galil_seiferas;

fuzz_target!(|data: &[u8]| {
    if let Ok(data) = std::str::from_utf8(data) {
        if data.len() > 1 {
            // use first byte as split point
            let split_len = (data[0..].chars().next().unwrap() as usize).min(data.len() - 1);
            let data = &data[1..];
            let (needle, hay) = data.split_at(split_len);
            assert_eq!(str::find(hay, needle), galil_seiferas::gs_find(hay.as_bytes(), needle.as_bytes()));
        }
    }
});
