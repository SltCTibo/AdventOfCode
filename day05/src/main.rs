// Part2 Resolution, part1 is in part1.rs

fn repare_update(broken: &mut Vec<&str>, rules: &Vec<String>) -> u32 {

    for i in 0..broken.len() {
        for j in 0..i {
            if rules.contains(&format!("{}|{}", broken[i].to_string(), broken[j].to_string())) {
                broken.swap(i, j);
                return repare_update(broken, rules)
            }
        }
    }

    broken[(broken.len() / 2) as usize].parse::<u32>().unwrap()
}

fn main() {
    let contents = std::fs::read_to_string("./input.txt")
                                        .expect("Should have opened the file");

    let mut rules: Vec<String> = Vec::new();
    let mut updates = Vec::<Vec<&str>>::new();

    for line in contents.lines() {
        if line.contains("|") {
            rules.push(line.to_string());
        } else if line.contains(",") {
            updates.push(line.split(",").collect());
        }
    }

    let mut res: u32 = 0;

    for update in &updates {
        for (i, page) in update.clone().into_iter().enumerate() {
            if update[..i].iter().any(|prev| rules.contains(&format!("{}|{}", page, prev))) {
                res += repare_update(&mut update.clone(), &rules);
                break;
            }
        }
    }

    print!("{res}");
}
