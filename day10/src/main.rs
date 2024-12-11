fn build_trailheads(map: &Vec<&str>, ch_pos: &(i32, i32), ch: char,  end: &mut Vec<(i32, i32)>) {
    if ch == '9' {
        if !end.contains(ch_pos) {
            end.push(*ch_pos);
        }
        return;
    }

    if ch_pos.0 - 1 >= 0 {
        let top_char = map[ch_pos.0 as usize - 1].chars().nth(ch_pos.1 as usize).unwrap();
        if top_char as u8 == ch as u8 + 1 {
            build_trailheads(map, &(ch_pos.0 - 1, ch_pos.1), top_char, end);
        }
    }

    if ch_pos.0 as usize + 1 < map.len() {
        let bottom_char = map[ch_pos.0 as usize + 1].chars().nth(ch_pos.1 as usize).unwrap();
        if bottom_char as u8 == ch as u8 + 1 {
            build_trailheads(map, &(ch_pos.0 + 1, ch_pos.1), bottom_char, end);
        }
    }

    if ch_pos.1 - 1 >= 0 {
        let left_char = map[ch_pos.0 as usize].chars().nth(ch_pos.1 as usize - 1).unwrap();
        if left_char as u8 == ch as u8 + 1 {
            build_trailheads(map, &(ch_pos.0, ch_pos.1 - 1), left_char, end);
        }
    }

    if ch_pos.1 as usize + 1 < map[0].len() {
        let right_char = map[ch_pos.0 as usize].chars().nth(ch_pos.1 as usize + 1).unwrap();
        if right_char as u8 == ch as u8 + 1 {
            build_trailheads(map, &(ch_pos.0, ch_pos.1 + 1), right_char, end);
        }
    }

    return;
}

fn main() {
    let contents = std::fs::read_to_string("./input.txt").expect("Supposed to read the file");

    let mut map: Vec<&str> = Vec::new();

    for line in contents.lines() {
        map.push(line);
    }

    let mut res: u32 = 0;

    let mut end: Vec<(i32, i32)> = vec![];
    for i in 0..map.len() {
        for j in 0..map[i].len() {
            if let Some(ch) = map[i].chars().nth(j) {
                if ch == '0' {
                    build_trailheads(&map, &(i as i32, j as i32), ch, &mut end);
                    res += end.len() as u32;
                    end.clear();
                }
            }
        }
    }

    println!("The result is {res}");
}
