pub fn reply(message: &str) -> &str {
    let message_char: Vec<char> = message.chars().filter(|c| !c.is_whitespace()).collect();

    if message_char.len() == 0 {
        return "Fine. Be that way!";
    }

    let mes = message_char.iter().filter(|c| c.is_alphabetic());

    if mes.clone().all(|c| c.is_uppercase())
        && *message_char.last().unwrap() == '?'
        && mes.count() > 0
    {
        return "Calm down, I know what I'm doing!";
    }

    let mes = message_char.iter().filter(|c| c.is_alphabetic());

    if mes.clone().all(|c| c.is_uppercase()) && mes.count() > 0 {
        return "Whoa, chill out!";
    }

    if *message_char.last().unwrap() == '?' {
        return "Sure.";
    }

    return "Whatever.";
}
