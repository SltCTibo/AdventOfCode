use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::io::Error;

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
    step: i32,
    manhattan: i32,
    prev: Vec<(usize, usize)>,
}

#[derive(Eq, Debug)]
struct State {
    weight: i32,
    step: i32,
    position: (usize, usize),
    direction: Direction,
    path: Vec<(usize, usize)>,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Note: we use reverse ordering for min-heap
        other.weight.cmp(&self.weight)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.weight == other.weight
    }
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
    if pos.0 > 0 && map[pos.0 - 1][pos.1] == '.' && *dir != Direction::Down {
        neighbors.insert((pos.0 - 1, pos.1), Direction::Up);
    }
    if pos.0 < rows - 1 && map[pos.0 + 1][pos.1] == '.' && *dir != Direction::Up {
        neighbors.insert((pos.0 + 1, pos.1), Direction::Down);
    }
    if pos.1 < cols - 1 && map[pos.0][pos.1 + 1] == '.' && *dir != Direction::Left {
        neighbors.insert((pos.0, pos.1 + 1), Direction::Right);
    }
    if pos.1 > 0 && map[pos.0][pos.1 - 1] == '.' && *dir != Direction::Right {
        neighbors.insert((pos.0, pos.1 - 1), Direction::Left);
    }

    neighbors
}

fn astar(map: &mut Vec<Vec<char>>, start: &(usize, usize), end: &(usize, usize)) -> Option<Node> {
    let mut queue = BinaryHeap::new();
    queue.push(State {
        weight: (start.0 + start.1) as i32,
        step: 0,
        position: *start,
        direction: Direction::None,
        path: vec![*start],
    });

    let mut q = HashMap::new();
    q.insert(
        *start,
        Node {
            step: 0,
            manhattan: (start.0 + start.1) as i32,
            prev: vec![*start],
        },
    );

    while let Some(State {
        weight,
        step,
        position,
        direction,
        path,
    }) = queue.pop() {
        if position == *end {
            return Some(Node {
                step,
                manhattan: (position.0 + position.1) as i32, // Replace with actual Manhattan distance to the goal
                prev: path,
            });
        }

        let edges = get_edges(map, &position, &direction);

        for (next_pos, next_dir) in &edges {
            let manhattan = (next_pos.0 + next_pos.1) as i32;
            let new_weight = step + 1 + manhattan;

            let mut new_path = path.clone();
            new_path.push(*next_pos);

            if let Some(node) = q.get_mut(next_pos) {
                if node.step + node.manhattan > new_weight {
                    node.step = step + 1;
                    node.manhattan = manhattan;
                    node.prev = new_path.clone();

                    queue.push(State {
                        weight: new_weight,
                        step: step + 1,
                        position: *next_pos,
                        direction: *next_dir,
                        path: new_path,
                    });
                }
            } else {
                q.insert(
                    *next_pos,
                    Node {
                        step: step + 1,
                        manhattan,
                        prev: new_path.clone(),
                    },
                );
                queue.push(State {
                    weight: new_weight,
                    step: step + 1,
                    position: *next_pos,
                    direction: *next_dir,
                    path: new_path,
                });
            }
        }
    }

    None
}

fn main() {
    let contents =
        std::fs::read_to_string("./input.txt").expect("Should have opened the input file");

    let corrupted: Vec<(usize, usize)> = contents
        .lines()
        .map(|line| {
            let v = line
                .split(',')
                .map(|s| s.parse::<usize>().unwrap())
                .collect::<Vec<_>>(); // Collect to Vec<usize> to access both parts
            (v[0], v[1])
        })
        .collect();

    let mut map: Vec<Vec<char>> = vec![vec!['.'; 71]; 71];

    let start: (usize, usize) = (0, 0);
    let end: (usize, usize) = (70, 70);

    for (x, y) in corrupted.iter() {
        map[*y][*x] = '#';
        match astar(&mut map, &start, &end) {
            Some(node) => {
                println!("{:?}", node.step);
            }
            None => {
                println!("No Path found because of pixel: ({x},{y})");
                break;
            },
        }
    }

    for ln in &map {
        for ch in ln {
            print!("{ch}");
        }
        println!();
    }
}
