use std::{clone, collections::BTreeMap, fs};

#[derive(Debug, PartialEq, Eq)]
enum TokenType {
    Literal,
    Comma,
    DoublePoint,
    Space,
    NewLine,
}

#[derive(Debug, PartialEq, Eq)]
struct Token {
    token_type: TokenType,
    value: String,
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum Operand {
    AND,
    OR,
    XOR,
}

impl Operand {
    fn to_operand(operand_type: &str) -> Self {
        match operand_type {
            "AND" => Operand::AND,
            "OR" => Operand::OR,
            "XOR" => Operand::XOR,
            _ => unreachable!(),
        }
    }
}
#[derive(Debug, PartialEq, Eq, Clone)]
struct Instruction {
    register_a: String,
    operand: Operand,
    register_b: String,
    register_out: String,
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
    let mut computer = parse(tokens);
    let ans = calculate(computer.1, &mut computer.0);

    println!("{:?}", ans);
    Ok(())
}

fn read_input() -> Result<String, std::io::Error> {
    let input = fs::read_to_string("input.txt")?;
    Ok(input)
}

fn parse(tokens: Vec<Token>) -> (BTreeMap<String, bool>, Vec<Instruction>) {
    let mut registers = BTreeMap::new();
    let mut program = Vec::new();

    let mut program_definition_begin = 0;

    for (i, token) in tokens.iter().enumerate().step_by(5) {
        if token.value == "\r" && tokens.get(i + 1).unwrap().token_type == TokenType::NewLine {
            program_definition_begin = i + 3;
            break;
        }
        registers.insert(
            token.value.clone(),
            tokens.get(i + 3).unwrap().value.trim().parse::<u8>() != Ok(0),
        );
    }

    for (i, token) in tokens
        .iter()
        .enumerate()
        .skip(program_definition_begin - 1)
        .step_by(10)
    {
        let instruction = Instruction {
            register_a: token.value.clone(),
            operand: Operand::to_operand(&tokens.get(i + 2).unwrap().value),
            register_b: tokens.get(i + 4).unwrap().value.clone(),
            register_out: tokens.get(i + 8).unwrap().value.clone().trim().to_string(),
        };
        program.push(instruction);
    }

    (registers, program)
}

fn tokenize(input: &String) -> Result<Vec<Token>, std::io::Error> {
    let mut tokens: Vec<Token> = vec![];

    let mut last_literal_index = 0;

    for (i, c) in input.chars().enumerate() {
        match c {
            ',' => {
                if last_literal_index < i {
                    tokens.push(Token::new(
                        TokenType::Literal,
                        input[last_literal_index..i].to_string(),
                    ));
                }
                tokens.push(Token::new(TokenType::Comma, ",".to_string()));
                last_literal_index = i + 1;
            }
            ':' => {
                if last_literal_index < i {
                    tokens.push(Token::new(
                        TokenType::Literal,
                        input[last_literal_index..i].to_string(),
                    ));
                }
                tokens.push(Token::new(TokenType::DoublePoint, ":".to_string()));
                last_literal_index = i + 1;
            }
            ' ' => {
                if last_literal_index < i {
                    tokens.push(Token::new(
                        TokenType::Literal,
                        input[last_literal_index..i].to_string(),
                    ));
                }
                tokens.push(Token::new(TokenType::Space, " ".to_string()));
                last_literal_index = i + 1;
            }
            '\n' => {
                if last_literal_index < i {
                    tokens.push(Token::new(
                        TokenType::Literal,
                        input[last_literal_index..i].to_string(),
                    ));
                }
                tokens.push(Token::new(TokenType::NewLine, "\n".to_string()));
                last_literal_index = i + 1;
            }
            _ => {}
        }
    }
    if last_literal_index != input.len() {
        tokens.push(Token::new(
            TokenType::Literal,
            input[last_literal_index..input.len()].to_string(),
        ));
    }
    Ok(tokens)
}

fn calculate(program: Vec<Instruction>, registers: &mut BTreeMap<String, bool>) -> i128 {
    println!("reg: {:?}", registers);

    let mut program = program.clone();
    let mut program_counter = 0;
    
    while program.len() != 0 {
        let instruction = program.get(program_counter).unwrap();
        if let Some(&a) = registers.get(&instruction.register_a) {
            if let Some(&b) = registers.get(&instruction.register_b) {
                let out = match instruction.operand {
                    Operand::AND => a && b,
                    Operand::OR => a || b,
                    Operand::XOR => a ^ b,
                };
        
                registers.insert(instruction.register_out.clone(), out);
                program.remove(program_counter);
                program_counter = 0;
            }
            else {
                program_counter += 1;
            }
        }
        else {
            program_counter += 1;
        }
    }

    println!("Registers: {:?}", registers);

    let registers: BTreeMap<String, bool> = registers
        .iter()
        .filter(|(key, _)| key.starts_with("z"))
        .map(|(key, &value)| (key.clone(), value))
        .collect();

    let mut res = 0;
    registers
        .iter()
        .enumerate()
        .for_each(|(i, (_, val))| res += (*val as i128) << i);
    res
}
