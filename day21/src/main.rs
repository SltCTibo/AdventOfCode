use std::collections::HashMap;

fn set_paths(pad: &HashMap<char, (i32, i32)>, paths: &mut HashMap<(char, char), Vec<char>>, empty: (i32, i32)) {
    for (i, first) in pad.iter() {
        for (j, second) in pad.iter() {
            let mut result: Vec<char> = vec![];
            if first.1 > second.1 {
                for _ in 0..first.1 - second.1 {
                    result.push('<');
                }
            }
            if first.0 < second.0 {
                for _ in 0..second.0 - first.0 {
                    result.push('v');
                }
            }
            if first.0 > second.0 {
                for _ in 0..first.0 - second.0 {
                    result.push('^');
                }
            }
            if first.1 < second.1 {
                for _ in 0..second.1 - first.1 {
                    result.push('>');
                }
            }
            if (first.0, second.1) == empty || (second.0, first.1) == empty {
                result.reverse();
            }
            result.push('A');
            paths.insert((*i, *j), result);
        }
    }
}

fn complexity(paths: &HashMap<(char, char), Vec<char>>, prev: char, current: char, index: i32, memo: &mut HashMap<(char, char, i32), usize>) -> usize {
    if let Some(res) = memo.get(&(prev, current, index)) {
        return *res
    }

    let mut result: usize = 0;
    let path = paths.get(&(prev, current)).unwrap();

    if index == 0 {
        return path.len();
    }

    let mut new_prev = 'A';
    for ch in path.iter() {
        result += complexity(paths, new_prev, *ch, index - 1, memo);
        new_prev = *ch;
    }

    memo.insert((prev, current, index), result);
    result
}

fn main() {
    let contents =
        std::fs::read_to_string("./input.txt").expect("Should have opened the input file");

    let codes: Vec<&str> = contents.lines().collect();

    let mut numpad = HashMap::new();
    numpad.insert('7', (0, 0));
    numpad.insert('8', (0, 1));
    numpad.insert('9', (0, 2));
    numpad.insert('4', (1, 0));
    numpad.insert('5', (1, 1));
    numpad.insert('6', (1, 2));
    numpad.insert('1', (2, 0));
    numpad.insert('2', (2, 1));
    numpad.insert('3', (2, 2));
    numpad.insert('0', (3, 1));
    numpad.insert('A', (3, 2));

    let mut direction = HashMap::new();
    direction.insert('^', (0, 1));
    direction.insert('A', (0, 2));
    direction.insert('<', (1, 0));
    direction.insert('v', (1, 1));
    direction.insert('>', (1, 2));

    let mut paths: HashMap<(char, char), Vec<char>> = HashMap::new();

    set_paths(&numpad, &mut paths, (3, 0));
    set_paths(&direction, &mut paths, (0, 0));

    let mut memo = HashMap::new();
    let mut res = 0;
    for code in &codes {
        let mut result: usize = 0;
        let mut prev = 'A';
        for digit in code.chars() {
            result += &complexity(&paths, prev, digit, 25, &mut memo);
            prev = digit;
        }
        let num = code[0..3].parse::<i128>().unwrap();
        res += num * result as i128;
        println!();
    }

    println!("The Result is {res}");
}
