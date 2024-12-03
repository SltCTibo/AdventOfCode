use regex::Regex;

fn part_one(contents: String) {
    let set = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    let mut mul: u128 = 0;
    for (_, [val1, val2]) in set.captures_iter(contents.as_str()).map(|c| c.extract()) {
        mul += (val1.parse::<u32>().unwrap() * val2.parse::<u32>().unwrap()) as u128;
    }

    println!("I got this from regex: {mul}");
}

fn part_two(contents: String) {
    let set = Regex::new(r"mul\((\d+),(\d+)\)|do\(\)|don't\(\)").unwrap();

    let mut do_calc: bool = true;

    let mut mul: u128 = 0;
    for cap in set.captures_iter(contents.as_str()) {
        if let Some(val) = cap.get(0).map(|m| m.as_str()) {
            match val {
                "do()" => {
                    do_calc = true;
                }
                "don't()" => {
                    do_calc = false;
                }
                _ => {
                    if do_calc {
                        if let Some(val1) = cap.get(1).map(|m| m.as_str()) {
                            if let Some(val2) = cap.get(2).map(|m| m.as_str()) {
                                mul += (val1.parse::<u32>().unwrap() * val2.parse::<u32>().unwrap()) as u128;
                            }
                        }
                    }
                }
            }
        }
    }

    println!("I got this from regex: {:?}", mul);
}

fn main() {
    let contents = std::fs::read_to_string("./input.txt")
                                        .expect("Should have been able to read the file");

    part_one(contents.clone());
    part_two(contents.clone());
}
