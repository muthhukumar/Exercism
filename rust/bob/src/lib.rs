pub fn reply(message: &str) -> &str {
    let message: Vec<char> = message.chars().collect();

    println!("Input: {:?}", message);

    if message.iter().all(|c| c.is_whitespace()) {
        return "Fine. Be that way!";
    } else if message
        .iter()
        .all(|c| c.is_numeric() || c.is_uppercase() || !c.is_alphanumeric() && *c != '?')
    {
        return "Whoa, chill out!";
    } else if message.iter().all(|c| c.is_uppercase() || *c == '?') {
        return "Calm down, I know what I'm doing!";
    }

    if let Some(last) = message.iter().filter(|c| c.is_whitespace()).last() {
        if *last == '?' {
            println!("list: {}", *last);
            return "Sure.";
        }
    }

    return "Whatever.";
}
