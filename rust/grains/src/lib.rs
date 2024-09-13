pub fn square(s: u32) -> u64 {
    u64::pow(2, s - 1)
}

pub fn total() -> u64 {
    (1..=64).map(square).sum()
}
