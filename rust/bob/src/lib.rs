pub fn reply(message: &str) -> &str {
    match message.trim() {
        m if m.to_uppercase() == m && m.chars().any(|c| c.is_uppercase()) && m.ends_with('?')=> "Calm down, I know what I'm doing!",
        m if m.to_uppercase() == m && m.chars().any(|c| c.is_uppercase()) => "Whoa, chill out!",
        m if m.ends_with('?') => "Sure.",
        "" => "Fine. Be that way!",
        _ => "Whatever.",
    }
}
