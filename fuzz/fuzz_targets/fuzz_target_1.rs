#![no_main]
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: String| {
    let _config = nccl::parse_config(&data);
});
