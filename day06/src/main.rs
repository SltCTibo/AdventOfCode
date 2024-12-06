enum Direction {
    Up,
    Down,
    Left,
    Right
}

fn test_map(rows: &mut Vec<String>, initial_position: &(i32, i32)) -> bool {

    let mut row: i32 = initial_position.0;
    let mut col: i32 = initial_position.1;

    let mut direction = Direction::Up;

    let mut loop_state = 0;

    rows[row as usize].replace_range(col as usize..col as usize +1, "X");

    while row >= 0 && row < rows.len() as i32 && col >= 0 && col < rows[0].len() as i32 {
        match direction {
            Direction::Up => {
                if row - 1 < 0 {
                    break;
                }
                if let Some(ch) = rows[row as usize - 1].chars().nth(col as usize) {
                    match ch {
                        '#' | 'O' => {
                            direction = Direction::Right;
                            if loop_state == 4 {
                                return true
                            }
                            loop_state += 1;
                        }
                        'X' => {
                            row -= 1;
                        }
                        '.' | '^' => {
                            row -= 1;
                            rows[row as usize].replace_range(col as usize..col as usize +1, "X");
                            loop_state = 0;
                        }
                        _ => ()
                    }
                }
            }
            Direction::Down => {
                if row + 1 >= rows.len() as i32 {
                    break;
                }
                if let Some(ch) = rows[row as usize + 1].chars().nth(col as usize) {
                    match ch {
                        '#' | 'O' => {
                            direction = Direction::Left;
                            if loop_state == 4 {
                                return true
                            }

                            loop_state += 1;
                        }
                        'X' => {
                            row += 1;
                        }
                        '.' | '^' => {
                            row += 1;
                            rows[row as usize].replace_range(col as usize..col as usize +1, "X");
                            loop_state = 0;
                        }
                        _ => ()
                    }
                }
            }
            Direction::Left => {
                if col - 1 < 0 {
                    break;
                }
                if let Some(ch) = rows[row as usize].chars().nth(col as usize - 1) {
                    match ch {
                        '#' | 'O' => {
                            direction = Direction::Up;
                            if loop_state == 4 {
                                return true
                            }
                            loop_state += 1;
                        }
                        'X' => {
                            col -= 1;
                        }
                        '.' | '^' => {
                            col -= 1;
                            rows[row as usize].replace_range(col as usize..col as usize +1, "X");
                            loop_state = 0;
                        }
                        _ => ()
                    }
                }
            }
            Direction::Right => {
                if col + 1 >= rows[0].len() as i32 {
                    break;
                }
                if let Some(ch) = rows[row as usize].chars().nth(col as usize + 1) {
                    match ch {
                        '#' | 'O' => {
                            direction = Direction::Down;
                            if loop_state == 4 {
                                return true
                            }
                            loop_state += 1;
                        }
                        'X' => {
                            col += 1;
                        }
                        '.' | '^' => {
                            col += 1;
                            rows[row as usize].replace_range(col as usize..col as usize +1, "X");
                            loop_state = 0;
                        }
                        _ => ()
                    }
                }
            }
        }
    }

    false
}

fn main() {
    let contents = std::fs::read_to_string("input.txt").expect("Should have been reading the file");

    let mut rows = Vec::<String>::new();

    let mut initial_postion: (i32, i32) = (0, 0);

    for line in contents.lines() {
        if line.contains("^") {
            if let Some(position) = line.char_indices().find(|&(_, c)| c == '^').map(|(i, _)| i) {
                initial_postion = (rows.len() as i32, position as i32);
            }
        }
        rows.push(line.to_string());
    }

    let mut loop_created = 0;

    for row in 0..rows.len() {
        for col in 0..rows[row].len() {
            if let Some(ch) = rows[row].chars().nth(col) {
                if ch == '#' || ch == '^' {
                    continue;
                }
            }
            rows[row as usize].replace_range(col as usize..col as usize +1, "O");
            if test_map(rows.clone().as_mut(), &mut initial_postion) {
                loop_created += 1;
            }
            rows[row as usize].replace_range(col as usize..col as usize +1, ".");
        }
    }

    for r in &rows {
        println!("{:?}", r);
    }

    println!("Loops created = {loop_created}");

}
