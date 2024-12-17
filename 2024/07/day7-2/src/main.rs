use std::error::Error;
use std::fs;
use futures::future::join_all;
use tokio;

#[derive(Debug, Clone)]
struct Sum {
    answer: u64,
    numbers: Vec<u64>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let input: String = fs::read_to_string("input.txt")?;
    let parsed_input = parse_input(&input);
    let ans = calculate(parsed_input).await;
    println!("{}", ans);
    Ok(())
}

fn parse_input(input: &String) -> Vec<Sum> {
    let sums = input
        .lines()
        .map(|str| {
            let split = str.split(':').collect::<Vec<&str>>();
            let numbers = split[1]
                .trim()
                .split(' ')
                .into_iter()
                .map(|n| n.parse::<u64>().unwrap())
                .collect();

            Sum {
                answer: split[0].parse::<u64>().unwrap(),
                numbers: numbers,
            }
        })
        .collect::<Vec<Sum>>();

    sums
}


async fn calculate(input: Vec<Sum>) -> u64 {
    let checks = input.into_iter().map(|sum| {
        tokio::spawn(async move {
            if check_valid(&sum).await {
                Some(sum)
            } else {
                None
            }
        })
    });

    let results = join_all(checks).await;

    let sums_valid: Vec<Sum> = results
        .into_iter()
        .filter_map(|res| res.ok().flatten())
        .collect();

    let total: u64 = sums_valid.iter().map(|sum| sum.answer).sum();
    total
}

async fn check_valid(sum: &Sum) -> bool {
    println!("loop");
    //Loop over the numbers for the amount of different combinations of + an * that are possible (2^n th)
    for i in 1..3_i64.pow(sum.numbers.len() as u32) {
        let base3_num: String = dec_to_base_3(i as u32);
        //Set total to the number that is present at the first index of the vector
        let mut total = sum.numbers[0];
        //Loop over the vector itself
        for vec_pos in 1..sum.numbers.len() {
            match base3_num.chars().nth(vec_pos) {
                Some('0') => {
                    if let Some(res) = total.checked_add(sum.numbers[vec_pos]) {
                        total = res;
                    } else {
                        break;
                    }
                },
                Some('1') => {
                    if let Some(res) = total.checked_mul(sum.numbers[vec_pos]) {
                        total = res;
                    } else {
                        break;
                    }
                },
                _ => {
                    total = format!("{}{}", total, sum.numbers[vec_pos])
                    .parse::<u64>()
                    .unwrap();
                },
            }

            if total > sum.answer {
                continue;
            }
        }
        if total == sum.answer {
            return true;
        }
    }
    false
}

fn dec_to_base_3(mut num: u32) -> String {
    let mut result = String::new();
    while num > 0 {
        let remainder = num % 3;
        result.push_str(&remainder.to_string());
        num /= 3;
    }
    result.chars().rev().collect()
}
