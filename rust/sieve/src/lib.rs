pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
    let mut result = Vec::new();
    (2..=upper_bound).for_each(|n| {
        if result.iter().all(|i| n % i != 0) {
            result.push(n);
        }
    });
    result
}
