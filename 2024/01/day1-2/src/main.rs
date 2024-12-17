use std::{io::{self, BufRead}, vec};

fn main() {
    let mut input = vec![];
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
    let (mut row_one, mut row_two) = parse_input(input);

    let similarity_score = calculate_similarity_score(&row_one, &row_two);

    row_one.sort();
    row_two.sort();

    let total_difference = calculate_total_difference(row_one, row_two);
    
    println!("similarity score: {}", similarity_score);
    println!("total difference: {}", total_difference);
}

fn parse_input(input: Vec<String>) -> (Vec<i64>, Vec<i64>){
    let mut row_one = Vec::new();
    let mut row_two = Vec::new();

    for mut data in input {
        let mut two = data.split_off(data.find(" ").unwrap());
        let one = data.trim().to_string();
        two = two.trim().to_string();

        row_one.push(one.parse::<i64>().unwrap());
        row_two.push(two.parse::<i64>().unwrap());
    }

    (row_one, row_two)
}

fn calculate_total_difference(row_one: Vec<i64>, row_two: Vec<i64>) -> i64 {
    row_one.iter().enumerate().fold(0, |acc, (i, val)| {
        acc + (val - row_two[i]).abs()
    })
}

fn calculate_similarity_score(row_one: &Vec<i64>, row_two: &Vec<i64>) -> i64 {
    row_one.iter().fold(0, |acc, &val1| {
        acc + (row_two.iter().fold(0, |acc, &val2| {
            if val1 == val2 {
             acc + 1   
            }
            else {
                acc
            }
        }) * val1)
    })
}
