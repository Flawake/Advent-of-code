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

#[derive(Debug, Clone, Copy)]
struct Register {
    reg_a: u64,
    reg_b: u64,
    reg_c: u64,
}

#[derive(Debug, Clone, Copy)]
struct Instruction {
    opcode: u8,
    operand: u8,
}

fn main() -> Result<(), std::io::Error> {
    let input = read_input()?;
    let tokens = tokenize(&input)?;
    let computer = parse(tokens);
    let ans = calculate(computer.0, computer.1);

    println!("{}", ans);
    Ok(())
}

fn read_input() -> Result<String, std::io::Error> {
    let input = fs::read_to_string("input.txt")?;
    Ok(input)
}

fn parse(tokens: Vec<Token>) -> (Register, Vec<Instruction>) {
    let registers = Register {
        reg_a: tokens[2].value.trim().parse::<u64>().unwrap(),
        reg_b: tokens[6].value.trim().parse::<u64>().unwrap(),
        reg_c: tokens[10].value.trim().parse::<u64>().unwrap(),
    };

    let program_entry = tokens
        .iter()
        .enumerate()
        .find_map(|(i, token)| {
            if token.token_type == TokenType::Literal && token.value == "Program" {
                return Some(i);
            }
            None
        })
        .unwrap();

    let mut program = Vec::new();
    for i in (program_entry + 2..tokens.len()).step_by(4) {
        if tokens[i].token_type == TokenType::Literal {
            let instruction = Instruction {
                opcode: tokens[i].value.trim().parse::<u8>().unwrap(),
                operand: tokens[i + 2].value.trim().parse::<u8>().unwrap(),
            };
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

fn calculate(registers_original: Register, program: Vec<Instruction>) -> u64 {
    let ans = String::from("2417751703415530");
    let mut output = String::new();
    let mut input = vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];

    loop {
        let mut registers = registers_original.clone();
        registers.reg_a = vec_to_int(&input);
        let mut pc = 0;
        loop {
            let instruction = program[pc].clone();
            match instruction.opcode {
                0 => {
                    adv(&mut registers, instruction.operand);
                    pc += 1;
                }
                1 => {
                    bxl(&mut registers, instruction.operand);
                    pc += 1;
                }
                2 => {
                    bst(&mut registers, instruction.operand);
                    pc += 1;
                }
                3 => {
                    pc = jnz(&mut registers, instruction.operand, pc);
                }
                4 => {
                    bxc(&mut registers, instruction.operand);
                    pc += 1;
                }
                5 => {
                    out(&mut registers, instruction.operand, &mut output);
                    pc += 1;
                }
                6 => {
                    bdv(&mut registers, instruction.operand);
                    pc += 1;
                }
                7 => {
                    cdv(&mut registers, instruction.operand);
                    pc += 1;
                }
                _ => unreachable!(),
            };
            if pc >= program.len() {
                break;
            }
        }
        output = if output.len() < 16 {
            ("0".repeat(16 - output.len())) + &output
        } else {
            output
        };

        if output == ans {
            return vec_to_int(&input);
        }
        for i in (0..output.len()).rev() {
            if output.chars().nth(i) != ans.chars().nth(i) {
                let mut j = i;
                while input[j] == 7 {
                    input[j] = 0;
                    j += 1;
                }
                input[j] = input[j] + 1;
                break;
            }
        }
        output.clear();
    }
}

fn adv(registers: &mut Register, operand: u8) {
    registers.reg_a = registers.reg_a / 2_u64.pow(get_combo_operand(&registers, operand) as u32);
}

fn bxl(registers: &mut Register, operand: u8) {
    registers.reg_b = registers.reg_b ^ operand as u64
}

fn bst(registers: &mut Register, operand: u8) {
    registers.reg_b = get_combo_operand(registers, operand) % 8;
}

fn jnz(registers: &mut Register, operand: u8, pc: usize) -> usize {
    if registers.reg_a == 0 {
        return pc + 1;
    }
    return (operand / 2) as usize;
}

fn bxc(registers: &mut Register, _operand: u8) {
    registers.reg_b = registers.reg_b ^ registers.reg_c;
}

fn out(registers: &mut Register, operand: u8, output: &mut String) {
    output.push_str(&(get_combo_operand(registers, operand) % 8).to_string());
}

fn bdv(registers: &mut Register, operand: u8) {
    registers.reg_b = registers.reg_a / 2_u64.pow(get_combo_operand(&registers, operand) as u32);
}

fn cdv(registers: &mut Register, operand: u8) {
    registers.reg_c = registers.reg_a / 2_u64.pow(get_combo_operand(&registers, operand) as u32);
}

fn get_combo_operand(registers: &Register, operand: u8) -> u64 {
    match operand {
        0..=3 => operand as u64,
        4 => registers.reg_a,
        5 => registers.reg_b,
        6 => registers.reg_c,
        _ => unreachable!(),
    }
}

fn vec_to_int(input: &Vec<u8>) -> u64 {
    let mut num = 0;

    for (i, &value) in input.iter().enumerate() {
        if value > 8 {
            unreachable!()
        }
        num |= (value as u64) << (i * 3);
    }

    num
}
