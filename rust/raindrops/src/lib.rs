pub fn raindrops(n: u32) -> String {
    let mut result = String::new();

    if n % 3 == 0 {
        result = format!("{result}Pling");
    }

    if n % 5 == 0 {
        result = format!("{result}Plang");
    }

    if n % 7 == 0 {
        result = format!("{result}Plong");
    }

    if n % 3 != 0 && n % 5 != 0 && n % 7 != 0 {
        result = n.to_string();
    }

    result
}
