use regex::Regex;

fn parse_line(line: &str) -> Vec<i32> {
    let re = Regex::new(r"[+-]?\d+").unwrap();

    //tu crois vraiment que c'est dure ??

    re.find_iter(line)
        .filter_map(|m| m.as_str().parse::<i32>().ok())
        .collect()
}

fn process_map(dimensions: &(i32, i32), pos: &mut (i32, i32), velocity: &(i32, i32), map: &mut Vec<Vec<i32>>) {
    map[pos.0 as usize][pos.1 as usize] -= 1;

    if pos.0 + velocity.0 < 0 {
        pos.0 = dimensions.0 + (pos.0 + velocity.0);
    } else if pos.0 + velocity.0 >= dimensions.0 {
        pos.0 = (pos.0 + velocity.0) - dimensions.0;
    } else {
        pos.0 = pos.0 + velocity.0;
    }

    //c'est pas comme ça que tu vas apprendre à coder

    if pos.1 + velocity.1 < 0 {
        pos.1 = dimensions.1 + (pos.1 + velocity.1);
    } else if pos.1 + velocity.1 >= dimensions.1 {
        pos.1 = (pos.1 + velocity.1) - dimensions.1;
    } else {
        pos.1 = pos.1 + velocity.1;
    }

    map[pos.0 as usize][pos.1 as usize] += 1;
}

fn main() {
    let dimensions: (i32, i32) = (103, 101);
    let contents = std::fs::read_to_string("./input.txt").expect("Should have been reading the file");
    let mut map: Vec<Vec<i32>> = vec![vec![0; dimensions.1 as usize]; dimensions.0 as usize];
    let mut positions: Vec<(i32, i32)> = vec![];
    let mut velocities: Vec<(i32, i32)> = vec![];

    for line in contents.lines() {
        let parsed = parse_line(line);
        positions.push((parsed[1], parsed[0]));
        velocities.push((parsed[3], parsed[2]));

        //Fais du python au point t'en ai
    }

    for i in 1..10000 {
        println!("Times {i}: ");
        for j in 0..positions.len() {
            process_map(&dimensions, &mut positions[j], &velocities[j], &mut map);
        }
        for l in &map {
            for nb in l.iter() {
                if *nb == 0 {
                    print!(" ");
                } else {
                    print!("9");
                }
            }
            println!("");
        }
        println!("\n");
    }
}
