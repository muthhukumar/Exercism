const ASTERISK: u8 = b'*';

enum MineFieldTile {
    Empty,
    Bomb,
    NearBomb(i32),
}

// impl Into<String> for MineFieldTile {
//     fn into(self) -> String {
//         match self {
//             MineFieldTile::NearBomb(count) => count.to_string(),
//             MineFieldTile::Empty => ' '.to_string(),
//             MineFieldTile::Bomb => '*'.to_string(),
//         }
//     }
// }
//
pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();

    let row = minefield.len();

    for i in 0..row {
        let cols = minefield[i].len();

        let col = minefield[i].as_bytes();

        let mut col_result: Vec<MineFieldTile> = col
            .into_iter()
            .map(|v| match *v {
                b'*' => MineFieldTile::Bomb,
                _ => MineFieldTile::Empty,
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
            if j - 1 >= 0 {
                if let Some(&v) = col.get(j - 1) {
                    if v == ASTERISK {
                        init_or_inc(&mut col_result, j);
                    }
                }
            }

            // top
            if i - 1 >= 0 {
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
            if i - 1 >= 0 && j - 1 >= 0 {
                if let Some(field) = minefield.get(i - 1) {
                    if let Some(field_as_byte) = field.as_bytes().get(j - 1) {
                        if *field_as_byte == ASTERISK {
                            init_or_inc(&mut col_result, j);
                        }
                    }
                }
            }

            // top right
            if i - 1 <= row && j + 1 <= cols {
                if let Some(field) = minefield.get(i - 1) {
                    if let Some(field_as_byte) = field.as_bytes().get(j + 1) {
                        if *field_as_byte == ASTERISK {
                            init_or_inc(&mut col_result, j);
                        }
                    }
                }
            }

            // bottom left
            if i + 1 <= row && j - 1 >= 0 {
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
                    MineFieldTile::NearBomb(count) => count.to_string(),
                    MineFieldTile::Empty => ' '.to_string(),
                    MineFieldTile::Bomb => '*'.to_string(),
                })
                .collect(),
        );
    }

    result
}

fn init_or_inc(col: &mut Vec<MineFieldTile>, idx: usize) {
    match col[idx] {
        MineFieldTile::Empty => col[idx] = MineFieldTile::NearBomb(1),
        MineFieldTile::NearBomb(count) => col[idx] = MineFieldTile::NearBomb(count + 1),
        _ => {}
    };
}
