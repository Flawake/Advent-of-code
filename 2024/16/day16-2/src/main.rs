use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::error::Error;
use std::fs;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum Dir {
    North,
    East,
    South,
    West,
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Node {
    pos: (u32, u32),
    cost: u32,
    dir: Dir,
    prev_node_pos: Option<Vec<(u32, u32)>>,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost) // Reverse order for min-heap
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Default for Node {
    fn default() -> Self {
        Node {
            pos: (0, 0),
            cost: u32::MAX,
            dir: Dir::East,
            prev_node_pos: None,
        }
    }
}

impl Dir {
    fn step_player(self, pos: (u32, u32)) -> (u32, u32) {
        match self {
            Dir::North => (pos.0 - 1, pos.1),
            Dir::East => (pos.0, pos.1 + 1),
            Dir::South => (pos.0 + 1, pos.1),
            Dir::West => (pos.0, pos.1 - 1),
        }
    }
}

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
        .map(|line| line.chars().map(|c| c as u8).collect())
        .collect()
}

fn calculate(map: Vec<Vec<u8>>) -> u32 {
    let player_pos = find_first_value(&map, b'S');
    let end_pos = find_first_value(&map, b'E');
    let directions = [Dir::North, Dir::East, Dir::South, Dir::West];

    let mut nodes: Vec<Vec<Node>> = (0..map.len())
        .map(|x| {
            (0..map[0].len())
                .map(|y| Node {
                    pos: (x as u32, y as u32),
                    ..Node::default()
                })
                .collect()
        })
        .collect();

    let mut heap = BinaryHeap::new();

    nodes[player_pos.0 as usize][player_pos.1 as usize].pos = player_pos;
    nodes[player_pos.0 as usize][player_pos.1 as usize].cost = 0;

    heap.push(nodes[player_pos.0 as usize][player_pos.1 as usize].clone());

    while let Some(node) = heap.pop() {
        directions.iter().for_each(|dir| {
            let next_pos = dir.step_player(node.pos);

            if map[next_pos.0 as usize][next_pos.1 as usize] == b'#' {
                return;
            }

            let move_cost = if node.dir == *dir {
                node.cost + 1
            } else {
                node.cost + 1001
            };

            if move_cost < nodes[next_pos.0 as usize][next_pos.1 as usize].cost {
                nodes[next_pos.0 as usize][next_pos.1 as usize].cost = move_cost;
                nodes[next_pos.0 as usize][next_pos.1 as usize].dir = *dir;
                nodes[next_pos.0 as usize][next_pos.1 as usize].prev_node_pos =
                    Some(vec![node.pos]);
                heap.push(nodes[next_pos.0 as usize][next_pos.1 as usize].clone());
            } else if move_cost == nodes[next_pos.0 as usize][next_pos.1 as usize].cost {
                nodes[next_pos.0 as usize][next_pos.1 as usize].dir = *dir;
                nodes[next_pos.0 as usize][next_pos.1 as usize].prev_node_pos =
                    if let Some(mut vec) = nodes[next_pos.0 as usize][next_pos.1 as usize]
                        .prev_node_pos
                        .clone()
                    {
                        vec.push((node.pos.0, node.pos.1));
                        Some(vec)
                    } else {
                        Some(vec![node.pos])
                    };
                heap.push(nodes[next_pos.0 as usize][next_pos.1 as usize].clone());
            }
            else if move_cost == nodes[next_pos.0 as usize][next_pos.1 as usize].cost + 1000 && *dir != nodes[next_pos.0 as usize][next_pos.1 as usize].dir {
                let future_pos = dir.step_player(next_pos);
                if map[future_pos.0 as usize][future_pos.1 as usize] == b'#' {
                    return;
                }
                nodes[next_pos.0 as usize][next_pos.1 as usize].dir = *dir;
                nodes[next_pos.0 as usize][next_pos.1 as usize].prev_node_pos =
                    if let Some(mut vec) = nodes[next_pos.0 as usize][next_pos.1 as usize]
                        .prev_node_pos
                        .clone()
                    {
                        vec.push((node.pos.0, node.pos.1));
                        Some(vec)
                    } else {
                        Some(vec![node.pos])
                    };
                heap.push(nodes[next_pos.0 as usize][next_pos.1 as usize].clone());
            }
        });
    }

    let mut walked_tiles = HashMap::new();

    let end_node = &nodes[end_pos.0 as usize][end_pos.1 as usize];

    let mut nodes_to_check = Vec::new();
    nodes_to_check.push(end_node);

    let mut walked_map = map.clone();

    while let Some(node) = nodes_to_check.pop() {
        walked_tiles.insert((node.pos.0, node.pos.1), true);
        walked_map[node.pos.0 as usize][node.pos.1 as usize] = b'O';
        if let Some(prev_positions) = node.prev_node_pos.clone() {
            prev_positions.iter().for_each(|pos| {
                nodes_to_check.push(&nodes[pos.0 as usize][pos.1 as usize]);
            });
        }
    }

    print_map(walked_map);

    walked_tiles.len() as u32
}

fn print_map(map: Vec<Vec<u8>>) {
    map.iter().for_each(|l| {
        l.iter().for_each(|c| print!("{}", *c as char));
        println!("");
    });
}

fn find_first_value(map: &Vec<Vec<u8>>, find: u8) -> (u32, u32) {
    map.iter()
        .enumerate()
        .find_map(|(y, line)| {
            line.iter()
                .position(|&val| val == find)
                .map(|x| (y as u32, x as u32))
        })
        .unwrap()
}

//503 is too high
//501 is too low
//500 is too low

//Must be 502 then -_-
