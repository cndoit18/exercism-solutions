use std::collections::HashSet;

pub fn check(candidate: &str) -> bool {
    let mut seen = HashSet::new();
    candidate
        .to_lowercase()
        .chars()
        .filter(|c| !c.is_whitespace() && c != &'-')
        .all(|c| seen.insert(c))
}
