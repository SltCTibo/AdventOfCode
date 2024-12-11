fn main() {
    let mut contents = std::fs::read_to_string("./input.txt").expect("Should have been reading the file");

    let mut res: u128 = 0;
    let mut reference_map: Vec<u32> = Vec::<u32>::new();

    // Map the disk spaces
    let mut replacing_char = "block".to_string();
    while !contents.is_empty() {
        if let Some(val) = contents.remove(0).to_digit(10) {
            if val == 0 && replacing_char == "block" {
                continue;
            }

            reference_map.push(val);

            if replacing_char == "memory" {
                replacing_char = "block".to_string();
            } else {
                replacing_char = "memory".to_string();
            }
        } else {
            break;
        }
    }

    let mut final_map: Vec<(u32, (u32, u32))> = Vec::new();
    let mut iter = reference_map.into_iter();
    let mut file_id = 0;
    while let Some(first) = iter.next() {
        let second = iter.next().unwrap_or(0);
        final_map.push((file_id, (first, second)));
        file_id += 1;
    }

    let mut i = final_map.len() -1;

    while i > 0 {
        for j in 0..i {
            if final_map[j].1.1 >= final_map[i].1.0 {
                let mut value_to_move = final_map.remove(i);
                final_map[i-1].1.1 += value_to_move.1.0 + value_to_move.1.1;
                value_to_move.1.1 = final_map[j].1.1 - value_to_move.1.0;
                final_map[j].1.1 = 0;
                final_map.insert(j+1, value_to_move);
                i += 1;
                break;
            }
        }
        i -= 1;
    }

    let mut iter_id = 0;
    for (idx, (block, free_space)) in &final_map {
        for i in 0..*block {
            res += (idx * (iter_id + i)) as u128;
        }
        iter_id += block + free_space;
    }

    println!("The actuall size is {}", res);
}
