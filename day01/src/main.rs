use std::fs;

fn part_one(list1: Vec<u32>, list2: Vec<u32>) {
    let mut res: u64 = 0;
    for (val1, val2) in list1.iter().zip(list2.iter()) {
        res += (*val1 as i64 - *val2 as i64).abs() as u64;
    }

    println!("res = {res}");
}

fn part_two(list1: Vec<u32>, list2: Vec<u32>) {
    let mut sim_score = 0;
    for nb in list1.into_iter() {
        let similarity: Vec<u32> = list2.clone().into_iter().filter(|x| x == &nb).collect();
        sim_score += nb * similarity.len() as u32;
    }
    println!("Similarity Score = {sim_score}");
}

fn main() {
    // Lire le fichier
    let contents = fs::read_to_string("./list.txt")
                                .expect("Should have been able to read the file");

    // Créer les listes
    let mut list1: Vec<u32> = Vec::new();
    let mut list2: Vec<u32> = Vec::new();

    // Parcourir chaque ligne
    for line in contents.lines() {
        let mut values = line.split_whitespace();
        if let Some(val1) = values.next() {
            if let Ok(parsed_val1) = val1.parse::<u32>() {
                list1.push(parsed_val1);
            } else {
                eprintln!("Failed to parse value: {val1}");
            }
        }
        if let Some(val2) = values.next() {
            if let Ok(parsed_val2) = val2.parse::<u32>() {
                list2.push(parsed_val2);
            } else {
                eprintln!("Failed to parse value: {val2}");
            }
        }
    }

    // Trier les listes
    list1.sort_unstable();
    list2.sort_unstable();

    part_one(list1.clone(), list2.clone());
    part_two(list1.clone(), list2.clone());
}
