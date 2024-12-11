use std::collections::HashMap;

const MAX_BLINK: u32 = 75;

fn count_pebbels(value: u64, step: u32, memo: &mut HashMap<(u64, u32), u64>) -> u64 {
    if let Some(&cached_result) = memo.get(&(value, step)) {
        return cached_result;
    }

    if step == 0 {
        return 1
    }

    let mut counter: u64 = 0;

    let digit_count = ((value as f64).log10().floor() as usize) + 1;
    if digit_count % 2 == 0 {
        let str_tmp = value.to_string();
        let (first, second) = str_tmp.split_at(str_tmp.len() / 2);
        counter += count_pebbels(first.parse::<u64>().unwrap(), step - 1, memo) + count_pebbels(second.parse::<u64>().unwrap(), step - 1, memo);
    } else {
        if value == 0 {
            counter += count_pebbels(1, step - 1, memo);
        } else {
            counter += count_pebbels(value * 2024, step - 1, memo);
        }
    }

    memo.insert((value, step), counter);
    counter
}

fn main() {
    let contents = std::fs::read_to_string("./input.txt").expect("Should have been reading the file");

    let mut pebbles: Vec<u64> = vec![];

    pebbles = contents.split_whitespace().map(|v| v.parse::<u64>().unwrap()).collect();
    let mut res: u64 = 0;
    let mut memo: HashMap<(u64, u32), u64> = HashMap::new();

    for pebble in pebbles {
        res += count_pebbels(pebble, MAX_BLINK, &mut memo);
    }

    println!("The result is {res}");
}
