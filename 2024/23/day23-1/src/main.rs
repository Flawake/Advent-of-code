use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs;

#[derive(Debug, PartialEq, Eq, Clone)]
struct Connection {
    comp1: String,
    comp2: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let input: String = fs::read_to_string("input.txt")?;
    let parsed_input = parse_input(&input);
    let ans = calculate(&parsed_input);
    println!("{}", ans);
    Ok(())
}

fn parse_input(input: &String) -> Vec<Connection> {
    input
        .lines()
        .into_iter()
        .filter(|line| line.contains("-"))
        .map(|line| {
            let parts: Vec<&str> = line.split('-').collect();
            Connection {
                comp1: parts.get(0).unwrap().to_string(),
                comp2: parts.get(1).unwrap().to_string(),
            }
        })
        .collect()
}

fn calculate(input: &Vec<Connection>) -> u32 {
    let pairs = pre_calculate_pairs(input);

    let mut networks = HashSet::new();

    for (computer, neighbors) in &pairs {
        for neighbor_1 in neighbors {
            if let Some(neighbors_2) = pairs.get(neighbor_1) {
                for neighbor_2 in neighbors_2 {
                    if pairs.get(neighbor_2).unwrap().contains(computer) {
                        let mut triple = vec![computer, neighbor_1, neighbor_2];
                        triple.sort();

                        if triple.iter().any(|&n| n.starts_with('t')) {
                            networks.insert(triple);
                        }
                    }
                }
            }
        }
    }

    // Return the count of unique triples
    networks.len() as u32
}

fn pre_calculate_pairs(input: &Vec<Connection>) -> HashMap<&str, Vec<&str>> {
    let mut pairs: HashMap<&str, Vec<&str>> = HashMap::new();
    input.iter().for_each(|cur| {
        pairs
            .entry(&cur.comp1)
            .and_modify(|vec| vec.push(&cur.comp2))
            .or_insert(vec![&cur.comp2]);
        pairs
            .entry(&cur.comp2)
            .and_modify(|vec| vec.push(&cur.comp1))
            .or_insert(vec![&cur.comp1]);
    });
    pairs
}
