pub fn collatz(n: u64) -> Option<u64> {
    let mut count = 0;
    let mut n = n;
    loop {
        n = match n {
            0 => {return None}
            1 => {return Some(count)},
            n if n % 2 == 0 => n / 2,
            n => n * 3 + 1,
        };
        count += 1;
    }
}
