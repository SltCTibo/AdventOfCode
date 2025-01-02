use std::collections::HashMap;

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
    None,
}

#[derive(PartialEq, Eq, Clone, Debug)]
struct Node {
    pos: (usize, usize),
    prev: Vec<(usize, usize)>
}

fn get_edges(
    map: &Vec<Vec<char>>,
    pos: &(usize, usize),
    dir: &Direction,
) -> HashMap<(usize, usize), Direction> {
    let mut neighbors: HashMap<(usize, usize), Direction> = HashMap::new();
    let rows = map.len();
    let cols = map[0].len();

    // Ensure we don't go out of bounds while accessing map elements
    if pos.0 > 0 && map[pos.0 - 1][pos.1] != '#' && *dir != Direction::Down {
        neighbors.insert((pos.0 - 1, pos.1), Direction::Up);
    }
    if pos.0 < rows - 1 && map[pos.0 + 1][pos.1] != '#' && *dir != Direction::Up {
        neighbors.insert((pos.0 + 1, pos.1), Direction::Down);
    }
    if pos.1 < cols - 1 && map[pos.0][pos.1 + 1] != '#' && *dir != Direction::Left {
        neighbors.insert((pos.0, pos.1 + 1), Direction::Right);
    }
    if pos.1 > 0 && map[pos.0][pos.1 - 1] != '#' && *dir != Direction::Right {
        neighbors.insert((pos.0, pos.1 - 1), Direction::Left);
    }

    neighbors
}

fn get_path(map: &Vec<Vec<char>>, start: &(usize, usize), end: &(usize, usize)) -> Node {
    let mut current_node: Node = Node {
        pos: *start,
        prev: vec![*start]
    };
    let mut current_dir = Direction::None;

    while current_node.pos != *end {
        let edge = get_edges(map, &current_node.pos, &current_dir);

        let (new_pos, new_dir) = edge.iter().next().unwrap();
        let mut new_path: Vec<(usize, usize)> = current_node.prev.clone();
        new_path.push(*new_pos);
        current_node = Node {
            pos: *new_pos,
            prev: new_path
        };
        current_dir = *new_dir;
    }

    current_node
}

fn main() {
    let contents =
        std::fs::read_to_string("./input.txt").expect("Should have opened the input file");

    let mut start: (usize, usize) = (0, 0);
    let mut end: (usize, usize) = (0, 0);

    let mut map: Vec<Vec<char>> = contents.lines().enumerate().map(|(i, line)|
        line.chars().enumerate().map(|(j, c)|{
            if c == 'S' {
                start = (i, j);
            } else if c == 'E' {
                end = (i, j);
            }
            return c
        }).collect::<Vec<char>>()
    ).collect();

    let mut res = 0;
    let first_node = get_path(&mut map, &start, &end);
    for (index, pos) in first_node.prev.iter().enumerate() {
        let to_check: Vec<(usize, (usize, usize))> = first_node.prev
            .iter()
            .enumerate()
            .filter(|&(target_idx, &(x, y))| {
                x.abs_diff(pos.0) + y.abs_diff(pos.1) <= 20 && target_idx as i32 - index as i32 - (x.abs_diff(pos.0) + y.abs_diff(pos.1)) as i32 >= 100
            })
            .map(|(i, &pt)| (i, pt))
            .collect();
        res += to_check.len();
    }

    println!("Result is {res}");
}
