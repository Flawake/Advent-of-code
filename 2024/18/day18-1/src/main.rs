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
    pos: (u8, u8),
    cost: u32
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
            cost: u32::MAX
        }
    }
}

impl Dir {
    fn step_player(self, pos: (u8, u8), map: &Vec<Vec<u8>>) -> Option<(u8, u8)> {
        match self {
            Dir::North =>if pos.0 > 0 {Some((pos.0 - 1, pos.1))} else {None},
            Dir::East => if pos.1 < (map[0].len() - 1) as u8 {Some((pos.0, pos.1 + 1))} else {None},
            Dir::South => if pos.0 < (map.len() - 1) as u8 {Some((pos.0 + 1, pos.1))} else {None},
            Dir::West => if pos.1 > 0 {Some((pos.0, pos.1 - 1))} else {None},
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let input: String = fs::read_to_string("input.txt")?;
    let parsed_input = parse_input(&input);
    let ans = calculate(&parsed_input);
    println!("{}", ans);
    Ok(())
}

fn parse_input(input: &String) -> Vec<(u16, u16)> {
    input
        .lines()
        .map(|line| {
            let split: Vec<&str> = line.split(",").collect();
            (
                split[0].parse::<u16>().unwrap(),
                split[1].parse::<u16>().unwrap(),
            )
        })
        .collect()
}

fn calculate(input: &Vec<(u16, u16)>) -> u32 {
    let width = 71;
    let heigth = 71;
    let mut map = vec![vec![u8::MIN; width]; heigth];

    for i in 0..1024 {
        let v = input[i];
        map[v.1 as usize][v.0 as usize] = 1
    }
    map.iter().for_each(|v| {
        v.iter().for_each(|c| print!("{}", if *c == 1 {
            "#"
        }
    else {
        "."
    }));
        println!();
    });
    let start_pos = (0, 0);
    let end_pos = (width as u8 - 1, heigth as u8 - 1);

    find_shortest_path(map, start_pos, end_pos)
}

fn find_shortest_path(map: Vec<Vec<u8>>, start_pos: (u8, u8), end_pos: (u8, u8)) -> u32 {
    let directions = [Dir::North, Dir::East, Dir::South, Dir::West];

    let mut nodes: Vec<Vec<Node>> = (0..map.len()).map(|x| {
        (0..map[0].len()).map(|y| Node {
            pos: (x as u8, y as u8),
            ..Node::default()
        }).collect()
    }).collect();

    let mut heap = BinaryHeap::new();

    nodes[start_pos.0 as usize][start_pos.1 as usize].pos = start_pos;
    nodes[start_pos.0 as usize][start_pos.1 as usize].cost = 0;

    heap.push(nodes[start_pos.0 as usize][start_pos.1 as usize].clone());

    while let Some(node) = heap.pop() {
        if node.pos == end_pos {
            return node.cost;
        }

        directions.iter().for_each(|dir| {
            let next_pos = dir.step_player(node.pos, &map);
            if next_pos == None {
                return;
            }
            let next_pos = next_pos.unwrap();
            if map[next_pos.0 as usize][next_pos.1 as usize] == 1 {
                return;
            }

            let move_cost = node.cost + 1;

            if move_cost < nodes[next_pos.0 as usize][next_pos.1 as usize].cost {
                nodes[next_pos.0 as usize][next_pos.1 as usize].cost = move_cost;
                heap.push(nodes[next_pos.0 as usize][next_pos.1 as usize].clone());
            }
        });
    }

    nodes.iter().for_each(|v| {
        v.iter().for_each(|node| print!(" :{}: ", node.cost));
        println!("");
    });

    0
}
