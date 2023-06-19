pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let y_len = minefield.len();

    let mut result: Vec<String> = Vec::new();

    for (row, line) in minefield.into_iter().enumerate() {
        for (col, tile) in line.chars().enumerate() {
            let mut count = 0;
            match (row, col, tile) {
                (0, 0, tile) => {
                    if tile == '*' {
                        continue;
                    }
                }
                _ => {}
            }
        }
    }

    result
}
