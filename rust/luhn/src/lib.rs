/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    code.chars()
        .rev()
        .filter(|c| !c.is_whitespace())
        .try_fold((0, 0), |(sum, count), val| {
            val.to_digit(10)
                .map(|digit| if count % 2 == 0 { digit } else { digit * 2 })
                .map(|digit| if digit > 9 { digit - 9 } else { digit })
                .map(|digit| (sum + digit, count + 1))
        })
        .map_or(false, |(sum, count)| sum % 10 == 0 && count > 1)
}
