#![no_main]
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: String| {
    if let Ok(config) = nccl::parse_config(&data) {
        let _parsed = config.parse_quoted();
    };
});
