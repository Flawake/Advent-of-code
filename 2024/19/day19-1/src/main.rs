use std::collections::HashMap;
use std::error::Error;
use std::fs;

struct TowelParts {
    parts: Vec<String>,
}

struct Towels {
    towel: Vec<String>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let input: String = fs::read_to_string("input.txt")?;
    let parsed_input = parse_input(&input);
    let ans = calculate(parsed_input.0, parsed_input.1);
    println!("{}", ans);
    Ok(())
}

fn parse_input(input: &String) -> (Towels, TowelParts) {

    let mut towel_parts = Vec::new();
    let mut towels = Vec::new();

    input.lines().into_iter().enumerate().for_each(|(i, line)| {
        if i == 0 {
            towel_parts = line.split(",").map(|str|str.trim().to_string()).collect();
        }
        else {
            if line != "" {
                towels.push(line.to_string());
            }
        }
    });

    println!("Towel parts: {:?}", &towel_parts);
    println!("Towels: {:?}", &towels);
    (Towels {towel: towels}, TowelParts {parts: towel_parts })
}

fn calculate(towels: Towels, parts: TowelParts) -> u32 {
    let mut found: HashMap<String, bool> = HashMap::new();
    towels.towel.iter().fold(0, |acc, towel| if can_construct_towel(towel, &parts, &mut found) {
        acc + 1
    } else {
        acc
    })
}

fn can_construct_towel(towel: &String, parts: &TowelParts, found: &mut HashMap<String, bool>) -> bool {
    if found.contains_key(towel) {
        return *found.get(towel).unwrap();
    }
    let res = parts.parts.iter().any(|part| {
        if towel.starts_with(part) {
            let new_towel = towel[part.len()..].to_string();
            if new_towel.is_empty() {
                return true;
            } else {
                return can_construct_towel(&new_towel, parts, found);
            }
        }
        false
    });
    found.insert(towel.clone(), res);
    res
}