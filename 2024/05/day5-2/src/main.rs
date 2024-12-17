use std::{fs, mem};

#[derive(Debug, PartialEq, Eq)]
enum TokenType {
    Literal,
    Comma,
    Pipe,
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
    let parsed_tokens = parse(tokens);
    let ans = calculate(parsed_tokens.0, parsed_tokens.1);

    println!("{:?}", ans);
    Ok(())
}

fn read_input() -> Result<String, std::io::Error> {
    let input = fs::read_to_string("input.txt")?;
    Ok(input)
}

fn parse(tokens: Vec<Token>) -> (Vec<Vec<i32>>, Vec<Vec<i32>>) {
    let mut rules = vec![];
    let mut to_print = vec![vec![]];

    let mut finding_rules = true;

    let mut vec_index = 0;

    let mut i: usize = 0;
    while i < tokens.len() {
        if finding_rules {
            if tokens[i].token_type == TokenType::Literal
            && tokens[i + 1].token_type == TokenType::Pipe
            && tokens[i + 2].token_type == TokenType::Literal
            {
                let x = tokens[i].value.trim().parse::<i32>();
                let y = tokens[i + 2].value.trim().parse::<i32>();

                if let (Ok(x_val), Ok(y_val)) = (x, y) {
                    rules.push(vec![x_val, y_val]);
                }
                else {
                    finding_rules = false;
                }

                i += 4;
            } else {
                finding_rules = false;
                i += 1;
            }
        }

        else {
            while tokens[i].token_type != TokenType::NewLine {
                if tokens[i].token_type == TokenType::Literal {
                    let x = tokens[i].value.trim().parse::<i32>();
                    if let Ok(x_val) = x {
                        to_print[vec_index].push(x_val);
                    }
                }
                i += 1;
            }
            to_print.push(vec![]);
            vec_index += 1;
            i += 1;
        }
    }
    println!("{:?}", to_print);
    (rules, to_print)
}

fn tokenize(input: &String) -> Result<Vec<Token>, std::io::Error> {
    let mut tokens: Vec<Token> = vec![];

    let mut last_literal_index = 1;

    tokens.push(Token::new(
        TokenType::Literal,
        "46".to_string(),
    ));

    for (i, c) in input.chars().enumerate() {
        match c {
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
            '|' => {
                if last_literal_index < i - 1 {
                    tokens.push(Token::new(
                        TokenType::Literal,
                        input[last_literal_index + 1..i].to_string(),
                    ));
                }
                tokens.push(Token::new(TokenType::Pipe, "|".to_string()));
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
    Ok(tokens)
}

fn calculate(x: Vec<Vec<i32>>, y: Vec<Vec<i32>>) -> i32 {
    //println!("{:?}", tokens);
    let mut result = 0;
    let c: Vec<Vec<i32>> = y.clone().into_iter().filter(|a| {
        for b in 0..a.len() {
            for r in  b..a.len() {
                for s in &x {
                    if a[b] == s[1] && a[r] == s[0]{
                        return true;
                    }
                }
            }
        }
        false
    }).collect();

    let t = order(x, c);

    for m in  t {
        if m.len() < 1 {
            continue;
        }
        result += m[m.len()/2]
    }
    result
}

fn order(x: Vec<Vec<i32>>, y: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut res: Vec<Vec<i32>> = vec![];

    let mut z = y.clone();

    for v in &mut z {
        let mut found_err = true;

        while found_err {
            found_err = false;

            for b in 0..v.len() {
                for r in b+1..v.len() {
                    for s in &x {
                        if v[b] == s[1] && v[r] == s[0] {
                            v.swap(b, r);
                            found_err = true;
                        }
                    }
                }
            }
        }

        res.push(v.clone());
    }

    res
}
