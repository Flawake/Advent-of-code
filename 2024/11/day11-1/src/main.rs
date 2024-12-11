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
    input.split(' ').into_iter().map(|s| s.parse::<u64>().unwrap()).collect()
}

fn calculate(input: Vec<u64>) -> u64 {
    println!("{:?}", input);
    let mut input = input;
    
    for _ in 0..25 {
        input = input.iter().flat_map(|stone| {
            if let (res, true) = replace_0(stone) {
                return res;
            }
            else if let (res, true) = even_and_split(stone) {
                return res;
            }
            else {
                return multiply_2024(stone);
            }
        }).collect();
    }

    input.len() as u64
}

fn replace_0(input: &u64) -> (Vec<u64>, bool) {
    if input == &0 {
        return (vec![1], true);
    }
    return (vec![*input], false);
}

fn even_and_split(input: &u64) -> (Vec<u64>, bool) {
    let input_str = input.to_string();
    let mut res = vec![];
    let input_len = input.to_string().len();
    if input_len % 2 == 0 {
        res.push(input_str[0..input_len / 2].parse::<u64>().unwrap());
        res.push(input_str[input_len / 2..input_len].parse::<u64>().unwrap());
        return (res, true);
    }
    res.push(*input);
    return (res, false);
}

fn multiply_2024(input: &u64) -> Vec<u64> {
    return vec![input * 2024];
}
