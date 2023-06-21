const ASTERISK: u8 = b'*';

enum TileState {
    Empty,
    Bomb,
    Score(i32),
}

pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();

    let row = minefield.len();

    for i in 0..row {
        let cols = minefield[i].len();

        let col = minefield[i].as_bytes();

        let mut col_result: Vec<TileState> = col
            .into_iter()
            .map(|v| match *v {
                b'*' => TileState::Bomb,
                _ => TileState::Empty,
            })
            .collect();

        for j in 0..cols {
            if col[j] != b' ' {
                continue;
            }

            // right
            if j + 1 <= cols {
                if let Some(&v) = col.get(j + 1) {
                    if v == ASTERISK {
                        init_or_inc(&mut col_result, j);
                    }
                }
            }

            // left
            if !j.overflowing_sub(1).1 {
                if let Some(&v) = col.get(j - 1) {
                    if v == ASTERISK {
                        init_or_inc(&mut col_result, j);
                    }
                }
            }

            // top
            if !i.overflowing_sub(1).1 {
                if let Some(field) = minefield.get(i - 1) {
                    if let Some(field_as_byte) = field.as_bytes().get(j) {
                        if *field_as_byte == ASTERISK {
                            init_or_inc(&mut col_result, j);
                        }
                    }
                }
            }

            // bottom
            if i + 1 <= row {
                if let Some(field) = minefield.get(i + 1) {
                    if let Some(field_as_byte) = field.as_bytes().get(j) {
                        if *field_as_byte == ASTERISK {
                            init_or_inc(&mut col_result, j);
                        }
                    }
                }
            }

            // top left
            if !i.overflowing_sub(1).1 && !j.overflowing_sub(1).1 {
                if let Some(field) = minefield.get(i - 1) {
                    if let Some(field_as_byte) = field.as_bytes().get(j - 1) {
                        if *field_as_byte == ASTERISK {
                            init_or_inc(&mut col_result, j);
                        }
                    }
                }
            }

            // top right
            if !i.overflowing_sub(1).1 && j + 1 <= cols {
                if let Some(field) = minefield.get(i - 1) {
                    if let Some(field_as_byte) = field.as_bytes().get(j + 1) {
                        if *field_as_byte == ASTERISK {
                            init_or_inc(&mut col_result, j);
                        }
                    }
                }
            }

            // bottom left
            if i + 1 <= row && !j.overflowing_sub(1).1 {
                if let Some(field) = minefield.get(i + 1) {
                    if let Some(field_as_byte) = field.as_bytes().get(j - 1) {
                        if *field_as_byte == ASTERISK {
                            init_or_inc(&mut col_result, j);
                        }
                    }
                }
            }

            // bottom right
            if i + 1 <= row && j + 1 <= cols {
                if let Some(field) = minefield.get(i + 1) {
                    if let Some(field_as_byte) = field.as_bytes().get(j + 1) {
                        if *field_as_byte == ASTERISK {
                            init_or_inc(&mut col_result, j);
                        }
                    }
                }
            }
        }

        result.push(
            col_result
                .iter()
                .map(|v| match v {
                    TileState::Score(count) => count.to_string(),
                    TileState::Empty => ' '.to_string(),
                    TileState::Bomb => '*'.to_string(),
                })
                .collect(),
        );
    }

    result
}

fn init_or_inc(col: &mut Vec<TileState>, idx: usize) {
    match col[idx] {
        TileState::Empty => col[idx] = TileState::Score(1),
        TileState::Score(count) => col[idx] = TileState::Score(count + 1),
        _ => {}
    };
}
