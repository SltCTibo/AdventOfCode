use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

#[derive(PartialEq, Eq, Debug, Clone, Copy, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
    None
}

#[derive(Debug, Clone)]
struct Node {
    pub score: i32,
    pub prev: Vec<(usize, usize)>
}

#[derive(Eq, Debug)]
struct State {
    score: i32,
    position: (usize, usize),
    direction: Direction,
    path: Vec<(usize, usize)>
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Note: we use reverse ordering for min-heap
        other.score.cmp(&self.score)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.score == other.score
    }
}

fn get_neighbors(
    map: &Vec<Vec<char>>,
    pos: &(usize, usize),
    dir: &Direction,
) -> HashMap<(usize, usize), Direction> {
    let mut neighbors: HashMap<(usize, usize), Direction> = HashMap::new();

    if map[pos.0 - 1][pos.1] == '.' && *dir != Direction::Down {
        neighbors.insert((pos.0 - 1, pos.1), Direction::Up);
    }
    if map[pos.0 + 1][pos.1] == '.' && *dir != Direction::Up {
        neighbors.insert((pos.0 + 1, pos.1), Direction::Down);
    }
    if map[pos.0][pos.1 + 1] == '.' && *dir != Direction::Left {
        neighbors.insert((pos.0, pos.1 + 1), Direction::Right);
    }
    if map[pos.0][pos.1 - 1] == '.' && *dir != Direction::Right {
        neighbors.insert((pos.0, pos.1 - 1), Direction::Left);
    }

    neighbors
}

fn dijkstra(map: &mut Vec<Vec<char>>, start: &(usize, usize), end: &(usize, usize), q: &mut HashMap<(usize, usize, Direction), Node>) -> i32 {
    let mut queue = BinaryHeap::new();

    queue.push(State {
        score: 0,
        position: *start,
        direction: Direction::None,
        path: vec![*start]
    });

    while let Some(State {
        score,
        position,
        direction,
        path
    }) = queue.pop() {

        if position == *end {
            return score;
        }

        let current = q.get(&(position.0, position.1, direction)).unwrap();
        if score > current.score {
            continue;
        }

        let edges = get_neighbors(map, &position, &direction);

        for (next_pos, next_dir) in edges.iter() {
            let mut new_score = score + 1;
            if direction != Direction::None && direction != *next_dir {
                new_score += 1000;
            }

            let mut new_path = path.clone();
            new_path.push(*next_pos);

            if let Some(node) = q.get_mut(&(next_pos.0, next_pos.1, *next_dir)) {
                if new_score < node.score {
                    q.insert((next_pos.0, next_pos.1, *next_dir), Node {
                        score: new_score,
                        prev: new_path.clone()
                    });
                    queue.push(State {
                        score: new_score,
                        position: *next_pos,
                        direction: *next_dir,
                        path: new_path
                    });
                } else if new_score == node.score {
                    for point in &node.prev {
                        if !new_path.contains(point) {
                            new_path.push(*point);
                        }
                    }
                    node.prev = new_path.clone();
                    queue.push(State {
                        score: new_score,
                        position: *next_pos,
                        direction: *next_dir,
                        path: new_path
                    });
                }
            } else {
                q.insert((next_pos.0, next_pos.1, *next_dir), Node {
                    score: new_score,
                    prev: new_path.clone()
                });
                queue.push(State {
                    score: new_score,
                    position: *next_pos,
                    direction: *next_dir,
                    path: new_path
                });
            }
        }
    }

    -1
}

fn main() {
    let mut contents = std::fs::read_to_string("input.txt").expect("Should have open the file");

    contents = contents.replace("E", ".");

    let mut map: Vec<Vec<char>> = vec![];

    for line in contents.lines() {
        map.push(line.chars().collect());
    }

    let start: (usize, usize) = (map.len() - 2, 1);
    let end: (usize, usize) = (1, map[0].len() - 2);

    let mut q = HashMap::new();
    q.insert((start.0, start.1, Direction::None), Node {
        score: 0,
        prev: vec![start]
    });
    let score = dijkstra(&mut map, &start, &end, &mut q);

    let node = q.get(&(end.0, end.1, Direction::Up)).unwrap();

    for (i, j) in &node.prev {
        map[*i][*j] = 'O';
    }
    for line in &map {
        for ch in line {
            print!("{ch}");
        }
        println!();
    }
    println!("There are {} O", node.prev.len());

    println!("The shortest path is {}", score);
}
