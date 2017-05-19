#![no_main]

#[macro_use]
extern crate libfuzzer_sys;

fuzz_target!(|data: &[u8]| {
    if data == b"banana!" {
        panic!("success!");
    }
});
