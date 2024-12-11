use std::io::{self, BufRead};

fn main() {
    let mut input = Vec::new();
    println!("Please input the numbers, press enter when done: ");
    for line in io::stdin().lock().lines() {
        match line {
            Ok(content) => {
                if content.is_empty() {
                    //End of data, stop reading
                    break;
                }
                input.push(content);
            }
            Err(e) => {
                println!("{}", e);
            }
        }
    }
    let (row_one, row_two) = parse_input(input);

    let total_difference = calculate_total_difference(row_one, row_two);
    println!("total difference: {}", total_difference);
}

fn parse_input(input: Vec<String>) -> (Vec<i64>, Vec<i64>) {

    let (mut row_one, mut row_two) = input.iter().map(|s| {
        let slices= s.split_whitespace().collect::<Vec<&str>>();
        (slices[0].parse::<i64>().unwrap(),
        slices[1].parse::<i64>().unwrap())
    }).collect::<(Vec<i64>, Vec<i64>)>();
    
    row_one.sort();
    row_two.sort();

    (row_one, row_two)
}

fn calculate_total_difference(row_one: Vec<i64>, row_two: Vec<i64>) -> i64 {
    row_one
        .iter()
        .enumerate()
        .fold(0, |acc, (i, val)| acc + (val - row_two[i]).abs())
}
