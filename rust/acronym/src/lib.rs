pub fn abbreviate(phrase: &str) -> String {
    phrase
        .split(|c: char| c == '-' || c == '_' || c.is_whitespace())
        .flat_map(|w| {
            w.chars().take(1).chain(
                w.chars()
                    .skip_while(|c| c.is_ascii_uppercase())
                    .filter(|c| c.is_ascii_uppercase()),
            )
        })
        .collect::<String>()
        .to_uppercase()
}
