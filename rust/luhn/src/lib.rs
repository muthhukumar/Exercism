/// Check a Luhn checksum.

pub fn is_valid(code: &str) -> bool {
    let not_space = |c: &char| b' ' != *c as u8;
    let not_space_or_digit = |c: char| !(b' ' == c as u8) && !c.is_digit(10);

    if code.chars().any(not_space_or_digit) {
        return false;
    };

    let code: Vec<i32> = code
        .chars()
        .filter(not_space)
        .map(|c| c.to_digit(10).unwrap_or(0) as i32)
        .rev()
        .enumerate()
        .map(|(idx, v)| match idx % 2 {
            0 => v,
            _ => match v * 2 {
                d if d > 9 => d - 9,
                d => d,
            },
        })
        .collect();

    code.len() > 1 && code.iter().sum::<i32>() % 10 == 0
}
