use std::fs;

#[derive(Debug, PartialEq, Eq)]
enum TokenType {
    Literal,
    Comma,
    DoublePoint,
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

#[derive(Debug)]
struct Register {
    reg_a: u32,
    reg_b: u32,
    reg_c: u32,
}

#[derive(Debug, Clone, Copy)]
struct Instruction {
    opcode: u8,
    operand: u8,
}

fn main() -> Result<(), std::io::Error> {
    let input = read_input()?;
    let tokens = tokenize(&input)?;
    let mut computer = parse(tokens);
    let ans = calculate(&mut computer.0, computer.1);

    println!("{:?}", ans);
    Ok(())
}

fn read_input() -> Result<String, std::io::Error> {
    let input = fs::read_to_string("input.txt")?;
    Ok(input)
}

fn parse(tokens: Vec<Token>) -> (Register, Vec<Instruction>) {
    let registers = Register {
        reg_a: tokens[2].value.trim().parse::<u32>().unwrap(),
        reg_b: tokens[6].value.trim().parse::<u32>().unwrap(),
        reg_c: tokens[10].value.trim().parse::<u32>().unwrap(),
    };
    
    let program_entry = tokens.iter().enumerate().find_map(|(i, token)| {
        if token.token_type == TokenType::Literal && token.value == "Program" {
            return Some(i);
        }
        None
    }).unwrap();

    let mut program = Vec::new();
    for i in (program_entry + 2..tokens.len()).step_by(4) {
        if tokens[i].token_type == TokenType::Literal {
            let instruction = Instruction {
                opcode: tokens[i].value.trim().parse::<u8>().unwrap(),
                operand: tokens[i + 2].value.trim().parse::<u8>().unwrap(),
            };
            println!("{:?}", instruction);
            program.push(instruction);
        }
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

fn calculate(registers: &mut Register, program: Vec<Instruction>) -> i32 {
    let mut pc = 0;
    loop {
        let instruction = program[pc].clone();
        match instruction.opcode {
            0 => {
                adv(registers, instruction.operand);
                pc += 1;
            },
            1 => {
                bxl(registers, instruction.operand);
                pc += 1;
            }
            2 => {
                bst(registers, instruction.operand);
                pc += 1;
            }
            3 => {
                pc = jnz(registers, instruction.operand, pc);
            }
            4 => {
                bxc(registers, instruction.operand);
                pc += 1;
            }
            5 => {
                out(registers, instruction.operand);
                pc += 1;
            }
            6 => {
                bdv(registers, instruction.operand);
                pc += 1;
            }
            7 => {
                cdv(registers, instruction.operand);
                pc += 1;
            }
            _ => unreachable!(),
        };
        if pc == program.len() {
            return 0;
        }
    }
}

fn adv(registers: &mut Register, operand: u8) {
   registers.reg_a = registers.reg_a / 2_u32.pow(get_combo_operand(&registers, operand));
}

fn bxl(registers: &mut Register, operand: u8) {
    registers.reg_b = registers.reg_b ^ operand as u32
}

fn bst(registers: &mut Register, operand: u8) {
    registers.reg_b = get_combo_operand(registers, operand) % 8;
}

fn jnz(registers: &mut Register, operand: u8, pc: usize) -> usize {
    if registers.reg_a == 0 {
        return pc;
    }
    return operand as usize;
}

fn bxc(registers: &mut Register, _operand: u8) {
    registers.reg_b = registers.reg_b ^ registers.reg_c;
}

fn out(registers: &mut Register, operand: u8) {
    println!("{},", get_combo_operand(registers, operand));
}

fn bdv(registers: &mut Register, operand: u8) {
    registers.reg_b = registers.reg_a / 2_u32.pow(get_combo_operand(&registers, operand));
}

fn cdv(registers: &mut Register, operand: u8) {
    registers.reg_c = registers.reg_a / 2_u32.pow(get_combo_operand(&registers, operand));
}

fn get_combo_operand(registers: &Register, operand: u8) -> u32 {
    match operand {
        0..=3  => operand as u32,
        4 => registers.reg_a,
        5 => registers.reg_b,
        6 => registers.reg_c,
        _ => unreachable!(),
    }
}
