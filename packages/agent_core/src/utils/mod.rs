pub mod name_lookup;
pub mod error_helper;
pub mod shuffle;

pub fn now_milli() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_millis() as u64
}

pub fn now_sec() -> u32 {
    (now_milli() / 1_000) as u32
}