use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let input: String = fs::read_to_string("input.txt")?;
    let parsed_input = parse_input(&input);
    let ans = calculate(&parsed_input);
    println!("{}", ans);
    Ok(())
}

fn parse_input(input: &String) -> Vec<u64> {
    input.lines().map(|str|str.parse::<u64>().unwrap()).collect()
}

fn calculate(secrets: &Vec<u64>) -> u64 {
    secrets.iter().fold(0, |acc, secret| {
        let mut secret = *secret;
        for _ in 0..2000 {
            let mut result = secret * 64;
            result = mix_numbers(&secret, &result);
            result = prune_numbers(&result);
            secret = result;

            result = result / 32;
            result = mix_numbers(&secret, &result);
            result = prune_numbers(&result);
            secret = result;

            result = result * 2048;
            result = mix_numbers(&secret, &result);
            result = prune_numbers(&result);
            secret = result;
        }
        acc + secret
    })
}

fn mix_numbers(secret: &u64, new: &u64) -> u64 {
    secret ^ new
}

fn prune_numbers(secret: &u64) -> u64 {
    secret % 16777216
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

        let expected = 4915986;
        let result = calculate(&vec![12]);

        assert_eq!(expected, result);

        let expected = 15322365;
        let result = calculate(&vec![17]);

        assert_eq!(expected, result);
    }
}