use core::fmt;
use std::{collections::HashMap, fs};

use petgraph::{dot::{Config, Dot}, graph::DiGraph};


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

impl fmt::Display for Operand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let symbol = match self {
            Operand::AND => "&",
            Operand::OR => "|",
            Operand::XOR => "^",
        };
        write!(f, "{}", symbol)
    }
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

    fn to_string(&self) -> String {
        match self {
            Operand::AND => "AND".to_owned(),
            Operand::OR => "OR".to_owned(),
            Operand::XOR => "XOR".to_owned(),
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
    let computer = parse(tokens);
    calculate(computer);
    Ok(())
}

fn read_input() -> Result<String, std::io::Error> {
    let input = fs::read_to_string("input.txt")?;
    Ok(input)
}

fn parse(tokens: Vec<Token>) -> Vec<Instruction> {
    let mut program = Vec::new();

    let mut program_definition_begin = 0;

    for (i, token) in tokens.iter().enumerate().step_by(5) {
        if token.value == "\r" && tokens.get(i + 1).unwrap().token_type == TokenType::NewLine {
            program_definition_begin = i + 3;
            break;
        }
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

    program
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

//Generated a .DOT file and just searched for the 8 pairs manually
fn calculate(program: Vec<Instruction>) {
    let mut graph = DiGraph::<String, String>::new();
    let mut node_map = HashMap::new();

    for (i, instruction) in program.iter().enumerate() {
        let operand = &(instruction.operand.clone().to_string() + &i.to_string());
        let nodes = vec![
            &instruction.register_a,
            &instruction.register_b,
            operand,
            &instruction.register_out,
        ];

        for reg in nodes {
            if !node_map.contains_key(reg) {
                let node_index = graph.add_node(reg.clone());
                node_map.insert(reg.clone(), node_index);
            }
        }

        let a_idx = node_map[&instruction.register_a];
        let b_idx = node_map[&instruction.register_b];
        let gate = node_map[&(instruction.operand.clone().to_string() + &i.to_string())];
        let out_idx = node_map[&instruction.register_out];

        graph.add_edge(a_idx, gate, instruction.operand.to_string().clone());
        graph.add_edge(b_idx, gate, instruction.operand.to_string().clone());
        graph.add_edge(gate, out_idx, instruction.operand.to_string().clone());
    }

    let dot_graph = Dot::with_config(&graph, &[Config::EdgeNoLabel]);
    println!("{}", dot_graph);

    std::fs::write("graph.dot", format!("{}", dot_graph)).expect("Unable to write file");

    println!("DOT file saved as 'graph.dot'. Use Graphviz or an online viewer to render.");
}
