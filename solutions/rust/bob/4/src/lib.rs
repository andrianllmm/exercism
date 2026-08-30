pub fn reply(message: &str) -> &str {
    if message.trim().is_empty() {
        return "Fine. Be that way!";
    } else if is_question(message) && is_yelling(message) {
        return "Calm down, I know what I'm doing!"
    } else if is_question(message) {
        return "Sure.";
    } else if is_yelling(message) {
        return "Whoa, chill out!";
    } else {
        return "Whatever.";
    }
}

fn is_question(message: &str) -> bool {
    message.trim().chars().last().unwrap() == '?'
}

fn is_yelling(message: &str) -> bool {
    has_letters(message) && message.chars().all(|c| !c.is_alphabetic() || c.is_uppercase())
}

fn has_letters(message: &str) -> bool {
    message.chars().any(|c| c.is_alphabetic())
}
