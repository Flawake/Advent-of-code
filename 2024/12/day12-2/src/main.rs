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
    let map = points_to_map(input);
    let mut sides = 0;
    map.iter().for_each(|r| {
        r.iter().for_each(|v| {
            print!("{}", if *v == true { '*' } else { '.' });
        });
        println!(" ")
    });

    sides += search_upper_lines(&map);
    println!("{}", sides);
    sides += search_right_lines(&map);
    println!("{}", sides);
    sides += search_lower_lines(&map);
    println!("{}", sides);
    sides += search_left_lines(&map);
    println!("{}", sides);
    sides
}

fn search_upper_lines(map: &Vec<Vec<bool>>) -> u32 {
    if map[0].len() == 1 {
        return 1;
    }

    let mut sides = 0;

    for (x, vec) in map.iter().enumerate() {
        let mut was_side = false;
        if x == 0 {
            for (y, &val) in vec.iter().enumerate() {
                if val == true {
                    was_side = true;
                    if y == map[0].len() - 1 {
                        sides += 1;
                        was_side = false;
                    }
                } else {
                    if was_side {
                        sides += 1;
                        was_side = false;
                    }
                }
            }
        } else {
            for (y, val) in vec.iter().enumerate() {
                if *val == true && map[x - 1][y] == false {
                    was_side = true;
                    if y == map[0].len() - 1 {
                        sides += 1;
                        was_side = false;
                    }
                } else {
                    if was_side {
                        sides += 1;
                        was_side = false;
                    }
                }
            }
        }
    }
    sides
}

fn search_lower_lines(map: &Vec<Vec<bool>>) -> u32 {
    if map[0].len() == 1 {
        return 1;
    }

    let mut sides = 0;

    for (x, vec) in map.iter().enumerate() {
        let mut was_side = false;
        if x == map.len() - 1{
            for (y, &val) in vec.iter().enumerate() {
                if val == true {
                    was_side = true;
                    if y == map[0].len() - 1 {
                        sides += 1;
                        was_side = false;
                    }
                } else {
                    if was_side {
                        sides += 1;
                        was_side = false;
                    }
                }
            }
        } else {
            for (y, val) in vec.iter().enumerate() {
                if *val == true && map[x + 1][y] == false {
                    was_side = true;
                    if y == map[0].len() - 1 {
                        sides += 1;
                        was_side = false;
                    }
                } else {
                    if was_side {
                        sides += 1;
                        was_side = false;
                    }
                }
            }
        }
    }
    sides
}

fn search_left_lines(map: &Vec<Vec<bool>>) -> u32 {
    if map.len() == 1 {
        return 1;
    }

    let mut sides = 0;

    for y in 0..map[0].len() {
        let mut was_side = false;
        if y == 0 {
            for x in 0..map.len() {
                if map[x][y] == true {
                    was_side = true;
                    if x == map.len() - 1 {
                        sides += 1;
                        was_side = false;
                    }
                } else {
                    if was_side {
                        sides += 1;
                        was_side = false;
                    }
                }
            }
        }
        else {
            for x in 0..map.len() {
                if map[x][y] == true && map[x][y - 1] == false {
                    was_side = true;
                    if x == map.len() - 1 {
                        sides += 1;
                        was_side = false;
                    }
                } else {
                    if was_side {
                        sides += 1;
                        was_side = false;
                    }
                }
            }
        }
    }
    sides
}

fn search_right_lines(map: &Vec<Vec<bool>>) -> u32 {
    if map.len() == 1 {
        return 1;
    }
    let mut sides = 0;

    for y in 0..map[0].len() {
        let mut was_side = false;
        if y == map[0].len() - 1 {
            for x in 0..map.len() {
                if map[x][y] == true {
                    was_side = true;
                    if x == map.len() - 1 {
                        sides += 1;
                        was_side = false;
                    }
                } else {
                    if was_side {
                        sides += 1;
                        was_side = false;
                    }
                }
            }
        }
        else {
            for x in 0..map.len() {
                if map[x][y] == true && map[x][y + 1] == false {
                    was_side = true;
                    if x == map.len() - 1 {
                        sides += 1;
                        was_side = false;
                    }
                } else {
                    if was_side {
                        sides += 1;
                        was_side = false;
                    }
                }
            }
        }
    }
    sides
}

