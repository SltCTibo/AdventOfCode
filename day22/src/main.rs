use std::collections::HashMap;

fn main() {
    let contents =
        std::fs::read_to_string("./input.txt").expect("Should have opened the input file");

    let mut secret_numbers: Vec<i64> = contents.lines().map(|v| v.parse::<i64>().unwrap()).collect();

    let mut overall_seq = HashMap::new();
    let mut sequence: Vec<i64> = vec![];
    let mut res = 0;
    for nb in secret_numbers.iter_mut() {
        let mut t: i64 = nb.clone();
        let mut banana_seq = HashMap::new();
        let mut before = t % 10;
        for _ in 0..2000{
            let to_mix = t * 64;
            t ^= to_mix;
            t %= 16777216;
            let to_mix = t / 32;
            t ^= to_mix;
            t %= 16777216;
            let to_mix = t * 2048;
            t ^= to_mix;
            t %= 16777216;
            if sequence.len() == 4 {
                sequence.remove(0);
            }
            sequence.push(t % 10 - before);
            before = t % 10;
            if sequence.len() == 4 {
                if !banana_seq.contains_key(&sequence) {
                    banana_seq.insert(sequence.clone(), before);
                }
            }
        }
        for (seq, banana) in banana_seq {
            *overall_seq.entry(seq).or_insert(0) += banana;
        }
        res += t;
    }

    if let Some((key, &value)) = overall_seq.iter().max_by_key(|&(_k, v)|v) {
        println!("Best banana is {:?} for {:?} bananas", key, value);
    }

    println!("The result is {res}");
}
