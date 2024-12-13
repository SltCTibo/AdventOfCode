use std::collections::HashMap;

#[derive(PartialEq, Eq, Clone, Copy)]
enum Wall {
    Top,
    Bottom,
    Left,
    Right
}

fn delete_region_from_map(map: &mut Vec<String>, region: &HashMap<(i32, i32), Vec<Wall>>) {
    for (pos, _) in region {
        map[pos.0 as usize].replace_range(pos.1 as usize..(pos.1+1) as usize, ".");
   	}
}

fn check_sides(map: &Vec<String>, ch: char, pos: &(i32, i32)) -> Vec<Wall> {
    let mut walls: Vec<Wall> = vec![];

    if pos.0 - 1 >= 0 {
        if let Some(next) = map[pos.0 as usize - 1].chars().nth(pos.1 as usize) {
            if next != ch {
                walls.push(Wall::Top);
            }
        }
    } else {
        walls.push(Wall::Top);
    }

    if pos.0 + 1 < map.len() as i32 {
        if let Some(next) = map[pos.0 as usize + 1].chars().nth(pos.1 as usize) {
            if next != ch {
                walls.push(Wall::Bottom);
            }
        }
    } else {
        walls.push(Wall::Bottom);
    }

    if pos.1 - 1 >= 0 {
        if let Some(next) = map[pos.0 as usize].chars().nth(pos.1 as usize - 1) {
            if next != ch {
                walls.push(Wall::Left);
            }
        }
    } else {
        walls.push(Wall::Left);
    }

    if pos.1 + 1 < map[pos.0 as usize].len() as i32 {
        if let Some(next) = map[pos.0 as usize].chars().nth(pos.1 as usize + 1) {
            if next != ch {
                walls.push(Wall::Right);
            }
        }
    } else {
        walls.push(Wall::Right);
    }

    walls
}

fn build_region(map: &Vec<String>, ch: char, pos: &(i32, i32), region: &mut HashMap<(i32, i32), Vec<Wall>>) {
	let current_walls = check_sides(map, ch, pos);

	region.insert(*pos, current_walls);

	if pos.0 - 1 >= 0 {
	    if let Some(next) = map[pos.0 as usize - 1].chars().nth(pos.1 as usize) {
			if !region.contains_key(&(pos.0 - 1, pos.1)) {
                if next == ch {
                    build_region(map, ch, &(pos.0 - 1, pos.1), region)
                }
            }
		}
	}

	if pos.0 + 1 < map.len() as i32 {
        if let Some(next) = map[pos.0 as usize + 1].chars().nth(pos.1 as usize) {
            if !region.contains_key(&(pos.0 + 1, pos.1)) {
                if next == ch {
                    build_region(map, ch, &(pos.0 + 1, pos.1), region)
                }
            }
        }
	}

	if pos.1 - 1 >= 0 {
	   if let Some(next) = map[pos.0 as usize].chars().nth(pos.1 as usize - 1) {
			if !region.contains_key(&(pos.0, pos.1 - 1)) {
                if next == ch {
                    build_region(map, ch, &(pos.0, pos.1 - 1), region)
                }
            }
        }
	}

	if pos.1 + 1 < map[pos.0 as usize].len() as i32 {
        if let Some(next) = map[pos.0 as usize].chars().nth(pos.1 as usize + 1) {
            if !region.contains_key(&(pos.0, pos.1 + 1)) {
                if next == ch {
                    build_region(map, ch, &(pos.0, pos.1 + 1), region)
                }
            }
        }
	}
}

fn count_corners(region: &HashMap<(i32, i32), Vec<Wall>>) -> u64 {
    let mut corners: u64 = 0;

    for (pos, walls) in region {
        match walls.len() {
            4 => {
                return 4
            }
            3 => {
                corners += 2;
            }
            _ => {
                if walls.len() == 2 && !((walls.contains(&Wall::Left) && walls.contains(&Wall::Right)) || (walls.contains(&Wall::Top) && walls.contains(&Wall::Bottom))) {
                    corners += 1;
                }
                if !region.contains_key(&(pos.0 - 1, pos.1 - 1)) && region.contains_key(&(pos.0, pos.1 - 1)) && region.contains_key(&(pos.0 - 1, pos.1)) {
                    corners += 1
                }
                if !region.contains_key(&(pos.0 + 1, pos.1 - 1)) && region.contains_key(&(pos.0, pos.1 - 1)) && region.contains_key(&(pos.0 + 1, pos.1)) {
                    corners += 1
                }
                if !region.contains_key(&(pos.0 - 1, pos.1 + 1)) && region.contains_key(&(pos.0, pos.1 + 1)) && region.contains_key(&(pos.0 - 1, pos.1)) {
                    corners += 1
                }
                if !region.contains_key(&(pos.0 + 1, pos.1 + 1)) && region.contains_key(&(pos.0, pos.1 + 1)) && region.contains_key(&(pos.0 + 1, pos.1)) {
                    corners += 1
                }
            }
        }
    }

    corners
}

fn main() {
	let contents = std::fs::read_to_string("./input.txt").expect("Should have been reading the file");

	let mut map: Vec<String> = contents.lines().map(|l| l.to_string()).collect();

	let mut res: u64 = 0;

	for row in 0..map.len() {
		for col in 0..map[row].len()  {
		    let ch = map[row].chars().nth(col).unwrap();
			if ch != '.' {
			    let mut region: HashMap<(i32, i32), Vec<Wall>> = HashMap::new();
                build_region(&map, ch, &(row as i32, col as i32), &mut region);
                let sides = count_corners(&region);
                res += sides * region.len() as u64;
                delete_region_from_map(&mut map, &region);
                region.clear();
			}
		}
	}

	println!("The result is {res}");
}
