fn solve_equation(target: &u128, current_value: &u128, values: &[u128], res: &mut Vec<(u128, u128)>, branch_nb: u128) {
    if values.is_empty() {
        if current_value == target {
            if !res.contains(&(branch_nb, *target)) {
                res.push((branch_nb, *target));
            }
        }
        return
    }
    let (value, remaining_values) = values.split_first().unwrap();
    if let Some(new_value) = current_value.checked_add(*value) {
        solve_equation(target, &new_value, remaining_values, res, branch_nb);
    }
    if let Some(new_value) = current_value.checked_mul(*value) {
        solve_equation(target, &new_value, remaining_values, res, branch_nb);
    }
    if let Ok(concatenated) = format!("{}{}", current_value, value).parse::<u128>() {
        solve_equation(target, &concatenated, remaining_values, res, branch_nb);
    }
}

fn main() {
    let contents = std::fs::read_to_string("./input.txt").expect("Should have been reading the file");

    let mut equations: Vec<(u128, Vec<u128>)> = Vec::new();

    for line in contents.lines() {
        let parts: Vec<&str> = line.split(":").collect();
        if let [key, values] = &parts[..] {
            let key = key.parse::<u128>().expect("Invalid key format");
            let values: Vec<u128> = values.split_whitespace().map(|v| v.parse::<u128>().unwrap()).collect();

            equations.push((key, values));
        }
    }

    let mut res: Vec<(u128, u128)> = Vec::new();

    let mut line_nb = 0;
    for (key, values) in &mut equations {
        let (starting_value, remaining_values) = values.split_first().unwrap();
        solve_equation(key, &starting_value, remaining_values, &mut res, line_nb);
        line_nb += 1;
    }

    let res: u128 = res.iter().map(|&(_, second)| second).sum();

    println!("res = {res}");
}
