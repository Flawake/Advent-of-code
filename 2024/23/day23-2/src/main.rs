use std::collections::{BTreeSet, HashMap, HashSet};
use std::error::Error;
use std::fs;

use itertools::Itertools;

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

//Example pairs:
// "co": ["ka", "ta", "de", "tc"]
// "ub": ["qp", "kh", "wq", "vc"] 
// "vc": ["aq", "ub", "wq", "tb"] 
// "tc": ["kh", "wh", "td", "co"] 
// "cg": ["de", "tb", "yn", "aq"] 
// "ta": ["co", "ka", "de", "kh"] 
// "kh": ["tc", "qp", "ub", "ta"] 
// "qp": ["kh", "ub", "td", "wh"] 
// "yn": ["aq", "cg", "wh", "td"] 
// "aq": ["yn", "vc", "cg", "wq"] 
// "wh": ["tc", "td", "yn", "qp"] 
// "ka": ["co", "tb", "ta", "de"] 
// "de": ["cg", "co", "ta", "ka"] 
// "tb": ["cg", "ka", "wq", "vc"] 
// "td": ["tc", "wh", "qp", "yn"] 
// "wq": ["tb", "ub", "aq", "vc"]

fn calculate(input: &Vec<Connection>) -> String {
    let pairs = pre_calculate_pairs(input);

    let mut networks = HashSet::new();

    for computer in pairs.keys().cloned() {
        let mut current_network = BTreeSet::from([computer.to_string()]);
        search_biggest_network(computer, pairs.clone(), &mut current_network, &mut networks);
    }

    let mut biggest_network = None;
    networks.iter().for_each(|network| {
        if biggest_network == None {
            biggest_network = Some(network);
        }
        else if biggest_network.unwrap().len() < network.len() {
            biggest_network = Some(network);
        }
    });
    biggest_network.unwrap().iter().join(",")
}

fn search_biggest_network(computer: &str, pairs: HashMap<&str, Vec<&str>>, current_network: &mut BTreeSet<String>, networks: &mut HashSet<BTreeSet<String>>) {
    if networks.insert(current_network.clone()) {
        if let Some(neighbors) = pairs.get(computer) {
            for neighbor in neighbors.clone() {
                let next = pairs.get(neighbor).unwrap();
                if current_network.iter().all(|node|next.contains(&&node[..])) {
                        current_network.insert(neighbor.to_string());
                        search_biggest_network(&neighbor, pairs.clone(), current_network, networks);
                    }
            }
        }
    }
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
