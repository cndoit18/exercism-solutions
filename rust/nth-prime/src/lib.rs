pub fn nth(n: u32) -> u32 {
    let mut primes = Vec::new();
    let mut current = 2;
    while primes.len() <= n as usize {
        if primes.iter().all(|&p| current % p != 0) {
            primes.push(current);
        }
        current += 1;
    }
    primes[n as usize]
}
