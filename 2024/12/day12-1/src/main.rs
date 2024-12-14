//Accidently erased the calculate_sides function while doing day 13, code is not working.

use std::collections::HashMap;
use std::error::Error;
use std::{fs, vec};

fn main() -> Result<(), Box<dyn Error>> {
    let input: String = fs::read_to_string("input.txt")?;
    let parsed_input = parse_input(&input);
    let ans = calculate(parsed_input);
    println!("{}", ans);
    Ok(())
}

fn parse_input(input: &String) -> Vec<Vec<u8>> {
    input
        .lines()
        .into_iter()
        .map(|str| str.chars().into_iter().map(|char| char as u8).collect())
        .collect()
}

fn calculate(input: Vec<Vec<u8>>) -> u32 {
    let mut visited = HashMap::new();
    let mut gardens = Vec::new();

    input.iter().enumerate().for_each(|(x, vec)| {
        vec.iter().enumerate().for_each(|(y, _)| {
            if visited.contains_key(&(x, y)) {
                return;
            }

            let val = input[x][y];
            let garden = explore_garden(&input, &mut visited, (x, y), val);
            gardens.push(garden);
        });
    });
    gardens.iter().fold(0, |acc, garden| {
        acc + (calculate_sides(&garden) * garden.len() as u32)
    })
}

fn explore_garden(
    map: &Vec<Vec<u8>>,
    visited: &mut HashMap<(usize, usize), bool>,
    start: (usize, usize),
    target_value: u8,
) -> Vec<(u32, u32)> {
    let mut stack = vec![start];
    let mut garden = Vec::new();

    while let Some((x, y)) = stack.pop() {
        if visited.contains_key(&(x, y)) || map[x][y] != target_value {
            continue;
        }

        visited.insert((x, y), true);
        garden.push((x as u32, y as u32));

        neighbors(x, y, map)
            .iter()
            .for_each(|(nx, ny)| stack.push((*nx, *ny)));
    }

    garden
}

fn neighbors(x: usize, y: usize, map: &Vec<Vec<u8>>) -> Vec<(usize, usize)> {
    let mut neighbors = Vec::new();
    if x > 0 {
        neighbors.push((x - 1, y));
    }
    if y > 0 {
        neighbors.push((x, y - 1));
    }
    if x + 1 < map.len() {
        neighbors.push((x + 1, y));
    }
    if y + 1 < map[0].len() {
        neighbors.push((x, y + 1));
    }
    neighbors
}

fn calculate_sides(input: &Vec<(u32, u32)>) -> u32 {
    0
}
