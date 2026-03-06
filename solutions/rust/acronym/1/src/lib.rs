pub fn abbreviate(phrase: &str) -> String {
    if phrase.is_empty() {
        return String::new();
    }

    let mut abbreviation = String::new();
    let mut previous_char: Option<char> = None;
    for c in phrase.chars() {
        if previous_char.is_none_or(|prev_c| {
            ((prev_c.is_whitespace() || prev_c == '-' || prev_c == '_') && c.is_ascii_alphabetic())
                || (prev_c.is_ascii_lowercase() && c.is_ascii_uppercase())
        }) {
            abbreviation.push(c.to_ascii_uppercase());
        }
        previous_char = Some(c);
    }
    abbreviation
}
