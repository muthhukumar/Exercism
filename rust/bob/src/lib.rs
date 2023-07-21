pub fn reply(message: &str) -> &str {
    let message: Vec<char> = message.chars().filter(|c| !c.is_whitespace()).collect();

    println!("{:?}", message);

    if message.len() == 0 {
        return "Fine. Be that way!";
    }

    if message
        .iter()
        .filter(|c| c.is_alphabetic())
        .all(|c| c.is_uppercase())
        && !message.iter().any(|c| *c == '?')
        && message.iter().filter(|c| c.is_alphabetic()).count() > 0
    {
        return "Whoa, chill out!";
    }

    if message
        .iter()
        .filter(|c| c.is_alphabetic())
        .all(|c| c.is_uppercase())
        && *message.last().unwrap() == '?'
        && message.iter().filter(|c| c.is_alphabetic()).count() > 0
    {
        return "Calm down, I know what I'm doing!";
    }

    if *message.last().unwrap() == '?' {
        return "Sure.";
    }

    return "Whatever.";
}
