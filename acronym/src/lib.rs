pub fn abbreviate(phrase: &str) -> String {
    // Replace hyphens with spaces and handle camelCase
    let mut normalized = String::new();
    let mut prev_is_lowercase = false;

    for c in phrase.chars() {
        match c {
            '-' | '_' => normalized.push(' '), // Replace hyphens and underscores with spaces
            c if c.is_uppercase() && prev_is_lowercase => {
                // Insert a space before an uppercase letter that follows a lowercase (camelCase)
                normalized.push(' ');
                normalized.push(c);
                prev_is_lowercase = false;
            }
            c if c.is_alphabetic() => {
                normalized.push(c);
                prev_is_lowercase = c.is_lowercase();
            }
            c if c.is_whitespace() => {
                normalized.push(' ');
                prev_is_lowercase = false;
            }
            // Ignore other characters (punctuation)
            _ => {}
        }
    }

    // Split into words, take the first letter of each word and convert to uppercase
    normalized
        .split_whitespace() // Split on spaces (including those we added)
        .filter(|word| !word.is_empty()) // Ignore empty words
        .map(|word| {
            word.chars()
                .next() // Get the first character
                .unwrap_or(' ') // In case of empty word (shouldn't happen with the filter)
                .to_uppercase() // Convert to uppercase
                .to_string()
        })
        .collect() // Combine the first letters into a string
}
