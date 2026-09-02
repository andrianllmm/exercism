pub fn abbreviate(phrase: &str) -> String {
    let mut acronym = String::new();
    let mut new_word = true;

    for c in phrase.chars() {
        // Hyphens and whitespace separate words
        if c == '-' || c.is_whitespace() {
            new_word = true;
        }
        else if c.is_alphabetic() {
            if new_word {
                acronym.push(c.to_ascii_uppercase());
                new_word = false;
            }
        }
    }

    acronym
}
