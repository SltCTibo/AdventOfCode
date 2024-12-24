fn move_player(map: &mut Vec<Vec<char>>, positions: Vec<(i32, i32)>, speed: (i32, i32)) {
    for pos in &positions {
        let next_pos = (pos.0 + speed.0, pos.1 + speed.1);
        let next_tile = map[next_pos.0 as usize][next_pos.1 as usize];

        match next_tile {
            '[' => {
                if speed.1 != 0 {
                    move_player(map, vec![next_pos], speed);
                } else {
                    move_player(map, vec![next_pos, (next_pos.0, next_pos.1 + 1)], speed);
                }
            }
            ']' => {
                if speed.1 != 0 {
                    move_player(map, vec![next_pos], speed)
                } else {
                    move_player(map, vec![next_pos, (next_pos.0, next_pos.1 - 1)], speed)
                }
            }
            _ => (),
        };
    }

    for pos in &positions {
        let next_pos = (pos.0 + speed.0, pos.1 + speed.1);
        map[next_pos.0 as usize][next_pos.1 as usize] = map[pos.0 as usize][pos.1 as usize];
        map[pos.0 as usize][pos.1 as usize] = '.';
    }
}

fn check_player_move(map: &Vec<Vec<char>>, positions: Vec<(i32, i32)>, speed: (i32, i32), able_to_move: &mut Vec<bool>) -> Vec<bool> {
    let mut is_able = true;

    for pos in &positions {
        let next_pos = (pos.0 + speed.0, pos.1 + speed.1);
        let next_tile = map[next_pos.0 as usize][next_pos.1 as usize];

        is_able = match next_tile {
            '.' => true,
            '[' => {
                if speed.1 != 0 {
                    !check_player_move(map, vec![next_pos], speed, able_to_move).contains(&false)
                } else {
                    !check_player_move(map, vec![next_pos, (next_pos.0, next_pos.1 + 1)], speed, able_to_move).contains(&false)
                }
            }
            ']' => {
                if speed.1 != 0 {
                    !check_player_move(map, vec![next_pos], speed, able_to_move).contains(&false)
                } else {
                    !check_player_move(map, vec![next_pos, (next_pos.0, next_pos.1 - 1)], speed, able_to_move).contains(&false)
                }
            }
            _ => false,
        };

        if !is_able {
            break;
        }
    }
    able_to_move.push(is_able);

    able_to_move.clone()
}

fn main() {
    let contents =
        std::fs::read_to_string("./input.txt").expect("Should have opened the input file");

    let mut map: Vec<Vec<char>> = Vec::new();
    let mut moves: Vec<char> = Vec::new();

    for line in contents.lines() {
        // Insert code here
        if line.contains("#") {
            map.push(line.chars().flat_map(|c|
                if c == 'O' {
                    vec!['[', ']']
                } else if c == '@' {
                    vec!['@', '.']
                } else {
                    vec![c, c]
                }).collect::<Vec<char>>());
        } else {
            let mut tmp = line.chars().collect::<Vec<char>>();
            moves.append(&mut tmp);
        }
    }

    let mut player_pos: (i32, i32) = (0, 0);
    for (row_index, row) in map.iter().enumerate() {
        if let Some(col_index) = row.iter().position(|&c| c == '@') {
            player_pos = (row_index as i32, col_index as i32);
        }
    }
    for ch in moves {
        let speed = (
            if ch == '^' {
                -1
            } else if ch == 'v' {
                1
            } else {
                0
            },
            if ch == '<' {
                -1
            } else if ch == '>' {
                1
            } else {
                0
            },
        );
        let mut able_to_move: Vec<bool> = vec![];
        check_player_move(&map, vec![player_pos], speed, &mut able_to_move);
        if !able_to_move.contains(&false) {
            move_player(&mut map, vec![player_pos], speed);
            player_pos.0 += speed.0;
            player_pos.1 += speed.1;
        }
    }

    let mut res = 0;
    for (i, l) in map.iter().enumerate() {
        for (j, c) in l.iter().enumerate() {
            print!("{c}");
            if *c == '[' {
                res += i * 100 + j;
            }
        }
        println!();
    }
    println!("The result is {}", res);
}
