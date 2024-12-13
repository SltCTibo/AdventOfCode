use regex::Regex;

fn extract_numbers(input: &str) -> Vec<i64> {
    let re = Regex::new(r"[+-]?\d+").unwrap();

    re.find_iter(input)
        .filter_map(|m| m.as_str().parse::<i64>().ok())
        .collect()
}

fn solve_equation(a: i64, b: i64, c: i64, d: i64, goal1: i64, goal2: i64) -> i64 {
    let det = (a * d) - (b * c);

    let res1 = ((d * goal1) + ((b * - 1) * goal2)) / det;
    let res2 = (((c * -1) * goal1) + (a * goal2)) / det;

    if (res1 * a) + (res2 * b) == goal1 && (res1 * c) + (res2 * d) == goal2 {
        (3 * res1) + res2
    } else {
        0
    }
}

fn main() {
    let contents = std::fs::read_to_string("./input.txt").expect("Should have been reading the file");

    let mut iter = contents.lines();
    let mut problem_line: Option<&str> = iter.next();

    let mut res: i64 = 0;

    while let Some(button_a) = problem_line {
        if let Some(button_b) = iter.next() {
            if let Some(prize) = iter.next() {
                let a = extract_numbers(button_a);
                let b = extract_numbers(button_b);
                let goals = extract_numbers(prize);

                res += solve_equation(a[0], b[0], a[1], b[1], goals[0] + 10000000000000, goals[1] + 10000000000000);
            }
        }
        iter.next();
        problem_line = iter.next();
    }

    println!("The result is {res}");
}
