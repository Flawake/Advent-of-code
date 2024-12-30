use std::error::Error;
use std::fs;

struct Captcha {
    captcha: Vec<char>,
}

struct Captchas {
    captchas: Vec<Captcha>,
}

impl FromIterator<char> for Captcha {
    fn from_iter<I: IntoIterator<Item = char>>(iter: I) -> Self {
        Captcha {
            captcha: iter.into_iter().collect(),
        }
    }
}

impl FromIterator<Captcha> for Captchas {
    fn from_iter<I: IntoIterator<Item = Captcha>>(iter: I) -> Self {
        Captchas {
            captchas: iter.into_iter().collect(),
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let input: String = fs::read_to_string("input.txt")?;
    let parsed_input = parse_input(&input);
    let ans = calculate(&parsed_input);
    println!("{}", ans);
    Ok(())
}

fn parse_input(input: &String) -> Captchas {
    input
        .lines()
        .map(|input| input.trim().chars().collect())
        .collect()
}

fn calculate(input: &Captchas) -> u32 {
    input
        .captchas
        .iter()
        .map(|captcha| {
            let sum = captcha.captcha.windows(2).fold(0, |acc, window| {
                if let (Some(first), Some(next)) = (window[0].to_digit(10), window[1].to_digit(10))
                {
                    if first == next {
                        return acc + (first);
                    }
                }
                acc
            });
            if let (Some(first), Some(last)) = (captcha.captcha.first().unwrap().to_digit(10), captcha.captcha.last().unwrap().to_digit(10)) {
                if first == last {
                    return sum + first;
                }
            }
            sum
        })
        .sum()
}
