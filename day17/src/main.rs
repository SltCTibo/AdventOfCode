fn get_combo(registers: &Vec<u64>, operand: u8) -> u64 {
    match operand {
        4 => registers[0],
        5 => registers[1],
        6 => registers[2],
        _ => operand as u64,
    }
}

fn read_operation(registers: &mut Vec<u64>, instructions: &Vec<u8>, operand: &Vec<u8>) -> Vec<u8> {
    let mut pointer: usize = 0;
    let mut output: Vec<u8> = vec![];
    while pointer < instructions.len() {
        match instructions[pointer] {
            0 => {
                registers[0] =
                    registers[0] / u64::pow(2, get_combo(registers, operand[pointer]) as u32);
            }
            1 => {
                registers[1] = registers[1] ^ operand[pointer] as u64;
            }
            2 => {
                registers[1] = get_combo(registers, operand[pointer]) % 8;
            }
            3 => {
                if registers[0] == 0 {
                    pointer += 2;
                    continue;
                } else {
                    pointer = operand[pointer] as usize;
                    continue;
                }
            }
            4 => {
                registers[1] = registers[1] ^ registers[2];
            }
            5 => {
                let ch = get_combo(registers, operand[pointer]) % 8;
                output.push(ch as u8);
            }
            6 => {
                registers[1] =
                    registers[0] / u64::pow(2, get_combo(registers, operand[pointer]) as u32);
            }
            7 => {
                registers[2] =
                    registers[0] / u64::pow(2, get_combo(registers, operand[pointer]) as u32);
            }
            _ => (),
        };
        pointer += 1;
    }

    output
}

fn main() {
    let contents =
        std::fs::read_to_string("./input.txt").expect("Should have opened the input file");

    let mut registers: Vec<u64> = vec![];
    let mut instructions: Vec<u8> = vec![];
    let mut operand: Vec<u8> = vec![];
    let mut program: Vec<u8> = vec![];

    for line in contents.lines() {
        let split: Vec<&str> = line.split(" ").collect();
        if split[0] == "Register" {
            registers.push(split.last().unwrap().parse::<u64>().unwrap());
        } else if split[0] == "Program:" {
            for (i, ope) in split.last().unwrap().split(",").into_iter().enumerate() {
                if i % 2 == 0 {
                    instructions.push(ope.parse::<u8>().unwrap());
                } else {
                    operand.push(ope.parse::<u8>().unwrap());
                }
                program.push(ope.parse::<u8>().unwrap());
            }
        }
    }

    let mut res = vec![];
    let mut bases: Vec<String> = vec![String::from("0o")];
    let mut smallest = 0;

    for _ in 0..program.len() {
        let mut tmp = vec![];
        for base in &bases {
            for digit in 0..8 {
                let candidate = format!("{}{}", base, digit);

                let trimmed = candidate.trim_start_matches("0o");
                match u64::from_str_radix(trimmed, 8) {
                    Ok(value) => {
                        registers[0] = value;
                        registers[1] = 0;
                        registers[2] = 0;
                        res = read_operation(&mut registers, &instructions, &operand);

                        if program.ends_with(&res) {
                            if program.eq(&res) && smallest == 0 {
                                smallest = value;
                                println!("C'EST EGAL {value}");
                            }
                            println!("n: {value}, octal: {:#o}, res : {:?}", value, res);
                            tmp.push(candidate);
                        }
                    }
                    Err(e) => {
                        println!("Could not parse {}: {}", candidate, e);
                    }
                }
            }
        }
        bases.clear();
        bases = tmp;
    }
}
