use std::fs;

fn check_trend(line: Vec<i8>) -> String {
    if let (Some(&v1), Some(&v2)) = (line.get(0), line.get(1)) {
        if v1 < v2 {
            String::from("Increasing")
        } else if v1 > v2 {
            String::from("Decreasing")
        } else {
            String::from("NULL")
        }
    } else {
        String::from("Invalid input") // Handle case when indices are out of bounds
    }
}

fn check_if_safe(line: Vec<i8>) -> bool {

    let trend = check_trend(line[..2].to_vec());

    let mut iter = line.iter();
    let mut tmp = iter.next();

    match trend.as_str() {
        "Increasing" => {
            while let Some(val) = tmp {
                tmp = iter.next();
                if let Some(next_val) = tmp {
                    if val >= next_val || ((next_val - val) < 1 || (next_val - val) > 3) {
                        return false;
                    }
                }
            }
            return true
        }
        "Decreasing" => {
            while let Some(val) = tmp {
                tmp = iter.next();
                if let Some(next_val) = tmp {
                    if val <= next_val || ( (val - next_val) < 1 || (val - next_val) > 3 ) {
                        return false;
                    }
                }
            }
            return true
        }
        _ => {
            return false
        }
    }
}

fn part_one(values: Vec<Vec<i8>>) -> (u32, Vec<Vec<i8>>) {
    let mut safe_reports = 0;
    let mut unsafe_reports: Vec<Vec<i8>> = Vec::new();

    for line in values {
        if check_if_safe(line.clone()) {
            safe_reports += 1;
        } else {
            unsafe_reports.push(line.clone());
        }
    }

    (safe_reports, unsafe_reports)
}

fn part_two(values: Vec<Vec<i8>>) -> u32 {
    let mut safe_reports = 0;

    for line in values {
        for i in 0..line.len() {
            let mut tmp = line.clone();
            tmp.remove(i);
            if check_if_safe(tmp) {
                safe_reports += 1;
                break;
            }
        }
    }

    println!("Part 2 = {safe_reports}");

    safe_reports
}

fn main() {
    let contents = fs::read_to_string("./input2.txt")
                                .expect("Should have been able to read the file");

    let mut values: Vec<Vec<i8>> = Vec::new();

    for line in contents.lines() {
        let ns = line.split_whitespace();
        let mut tmp_vec: Vec<i8> = Vec::new();
        for val in ns {
            tmp_vec.push(val.parse::<i8>().unwrap());
        }
        values.push(tmp_vec);
    }

    let (mut safe_reports, unsafe_reports) = part_one(values.clone());

    println!("First safe_reports = {safe_reports}");

    safe_reports += part_two(unsafe_reports);

    println!("Final safe_reports = {safe_reports}");
}

// SHITTY FUNCTION DEVELOPED WITH A SICK BRAIN
// fn part_one(values: Vec<Vec<i8>>) -> (u32, Vec<Vec<i8>>){
//     let mut safe_reports: u32 = 0;
//     let mut is_safe;
//     let mut ret: Vec<Vec<i8>> = Vec::new();

//     for line in values.into_iter() {
//         let mut iter = line.iter();
//         let mut tmp = iter.next();

//         is_safe = true;

//         let mut line_state = String::from("define");

//         while let Some(val) = tmp {
//             tmp = iter.next();
//             if let Some(next_val) = tmp {
//                 if line_state == "define" {
//                     if val > next_val {
//                         line_state = String::from("decreasing");
//                     } else if val < next_val {
//                         line_state = String::from("increasing");
//                     } else {
//                         is_safe = false;
//                         break;
//                     }
//                 }
//                 if val > next_val && ((val - next_val) < 1 || (val - next_val) > 3) {
//                     is_safe = false;
//                     break;
//                 } else if val <= next_val && ( (next_val - val) < 1 || (next_val - val) > 3 ) {
//                     is_safe = false;
//                     break ;
//                 }

//                 if val > next_val && line_state == "increasing" {
//                     is_safe = false;
//                     break ;
//                 } else if val < next_val && line_state == "decreasing" {
//                     is_safe = false;
//                     break ;
//                 }
//             }
//         }

//         if is_safe {
//             safe_reports += 1;
//         } else {
//             ret.push(line.clone());
//         }
//     }

//     (safe_reports, ret)
// }
