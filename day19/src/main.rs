use std::collections::HashMap;

fn check_design(patterns: &Vec<String>, design: &mut String, max_patter_length: usize, memo: &mut HashMap<String, u64>) -> u64 {
    if let Some(already_know) = memo.get(design) {
        return *already_know;
    }

    if design.is_empty() {
        return 1
    }

    let mut res: u64 = 0;

    for i in 1..max_patter_length+1 {
        if i > design.len() { break }
        if patterns.contains(&design[0..i].to_string()) {
            res += check_design(patterns, &mut design.chars().skip(i).collect(), max_patter_length, memo);
        }
    }

    memo.insert(design.clone(), res);
    res
}

fn main() {
    let contents =
        std::fs::read_to_string("./input.txt").expect("Should have opened the input file");

    let mut lines = contents.split("\n\n");

    let patterns: Vec<String> = lines
        .next()
        .unwrap_or_default()
        .split(',')
        .map(|s| s.trim().to_string())
        .collect();

    let mut designs: Vec<String> = lines
        .next()
        .unwrap_or_default()
        .lines()
        .map(|line| line.trim().to_string())
        .collect();

    let max_pattern_length = patterns.iter().max_by(|x, y| x.len().cmp(&y.len())).unwrap().len();

    let mut res = 0;
    let mut memo: HashMap<String, u64> = HashMap::new();
    for design in designs.iter_mut() {
        let nb_combinaison = check_design(&patterns, design, max_pattern_length, &mut memo);
        if nb_combinaison > 0 {
            println!("nb Combinaisons: {nb_combinaison}");
            res += nb_combinaison;
        }
    }

    println!("The result is {res}");
}