fn points_to_map(input: &Vec<(u32, u32)>) -> Vec<Vec<bool>> {
    let min_x = input.iter().map(|(x, _)| *x).min().unwrap() as usize;
    let max_x = input.iter().map(|(x, _)| *x).max().unwrap() as usize;

    let min_y = input.iter().map(|(_, y)| *y).min().unwrap() as usize;
    let max_y = input.iter().map(|(_, y)| *y).max().unwrap() as usize;

    let mut res = vec![vec![false; max_y - min_y + 1]; max_x - min_x + 1];
    for (x, y) in input {
        res[*x as usize - min_x][*y as usize - min_y] = true;
    }
    res
}
/*
fn calculate_sides(input: &Vec<(u32, u32)>) -> u32 {
    println!("{:?}", input);
    let mut current_tile = input[0];
    let mut perimeter = 0;

    #[derive(PartialEq, Eq)]
    enum Dir {
        North,
        East,
        South,
        West,
    }

    let mut direction = Dir::East;
    let mut first_loop = true;
    loop {
        if !first_loop {
            if current_tile == input[0] && direction == Dir::East {
                break;
            }
        }
        first_loop = false;
        let mut do_break = false;
        if direction == Dir::North {
            while input.contains(&(current_tile.0.wrapping_sub(1), current_tile.1)) {
                if input.contains(&(current_tile.0, current_tile.1.wrapping_sub(1))) {
                    current_tile = (current_tile.0, current_tile.1 - 1);
                    direction = Dir::West;
                    perimeter += 1;
                    do_break = true;
                    break;
                }
                current_tile = (current_tile.0 - 1, current_tile.1)
            }
            if do_break {
                continue;
            }
            if input.contains(&(current_tile.0, current_tile.1.wrapping_sub(1))) {
                current_tile = (current_tile.0, current_tile.1 - 1);
                direction = Dir::West;
                perimeter += 1;
                continue;
            }
            direction = Dir::East;
            perimeter += 1;
            continue;
        } else if direction == Dir::East {
            while input.contains(&(current_tile.0, current_tile.1 + 1)) {
                if input.contains(&(current_tile.0.wrapping_sub(1), current_tile.1)) {
                    current_tile = (current_tile.0 - 1, current_tile.1);
                    direction = Dir::North;
                    perimeter += 1;
                    do_break = true;
                    break;
                }
                current_tile = (current_tile.0, current_tile.1 + 1)
            }
            if do_break {
                continue;
            }
            if input.contains(&(current_tile.0.wrapping_sub(1), current_tile.1)) {
                current_tile = (current_tile.0 - 1, current_tile.1);
                direction = Dir::North;
                perimeter += 1;
                continue;
            }
            direction = Dir::South;
            perimeter += 1;
            continue;
        } else if direction == Dir::South {
            while input.contains(&(&current_tile.0 + 1, current_tile.1)) {
                if input.contains(&(current_tile.0, current_tile.1 + 1)) {
                    current_tile = (current_tile.0, current_tile.1 + 1);
                    direction = Dir::East;
                    perimeter += 1;
                    do_break = true;
                    break;
                }
                current_tile = (current_tile.0 + 1, current_tile.1)
            }
            if do_break {
                continue;
            }
            if input.contains(&(current_tile.0, current_tile.1 + 1)) {
                current_tile = (current_tile.0, current_tile.1 + 1);
                direction = Dir::East;
                perimeter += 1;
                continue;
            }
            direction = Dir::West;
            perimeter += 1;
            continue;
        } else if direction == Dir::West {
            while input.contains(&(current_tile.0, current_tile.1.wrapping_sub(1))) {
                if input.contains(&(current_tile.0 + 1, current_tile.1)) {
                    current_tile = (current_tile.0 + 1, current_tile.1);
                    direction = Dir::South;
                    perimeter += 1;
                    do_break = true;
                    break;
                }
                current_tile = (current_tile.0, current_tile.1 - 1)
            }
            if do_break {
                continue;
            }
            if input.contains(&(current_tile.0 + 1, current_tile.1)) {
                current_tile = (current_tile.0 + 1, current_tile.1);
                direction = Dir::South;
                perimeter += 1;
                continue;
            }
            direction = Dir::North;
            perimeter += 1;
            continue;
        }
    }
    perimeter
}
*/
