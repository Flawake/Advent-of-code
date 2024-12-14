use std::{fs, vec};
use nalgebra::{Matrix2, Vector2};
use std::cmp::min;

#[derive(Clone)]
struct ClawMachine {
    a_x_value: i64,
    a_y_value: i64,
    b_x_value: i64,
    b_y_value: i64,
    prize_x_value: i64,
    prize_y_value: i64,
}

#[derive(Debug, PartialEq, Eq)]
enum TokenType {
    Literal,
    Diactric,
    Plus,
    Equal,
    Comma,
    Space,
    NewLine,
}

#[derive(Debug, PartialEq, Eq)]
struct Token {
    token_type: TokenType,
    value: String,
}

impl Token {
    fn new(token_type: TokenType, val: String) -> Self {
        Token {
            token_type,
            value: val,
        }
    }
}

fn main() -> Result<(), std::io::Error> {
    let input = read_input()?;
    let tokens = tokenize(&input)?;
    let claw_machines = parse(tokens);
    let ans = calculate(claw_machines);

    println!("{:?}", ans);
    Ok(())
}

fn read_input() -> Result<String, std::io::Error> {
    let input = fs::read_to_string("input.txt")?;
    Ok(input)
}

fn parse(tokens: Vec<Token>) -> Vec<ClawMachine> {
    let mut machines = vec![];

    let mut i: usize = 0;
    while i < tokens.len() - 37 {
        if tokens[i + 7].token_type == TokenType::Literal
            && tokens[i + 12].token_type == TokenType::Literal
            && tokens[i + 21].token_type == TokenType::Literal
            && tokens[i + 26].token_type == TokenType::Literal
        {

            let button_a_x = tokens[i + 7].value.trim().parse::<i64>().unwrap();
            let button_a_y = tokens[i + 12].value.trim().parse::<i64>().unwrap();

            let button_b_x = tokens[i + 21].value.trim().parse::<i64>().unwrap();
            let button_b_y = tokens[i + 26].value.trim().parse::<i64>().unwrap();

            let prize_a = tokens[i + 33].value.trim().parse::<i64>().unwrap() + 10000000000000;
            let prize_b = tokens[i + 38].value.trim().parse::<i64>().unwrap() + 10000000000000;

            machines.push(ClawMachine {
                a_x_value: button_a_x,
                a_y_value: button_a_y,
                b_x_value: button_b_x,
                b_y_value: button_b_y,
                prize_x_value: prize_a,
                prize_y_value: prize_b,
            });

            i += 41;
        } else {
            i += 1;
        }
    }
    machines
}

fn tokenize(input: &String) -> Result<Vec<Token>, std::io::Error> {
    let mut tokens: Vec<Token> = vec![];

    let mut last_literal_index = 0;

    for (i, c) in input.chars().enumerate() {
        match c {
            ':' => {
                if last_literal_index < i - 1 {
                    tokens.push(Token::new(
                        TokenType::Literal,
                        input[last_literal_index + 1..i].to_string(),
                    ));
                }
                tokens.push(Token::new(TokenType::Diactric, ":".to_string()));
                last_literal_index = i;
            }
            '+' => {
                if last_literal_index < i - 1 {
                    tokens.push(Token::new(
                        TokenType::Literal,
                        input[last_literal_index + 1..i].to_string(),
                    ));
                }
                tokens.push(Token::new(TokenType::Plus, "+".to_string()));
                last_literal_index = i;
            }
            '=' => {
                if last_literal_index < i - 1 {
                    tokens.push(Token::new(
                        TokenType::Literal,
                        input[last_literal_index + 1..i].to_string(),
                    ));
                }
                tokens.push(Token::new(TokenType::Equal, "=".to_string()));
                last_literal_index = i;
            }
            ',' => {
                if last_literal_index < i - 1 {
                    tokens.push(Token::new(
                        TokenType::Literal,
                        input[last_literal_index + 1..i].to_string(),
                    ));
                }
                tokens.push(Token::new(TokenType::Comma, ",".to_string()));
                last_literal_index = i;
            }
            ' ' => {
                if last_literal_index < i - 1 {
                    tokens.push(Token::new(
                        TokenType::Literal,
                        input[last_literal_index + 1..i].to_string(),
                    ));
                }
                tokens.push(Token::new(TokenType::Space, " ".to_string()));
                last_literal_index = i;
            }
            '\n' => {
                if last_literal_index < i - 1 {
                    tokens.push(Token::new(
                        TokenType::Literal,
                        input[last_literal_index + 1..i].to_string(),
                    ));
                }
                tokens.push(Token::new(TokenType::NewLine, "\n".to_string()));
                last_literal_index = i;
            }
            _ => {}
        }
    }
    tokens.push(Token::new(
        TokenType::Literal,
        input[last_literal_index + 1..input.len()].to_string(),
    ));
    Ok(tokens)
}

fn calculate(machines: Vec<ClawMachine>) -> i64 {
    machines.iter().fold(0i64,|acc, machine| {
        acc + find_min_cost((machine.a_x_value, machine.a_y_value), (machine.b_x_value, machine.b_y_value), (machine.prize_x_value, machine.prize_y_value))
    })
}



fn find_min_cost(a: (i64, i64), b: (i64, i64), t: (i64, i64)) -> i64 {
    if (t.0 * b.1 - t.1 * b.0) % (a.0 * b.1 - b.0 * a.1) != 0 {
        return 0;
    }
    let fa = (t.0 * b.1 - t.1 * b.0) / (a.0 * b.1 - b.0 * a.1);

    if (t.0 - a.0 * fa) % b.0 != 0 {
        return 0;
    }
    let fb = (t.0 - a.0 * fa) / b.0;
    return fa * 3 + fb;
}
