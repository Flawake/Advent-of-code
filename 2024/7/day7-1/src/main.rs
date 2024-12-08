use std::error::Error;
use std::fs;

#[derive(Debug, Clone)]
struct Sum {
    answer: u64,
    numbers: Vec<u64>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let input: String = fs::read_to_string("input.txt")?;
    let parsed_input = parse_input(&input);
    let ans = calculate(parsed_input);
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

fn calculate(input: Vec<Sum>) -> u64 {
    let sums_valid: Vec<Sum> = input
        .iter()
        .filter(|sum| {
            //Loop over the numbers for the amount of different combinations of + an * that are possible (2^n th)
            for i in 1..(1 << sum.numbers.iter().count()) {
                //Set total to the number that is present at the first index of the vector
                let mut total = sum.numbers[0];
                //Loop over the vector itself
                for vec_pos in 1..sum.numbers.iter().count() {
                    let bit = (i >> vec_pos) & 1;

                    if bit == 1 {
                        if let Some(res) = total.checked_add(sum.numbers[vec_pos]) {
                            total = res;
                        } else {
                            continue;
                        }
                    } else {
                        if let Some(res) = total.checked_mul(sum.numbers[vec_pos]) {
                            total = res;
                        } else {
                            continue;
                        }
                    }
                }
                if total == sum.answer {
                    println!("true");
                    return true;
                }
            }
            false
        })
        .cloned()
        .collect();

    let total = sums_valid.iter().map(|sum| sum.answer).sum();
    total
}
