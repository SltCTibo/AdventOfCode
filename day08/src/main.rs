use std::collections::HashMap;

fn main() {
    let contents = std::fs::read_to_string("./input.txt").expect("Should have opened the file");

    let mut antennas: HashMap<char, Vec<(i32, i32)>> = HashMap::new();

    let mut map: Vec<String> = Vec::<String>::new();

    for (row, line) in contents.lines().enumerate() {
        map.push(line.to_string());
        for (col, ch) in line.chars().enumerate() {
            if ch != '.' {
                if !antennas.contains_key(&ch) {
                    antennas.insert(ch, vec![]);
                }
                if let Some(value) = antennas.get_mut(&ch) {
                    value.push((row as i32, col as i32));
                }
            }
        }
    }

    let mut antinodes: Vec<(i32, i32)>  = Vec::<(i32, i32)>::new();

    for key in antennas.keys() {
        if let Some(ant) = antennas.get(key) {
            for i in 0..ant.len() {
                for j in i+1..ant.len() {
                    let gap: (i32, i32) = (ant[i].0 - ant[j].0, ant[i].1 - ant[j].1);
                    let mut projector: i32 = 1;
                    let mut exit_flag: u8 = 0;
                    while exit_flag < 2 {
                        exit_flag = 0;
                        let antinode_1 = (ant[i].0 + (gap.0 * projector), ant[i].1 + (gap.1 * projector));
                        if antinode_1.0 >= 0 && antinode_1.0 < map[0].len() as i32 && antinode_1.1 >= 0 && antinode_1.1 < map.len() as i32 {
                            if !antinodes.contains(&antinode_1) {
                                antinodes.push(antinode_1);
                            }
                        } else {
                            exit_flag += 1;
                        }
                        let antinode_2 = (ant[j].0 - (gap.0 * projector), ant[j].1 - (gap.1 * projector));
                        if antinode_2.0 >= 0 && antinode_2.0 < map[0].len() as i32 && antinode_2.1 >= 0 && antinode_2.1 < map.len() as i32  {
                            if !antinodes.contains(&antinode_2) {
                                antinodes.push(antinode_2);
                            }
                        } else {
                            exit_flag += 1;
                        }
                        projector += 1;
                    }
                }
            }
        }
    }

    for (_, values) in antennas.iter() {
        if values.len() > 1 {
            for val in values.iter() {
                if !antinodes.contains(val) {
                    antinodes.push(*val);
                }
            }
        }
    }

    for (row, col) in antinodes.iter() {
        map[*row as usize].replace_range(*col as usize..*col as usize + 1, "#");
    }

    for map_line in map.iter() {
        println!("{map_line}");
    }

    println!("There is {} antinodes", antinodes.len());
}
