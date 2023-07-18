pub fn reply(message: &str) -> &str {
    match message {
        x if x.contains('?') => "Sure.",
        _ => "Whatever",
    }
}
