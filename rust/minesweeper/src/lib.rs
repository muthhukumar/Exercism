pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();

    let asterisk = b'*';
    let space = b' ';

    let row = minefield.len();

    for i in 0..row {
        let cols = minefield[i].len();

        let col = minefield[i].as_bytes();

        let mut col_result: Vec<i32> = col
            .into_iter()
            .map(|v| {
                if *v == asterisk {
                    return asterisk as i32;
                } else if *v == space {
                    return space as i32;
                } else {
                    return 0;
                }
            })
            .collect();

        for j in 0..cols {
            if col[j] != b' ' {
                continue;
            }

            // right
            if j + 1 <= cols {
                if let Some(&v) = col.get(j + 1) {
                    if v == asterisk {
                        init_or_inc(&mut col_result, j);
                    }
                }
            }

            // left
            if j - 1 >= 0 {
                if let Some(&v) = col.get(j - 1) {
                    if v == asterisk {
                        init_or_inc(&mut col_result, j);
                    }
                }
            }

            // // top
            // if i - 1 >= 0 && minefield[i - 1].as_bytes()[j] == asterisk {
            //     init_or_inc(&mut col_result, j);
            // }
            //
            // // bottom
            // if i + 1 <= row && minefield[i + 1].as_bytes()[j] == asterisk {
            //     init_or_inc(&mut col_result, j);
            // }
            //
            // // top left
            // if i - 1 >= 0 && j - 1 >= 0 && minefield[i - 1].as_bytes()[j - 1] == asterisk {
            //     init_or_inc(&mut col_result, j);
            // }
            //
            // // top right
            // if i - 1 <= row && j + 1 <= cols && minefield[i - 1].as_bytes()[j + 1] == asterisk {
            //     init_or_inc(&mut col_result, j);
            // }
            //
            // // bottom left
            // if i + 1 <= row && j - 1 >= 0 && minefield[i + 1].as_bytes()[j - 1] == asterisk {
            //     init_or_inc(&mut col_result, j);
            // }
            //
            // // bottom right
            // if i + 1 <= row && j + 1 <= cols && minefield[i + 1].as_bytes()[j + 1] == asterisk {
            //     init_or_inc(&mut col_result, j);
            // }
        }

        println!("result: {:?}", col_result);

        result.push(
            col_result
                .into_iter()
                .map(|v| char::from(v))
                .map(|v| v.to_string())
                .collect::<String>(),
        );
    }

    result
}

fn init_or_inc(col: &mut Vec<i32>, idx: usize) {
    println!("idx: {}, col: {}", idx, col[idx]);
    if col[idx] != 32 {
        col[idx] = col[idx] + 1;
    } else {
        col[idx] = 0;
    }
}
