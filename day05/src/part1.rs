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
        let mut to_add = true;

        for (i, page) in update.clone().into_iter().enumerate() {
            if update[..i].iter().any(|prev| rules.contains(&format!("{}|{}", page, prev))) {
                to_add = false;
                break;
            }
        }

        if to_add {
            res += update[(update.len() / 2) as usize].parse::<u32>().unwrap();
        }
    }

    print!("{res}");
}
