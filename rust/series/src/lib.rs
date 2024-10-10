use std::str;

pub fn series(digits: &str, len: usize) -> Vec<String> {
    digits
        .as_bytes()
        .windows(len)
        .map(|s| str::from_utf8(s).unwrap().to_string())
        .collect()
}
