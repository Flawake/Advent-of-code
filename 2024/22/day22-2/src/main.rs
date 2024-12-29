use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let input: String = fs::read_to_string("input.txt")?;
    let parsed_input = parse_input(&input);
    let ans = calculate(&parsed_input);
    println!("{}", ans);
    Ok(())
}

fn parse_input(input: &str) -> Vec<u64> {
    input
        .lines()
        .map(|str| str.parse::<u64>().unwrap())
        .collect()
}

fn calculate(secrets: &[u64]) -> u32 {
    let mut windows_map: HashMap<[i32; 4], Vec<u64>> = HashMap::new();

    secrets.iter().for_each(|&secret| {
        let mut transformed_secret = secret;
        let mut changes = Vec::new();
        let mut prev_price = None;

        for _ in 0..2000 {
            let mut result = transformed_secret * 64;
            result = mix_numbers(&transformed_secret, &result);
            result = prune_numbers(&result);
            transformed_secret = result;

            result /= 32;
            result = mix_numbers(&transformed_secret, &result);
            result = prune_numbers(&result);
            transformed_secret = result;

            result *= 2048;
            result = mix_numbers(&transformed_secret, &result);
            result = prune_numbers(&result);
            transformed_secret = result;

            let price = secret_to_price(transformed_secret) as i32;

            changes.push((price, price as i32 - prev_price.unwrap_or(-1000)));

            prev_price = Some(price);
        }

        let mut added_windows = HashSet::new();

        for window in changes.windows(4) {
            if added_windows.contains(&window.iter().map(|val| val.1).collect::<Vec<i32>>()) {
                continue;
            }
            added_windows.insert(window.iter().map(|val| val.1).collect::<Vec<i32>>());

            let window_array = [window[0].1, window[1].1, window[2].1, window[3].1];
            windows_map
                .entry(window_array)
                .or_default()
                .push(window[3].0 as u64);
        }
    });

    let mut res = 0;
    for i in -9..9 {
        for j in -9..9 {
            for k in -9..9 {
                for l in -9..9 {
                    if i + j + k + l > 9 || i + j + k + l < -9 {
                        continue;
                    }
                    if i + j + k > 18 || j + k + l > 18 || i + j + k < -18 || j + k + l < -18 {
                        continue;
                    }

                    let expected_changes = [i, j, k, l];

                    if let Some(prices_window) = windows_map.get(&expected_changes) {
                        res = res.max(prices_window.iter().fold(0, |acc, price| acc + *price));
                    }
                }
            }
        }
    }

    res as u32
}

fn mix_numbers(secret: &u64, new: &u64) -> u64 {
    secret ^ new
}

fn prune_numbers(secret: &u64) -> u64 {
    secret % 16777216
}

fn secret_to_price(secret: u64) -> u16 {
    (secret % 10) as u16
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_mix_numbers() {
        let expected = 37;
        let result = mix_numbers(&42, &15);

        assert_eq!(expected, result);
    }

    #[test]
    fn test_prune_numbers() {
        let expected = 16113920;
        let result = prune_numbers(&100000000);

        assert_eq!(expected, result);
    }

    #[test]
    fn test_calculate() {
        let expected = 7;
        let result = calculate(&vec![1]);

        assert_eq!(expected, result);

        let expected = 15322365;
        let result = calculate(&vec![17]);

        assert_eq!(expected, result);
    }

    #[test]
    fn test_secret_to_price() {
        let expected = 2;
        let result = secret_to_price(12342);

        assert_eq!(expected, result);

        let expected = 9;
        let result = secret_to_price(9);

        assert_eq!(expected, result);
    }
}
