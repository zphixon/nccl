#![no_main]
use libfuzzer_sys::fuzz_target;

fuzz_target!(|config: nccl::Config| {
    let _parsed = config.parse_quoted();
});
