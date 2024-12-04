const WORD: &str = "XMAS";
const REVERSED: &str = "SAMX";

fn check_diagonal(lines: Vec<String>, row: usize, col: usize, len: usize) -> u32 {
    let mut res: u32 = 0;
    let mut top_left = String::new();
    let mut top_right = String::new();
    let mut bot_left = String::new();
    let mut bot_right = String::new();

    if row >= 3 && col >= 3 {
        top_left.push(lines.get(row).unwrap().chars().nth(col).unwrap());
        top_left.push(lines.get(row - 1).unwrap().chars().nth(col - 1).unwrap());
        top_left.push(lines.get(row - 2).unwrap().chars().nth(col - 2).unwrap());
        top_left.push(lines.get(row - 3).unwrap().chars().nth(col - 3).unwrap());
        if top_left.eq(WORD) || top_left.eq(REVERSED) {
            res += 1;
        }
    }

    if row >= 3 && col + 3 < len {
        top_right.push(lines.get(row).unwrap().chars().nth(col).unwrap());
        top_right.push(lines.get(row - 1).unwrap().chars().nth(col + 1).unwrap());
        top_right.push(lines.get(row - 2).unwrap().chars().nth(col + 2).unwrap());
        top_right.push(lines.get(row - 3).unwrap().chars().nth(col + 3).unwrap());
        if top_right == WORD || top_right == REVERSED {
            res += 1;
        }
    }

    if row + 3 < lines.len() && col >= 3 {
        bot_left.push(lines.get(row).unwrap().chars().nth(col).unwrap());
        bot_left.push(lines.get(row + 1).unwrap().chars().nth(col - 1).unwrap());
        bot_left.push(lines.get(row + 2).unwrap().chars().nth(col - 2).unwrap());
        bot_left.push(lines.get(row + 3).unwrap().chars().nth(col - 3).unwrap());
        if bot_left.eq(WORD) || bot_left.eq(REVERSED) {
            res += 1;
        }
    }

    if row + 3 < lines.len() && col + 3 < len {
        bot_right.push(lines.get(row).unwrap().chars().nth(col).unwrap());
        bot_right.push(lines.get(row + 1).unwrap().chars().nth(col + 1).unwrap());
        bot_right.push(lines.get(row + 2).unwrap().chars().nth(col + 2).unwrap());
        bot_right.push(lines.get(row + 3).unwrap().chars().nth(col + 3).unwrap());
        if bot_right == WORD || bot_right == REVERSED {
            res += 1;
        }
    }

    res
}

fn check_vertical(lines: Vec<String>, row: usize, col: usize) -> u32 {
    let mut res: u32 = 0;
    let mut utod = String::new();
    let mut dtou = String::new();

    if row >= 3 {
        dtou.push(lines.get(row - 3).unwrap().chars().nth(col).unwrap());
        dtou.push(lines.get(row - 2).unwrap().chars().nth(col).unwrap());
        dtou.push(lines.get(row - 1).unwrap().chars().nth(col).unwrap());
        dtou.push(lines.get(row).unwrap().chars().nth(col).unwrap());
        if dtou.eq(WORD) || dtou.eq(REVERSED) {
            res += 1;
        }
    }

    if row + 3 < lines.len() {
        utod.push(lines.get(row).unwrap().chars().nth(col).unwrap());
        utod.push(lines.get(row + 1).unwrap().chars().nth(col).unwrap());
        utod.push(lines.get(row + 2).unwrap().chars().nth(col).unwrap());
        utod.push(lines.get(row + 3).unwrap().chars().nth(col).unwrap());
        if utod == WORD || utod == REVERSED {
            res += 1;
        }
    }

    res
}

fn check_horizontal(lines: Vec<String>, row: usize, col: usize, len: usize) -> u32 {
    let mut res: u32 = 0;
    let mut ltor;
    let mut rtol;

    if col >= 3 {
        rtol = String::from(&lines.get(row).unwrap()[col-3..col+1]);
        if rtol.eq(WORD) || rtol.eq(REVERSED) {
            res += 1;
        }
    }

    if col + 3 < len {
        ltor = String::from(&lines.get(row).unwrap()[col..col+4]);
        if ltor == WORD || ltor == REVERSED {
            res += 1;
        }
    }

    res
}

fn part_one(contents: String) {
    let values: Vec<String> = contents.lines().map(String::from).collect();
    let mut res = 0;

    for (row, line) in contents.lines().enumerate() {
        for (col, ch) in line.chars().enumerate() {
            if ch == 'X' {
                res += check_horizontal(values.clone(), row, col, line.len());
                res += check_vertical(values.clone(), row, col);
                res += check_diagonal(values.clone(), row, col, line.len());
            }
        }
    }

    print!("XMAS is {res} times in the file");
}

fn check_cross(values: Vec<String>, row: usize, col: usize, len: usize) -> u32 {
    if row > 0 && row + 1 < values.len() && col > 0 && col + 1 < len {
        let mut ltor: String = String::new();
        let mut rtol: String = String::new();

        ltor.push(values.get(row - 1).unwrap().chars().nth(col - 1).unwrap());
        ltor.push(values.get(row).unwrap().chars().nth(col).unwrap());
        ltor.push(values.get(row + 1).unwrap().chars().nth(col + 1).unwrap());

        rtol.push(values.get(row - 1).unwrap().chars().nth(col + 1).unwrap());
        rtol.push(values.get(row).unwrap().chars().nth(col).unwrap());
        rtol.push(values.get(row + 1).unwrap().chars().nth(col - 1).unwrap());

        if (ltor == "SAM" || ltor == "MAS") && (rtol == "SAM" || rtol == "MAS") {
            return 1
        }
    }

    0
}

fn part_two(contents: String) {
    let values: Vec<String> = contents.lines().map(String::from).collect();
    let mut res: u32 = 0;

    for (row, line) in contents.lines().enumerate() {
        for (col, ch) in line.chars().enumerate() {
            if ch == 'A' {
                res += check_cross(values.clone(), row, col, line.len());
            }
        }
    }

    println!("X-MAS is {res} times in the file");
}

fn main() {
    let contents = std::fs::read_to_string("./input.txt").expect("Should have opened the input file");

    part_one(contents.clone());

    part_two(contents.clone());

}
