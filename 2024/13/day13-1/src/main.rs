use std::fs;

#[derive(Clone)]
struct ClawMachine {
    a_x_value: i32,
    a_y_value: i32,
    b_x_value: i32,
    b_y_value: i32,
    prize_x_value: i32,
    prize_y_value: i32,
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

            let button_a_x = tokens[i + 7].value.trim().parse::<i32>().unwrap();
            let button_a_y = tokens[i + 12].value.trim().parse::<i32>().unwrap();

            let button_b_x = tokens[i + 21].value.trim().parse::<i32>().unwrap();
            let button_b_y = tokens[i + 26].value.trim().parse::<i32>().unwrap();

            let prize_a = tokens[i + 33].value.trim().parse::<i32>().unwrap();
            let prize_b = tokens[i + 38].value.trim().parse::<i32>().unwrap();

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

fn calculate(machines: Vec<ClawMachine>) -> i32 {
    machines.iter().fold(0,|acc, machine| {
        let mut min = i32::MAX;
        for i in 0..((machine.prize_x_value / machine.a_x_value) + 1).min(100) {
            let mut total_y = 0;
            let res = machine.prize_x_value - (i * machine.a_x_value);
            if res % machine.b_x_value == 0 {
                total_y = res / machine.b_x_value;
            }

            if (i * machine.a_y_value) + (total_y * machine.b_y_value) == machine.prize_y_value {
                if (i * 3) + total_y < min {
                    min = (i * 3) + total_y;
                }
            }
        }
        if min != i32::MAX {
            acc + min
        }
        else {
            acc
        }
    })
}
