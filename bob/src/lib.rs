pub fn reply(message: &str) -> &str {
    let trimmed = message.trim();

    match trimmed {
        "" => "Fine. Be that way!",
        m if is_shout(m) && m.ends_with('?') => "Calm down, I know what I'm doing!",
        m if is_shout(m) => "Whoa, chill out!",
        m if m.ends_with('?') => "Sure.",
        _ => "Whatever.",
    }
}

fn is_shout(message: &str) -> bool {
    let has_letters = message.chars().any(|c| c.is_ascii_alphabetic());
    let all_uppercase = message
        .chars()
        .filter(|c| c.is_ascii_alphabetic())
        .all(|c| c.is_ascii_uppercase());
    has_letters && all_uppercase
}
