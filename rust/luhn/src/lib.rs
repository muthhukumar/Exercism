/// Check a Luhn checksum.

pub fn is_valid(code: &str) -> bool {
    let not_space = |c: &char| b' ' != *c as u8;
    if code.chars().any(|c| !(b' ' == c as u8) && !c.is_digit(10)) {
        return false;
    };
    let code = {
        let code: Vec<i32> = code
            .chars()
            .filter(not_space)
            .filter(|c| c.is_numeric())
            .map(|c| c.to_digit(10).unwrap_or(0) as i32)
            .collect();
        if code.len() <= 1 {
            return false;
        };
        code
    };
    let result: Vec<i32> = code
        .iter()
        .rev()
        .enumerate()
        .map(|(idx, v)| match idx % 2 {
            0 => *v,
            _ => match *v * 2 {
                d if d > 9 => d - 9,
                d => d,
            },
        })
        .collect();
    result.iter().sum::<i32>() % 10 == 0
}
