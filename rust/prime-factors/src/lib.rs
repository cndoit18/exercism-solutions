pub fn factors(n: u64) -> Vec<u64> {
    let mut f = Vec::new();
    let mut n = n;
    while let Some(value) = (2..=n).find(|&i| n % i == 0) {
        n /= value;
        f.push(value);
        continue;
    }
    f
}
