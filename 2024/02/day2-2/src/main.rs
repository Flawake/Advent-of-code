use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let input: String = fs::read_to_string("input.txt")?;
    let data = parse_input(&input);
    //let ans = check_safe_count(data);
    let ans = check_safe_count_2(data);
    println!("{}", ans);
    Ok(())
}

fn parse_input(input: &String) -> Vec<Vec<u32>> {
    return input
        .split_terminator('\n')
        .map(|c| {
            c.split_whitespace()
                .map(|u| u.parse::<u32>().unwrap())
                .collect()
        })
        .collect();
}

fn check_safe_count(data: Vec<Vec<u32>>) -> u32 {
    let c = data.clone().into_iter()
        .filter(|line| {
            line.windows(2).all(|pair| match pair {
                [prev, curr] if curr > prev && curr - prev <= 3 => true,
                _ => false,
            })
        })
        .count() as u32;
    data.into_iter()
        .filter(|line| {
            line.windows(2).all(|pair| match pair {
                [prev, curr] if curr < prev && prev - curr <= 3 => true,
                _ => false,
            })
        })
        .count() as u32 + c
}

fn check_safe_count_2(data: Vec<Vec<u32>>) -> u32 {
    let c = data.into_iter()
    .filter(|line| {
        // Helper function to check if a sequence can be valid after one removal
        let can_be_fixed = |line: &Vec<u32>, ascending: bool| {
            let mut fail_count = 0;

            // Initial pass to count failures
            let mut valid = line.windows(2).all(|pair| match pair {
                [prev, curr] if ascending && curr > prev && curr - prev <= 3 => true,
                [prev, curr] if !ascending && curr < prev && prev - curr <= 3 => true,
                _ if fail_count < 1 => {
                    fail_count += 1;
                    false // Fail the current check, but mark a failure
                },
                _ => false, // More than one failure or invalid condition
            });

            if fail_count <= 1 {
                // Validate by simulating a single removal
                for i in 0..line.len() {
                    let mut filtered_line = line.clone();
                    filtered_line.remove(i);

                    if filtered_line.windows(2).all(|pair| match pair {
                        [prev, curr] if ascending && curr > prev && curr - prev <= 3 => true,
                        [prev, curr] if !ascending && curr < prev && prev - curr <= 3 => true,
                        _ => false,
                    }) {
                        valid = true;
                        break;
                    }
                }
            }

            valid
        };

        // Check for both ascending and descending
        can_be_fixed(line, true) || can_be_fixed(line, false)
    })
    .count() as u32;
    c
}
