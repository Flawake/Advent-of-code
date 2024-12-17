use std::cmp::Ordering;
use std::collections::BinaryHeap;
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

    let mut nodes: Vec<Vec<Node>> = (0..map.len()).map(|x| {
        (0..map[0].len()).map(|y| Node {
            pos: (x as u32, y as u32),
            ..Node::default()
        }).collect()
    }).collect();

    let mut heap = BinaryHeap::new();

    nodes[player_pos.0 as usize][player_pos.1 as usize].pos = player_pos;
    nodes[player_pos.0 as usize][player_pos.1 as usize].cost = 0;

    heap.push(nodes[player_pos.0 as usize][player_pos.1 as usize].clone());

    while let Some(node) = heap.pop() {
        if node.pos == end_pos {
            return node.cost;
        }

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
                heap.push(nodes[next_pos.0 as usize][next_pos.1 as usize].clone());
            }
        });
    }

    0
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
