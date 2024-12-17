use std::fs;

#[derive(Debug, PartialEq, Eq)]
enum TokenType {
    Literal,
    LeftParant,
    RightParant,
    Space,
    Comma,
    Plus,
    Minus,
    Equals,
    Semicolon,
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
    let ans = calculate(tokens);

    println!("{:?}", ans);
    Ok(())
}

fn read_input() -> Result<String, std::io::Error> {
    let input = fs::read_to_string("input.txt")?;
    Ok(input)
}

fn parse_string(input: &String) -> String {
    input
        .chars()
        .into_iter()
        .map(|c| if c == '\n' || c == '\r' { ' ' } else { c })
        .collect::<String>()
}

fn tokenize(input: &String) -> Result<Vec<Token>, std::io::Error> {
    let mut tokens: Vec<Token> = vec![];

    let mut last_literal_index = 0;
    let parsed_input = parse_string(input);

    for (i, c) in parsed_input.chars().enumerate() {
        match c {
            '(' => {
                if last_literal_index < i - 1 {
                    tokens.push(Token::new(
                        TokenType::Literal,
                        parsed_input[last_literal_index + 1..i].to_string(),
                    ));
                }
                tokens.push(Token::new(TokenType::LeftParant, "(".to_string()));
                last_literal_index = i;
            }
            ')' => {
                if last_literal_index < i - 1 {
                    tokens.push(Token::new(
                        TokenType::Literal,
                        parsed_input[last_literal_index + 1..i].to_string(),
                    ));
                }
                tokens.push(Token::new(TokenType::RightParant, ")".to_string()));
                last_literal_index = i;
            }
            ' ' => {
                if last_literal_index < i - 1 {
                    tokens.push(Token::new(
                        TokenType::Literal,
                        parsed_input[last_literal_index + 1..i].to_string(),
                    ));
                }
                tokens.push(Token::new(TokenType::Space, " ".to_string()));
                last_literal_index = i;
            }
            ',' => {
                if last_literal_index < i - 1 {
                    tokens.push(Token::new(
                        TokenType::Literal,
                        parsed_input[last_literal_index + 1..i].to_string(),
                    ));
                }
                tokens.push(Token::new(TokenType::Comma, ",".to_string()));
                last_literal_index = i;
            }
            '+' => {
                if last_literal_index < i - 1 {
                    tokens.push(Token::new(
                        TokenType::Literal,
                        parsed_input[last_literal_index + 1..i].to_string(),
                    ));
                }
                tokens.push(Token::new(TokenType::Plus, "+".to_string()));
                last_literal_index = i;
            }
            '-' => {
                if last_literal_index < i - 1 {
                    tokens.push(Token::new(
                        TokenType::Literal,
                        parsed_input[last_literal_index..i].to_string(),
                    ));
                }
                tokens.push(Token::new(TokenType::Minus, "-".to_string()));
                last_literal_index = i;
            }
            '=' => {
                if last_literal_index < i - 1 {
                    tokens.push(Token::new(
                        TokenType::Literal,
                        parsed_input[last_literal_index + 1..i].to_string(),
                    ));
                }
                tokens.push(Token::new(TokenType::Equals, "=".to_string()));
                last_literal_index = i;
            }
            ';' => {
                if last_literal_index < i - 1 {
                    tokens.push(Token::new(
                        TokenType::Literal,
                        parsed_input[last_literal_index + 1..i].to_string(),
                    ));
                }
                tokens.push(Token::new(TokenType::Semicolon, ";".to_string()));
                last_literal_index = i;
            }
            _ => {}
        }
    }
    Ok(tokens)
}

fn calculate(tokens: Vec<Token>) -> i32 {
    println!("{:?}", tokens);
    let mut result = 0; // Start with 1 since we are multiplying
    let mut i = 0; // We'll use this index to loop through the vector
    let mut do_multiply = true;

    while i + 5 < tokens.len() {

        if tokens[i].value.ends_with("do")
            && tokens[i + 1].token_type == TokenType::LeftParant
            && tokens[i + 2].token_type == TokenType::RightParant
        {
            do_multiply = true;
            i += 3;
            continue;
        }

        if tokens[i].value.ends_with("don't")
            && tokens[i + 1].token_type == TokenType::LeftParant
            && tokens[i + 2].token_type == TokenType::RightParant
        {
            do_multiply = false;
            i += 3;
            continue;
        }

        if !do_multiply {
            i += 1;
            continue;
        }

        if tokens[i].value.ends_with("mul")
            && tokens[i + 1].token_type == TokenType::LeftParant
            && tokens[i + 2].token_type == TokenType::Literal
            && tokens[i + 3].token_type == TokenType::Comma
            && tokens[i + 4].token_type == TokenType::Literal
            && tokens[i + 5].token_type == TokenType::RightParant
        {
            let x = tokens[i + 2].value.parse::<i32>();
            let y = tokens[i + 4].value.parse::<i32>();

            if let (Ok(x_val), Ok(y_val)) = (x, y) {
                result += x_val * y_val;
            }

            i += 6;
        } else {
            i += 1;
        }
    }

    result
}
