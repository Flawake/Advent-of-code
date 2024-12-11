use std::collections::HashMap;
use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let input: String = fs::read_to_string("input.txt")?;
    let parsed_input = parse_input(&input);
    let ans = calculate(parsed_input);
    println!("{}", ans);
    Ok(())
}

fn parse_input(input: &String) -> Vec<u64> {
    input
        .split(' ')
        .into_iter()
        .map(|s| s.parse::<u64>().unwrap())
        .collect()
}

fn calculate(input: Vec<u64>) -> u64 {
    let mut total = 0;

    for i in input.iter() {
        total += split_stone(*i as usize, 75, &mut HashMap::new());
    }

    total as u64
}

fn split_stone(num: usize, iters: usize, mem: &mut HashMap<(usize, usize), usize>) -> usize {
    if iters == 0 {
        return 1;
    }
    if let Some(&val) = mem.get(&(num, iters)) {
        return val;
    }

    match num {
        0 => {
            let num_stones = split_stone(1, iters - 1, mem);
            mem.insert((num, iters), num_stones);
            num_stones
        }
        _ if num.to_string().len() % 2 == 0 => {
            let num_string = num.to_string();
            let num1 = num_string[0..num_string.len() / 2].parse::<u64>().unwrap() as usize;
            let num2 = num_string[num_string.len() / 2..num_string.len()]
                .parse::<u64>()
                .unwrap() as usize;
            let num_stones = split_stone(num1, iters - 1, mem) + split_stone(num2, iters - 1, mem);
            mem.insert((num, iters), num_stones);
            num_stones
        }
        _ => {
            let num1 = num * 2024;
            let num_stones = split_stone(num1, iters - 1, mem);
            mem.insert((num, iters), num_stones);
            num_stones
        }
    }
}
