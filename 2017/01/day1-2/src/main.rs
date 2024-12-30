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

fn parse_input(input: &str) -> Captchas {
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
            captcha.captcha.iter().enumerate().fold(0, |acc, (i, &cur_char)| {
                let next_pos = (i + captcha.captcha.len() / 2) % captcha.captcha.len();
                if cur_char == captcha.captcha[next_pos]
                {
                    return acc + (cur_char.to_digit(10).unwrap());
                }
                acc
            })
        })
        .sum()
}
