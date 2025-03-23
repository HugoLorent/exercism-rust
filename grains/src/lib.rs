pub fn square(s: u32) -> u64 {
    if !(1..=64).contains(&s) {
        panic!("Square must be between 1 and 64, got {}", s)
    }
    2_u64.pow(s - 1)
}

pub fn total() -> u64 {
    // The sum of powers of 2 from 0 to 63 (2^0 + 2^1 + ... + 2^63)
    // equals to 2^64 - 1, which is exactly u64::MAX
    u64::MAX
}
