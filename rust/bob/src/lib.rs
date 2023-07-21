pub fn reply(message: &str) -> &str {
    let char: Vec<char> = message
        .chars()
        .filter(|c| !c.is_whitespace() || c.is_alphabetic() || *c == '?')
        .collect();

    if char.len() == 0 {
        return "Fine. Be that way!";
    }

    let last_char = *char.last().unwrap();

    let alpha_char: Vec<&char> = char.iter().filter(|c| c.is_alphabetic()).collect();

    if alpha_char.iter().all(|c| c.is_uppercase()) {
        if alpha_char.len() > 0 {
            if last_char == '?' {
                return "Calm down, I know what I'm doing!";
            } else {
                return "Whoa, chill out!";
            }
        }
    }

    if last_char == '?' {
        return "Sure.";
    }

    return "Whatever.";
}
