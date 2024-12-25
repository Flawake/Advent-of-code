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
    prev_node_pos: Option<(u32, u32)>,
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
            prev_node_pos: None,
        }
    }
}

impl Dir {
    fn step_player(self, pos: (u32, u32), map: &Vec<Vec<u8>>) -> Option<(u32, u32)> {
        match self {
            Dir::North => {
                if pos.0 > 0 {
                    return Some((pos.0 - 1, pos.1));
                } else {
                    None
                }
            }
            Dir::East => {
                if pos.1 < (map[0].len() - 1) as u32 {
                    Some((pos.0, pos.1 + 1))
                } else {
                    None
                }
            }
            Dir::South => {
                if pos.0 < (map.len() - 1) as u32 {
                    Some((pos.0 + 1, pos.1))
                } else {
                    None
                }
            }
            Dir::West => {
                if pos.1 > 0 {
                    Some((pos.0, pos.1 - 1))
                } else {
                    None
                }
            }
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

fn parse_input(input: &String) -> Vec<Vec<u8>> {
    input
        .lines()
        .map(|line| line.chars().map(|c| c as u8).collect())
        .collect()
}

fn calculate(map: &Vec<Vec<u8>>) -> u32 {
    let start_pos = find_first_value(map, b'S');
    let end_pos = find_first_value(map, b'E');
    let nodes = dijkstra(map, start_pos, end_pos, vec![b'#']);
    print_cost_map(map, &nodes);
    nodes.iter().for_each(|node| println!("{:?}", node));
    let cheats = find_cheats(&nodes);
    let ans = cheats.iter().fold(0,|acc, (_, value)| acc + value);
    ans
}

fn dijkstra(
    map: &Vec<Vec<u8>>,
    start_pos: (u32, u32),
    end_pos: (u32, u32),
    obstacles: Vec<u8>,
) -> Vec<Node> {
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

    nodes[start_pos.0 as usize][start_pos.1 as usize].pos = start_pos;
    nodes[start_pos.0 as usize][start_pos.1 as usize].cost = 0;

    heap.push(nodes[start_pos.0 as usize][start_pos.1 as usize].clone());

    while let Some(mut node) = heap.pop() {
        if node.pos == end_pos {
            let mut path = Vec::new();
            path.push(node.clone());
            while node.prev_node_pos != None {
                node = nodes[node.prev_node_pos.unwrap().0 as usize]
                    [node.prev_node_pos.unwrap().1 as usize]
                    .clone();
                path.push(node.clone());
            }
            if start_pos == (7, 5) && end_pos == (3, 1) {
                print_cost_map(map, &path);
            }
            return path;
        }

        directions.iter().for_each(|dir| {
            let next_pos = dir.step_player(node.pos, &map);
            if next_pos.is_none() {
                return;
            }
            let next_pos = next_pos.unwrap();

            if next_pos != end_pos && obstacles.contains(&map[next_pos.0 as usize][next_pos.1 as usize]) {
                return;
            }

            let move_cost = node.cost + 1;

            if move_cost < nodes[next_pos.0 as usize][next_pos.1 as usize].cost {
                nodes[next_pos.0 as usize][next_pos.1 as usize].cost = move_cost;
                nodes[next_pos.0 as usize][next_pos.1 as usize].prev_node_pos = Some(node.pos);
                heap.push(nodes[next_pos.0 as usize][next_pos.1 as usize].clone());
            }
        });
    }
    Vec::new()
}

fn find_cheats(nodes: &Vec<Node>) -> HashMap<u32, u32> {
    let mut improvements = HashMap::new();
    nodes.iter().enumerate().for_each(|(i, node)| {
        nodes.iter().skip(i + 1).for_each(|next| {
            let distance = manhattan_distance(node, next);
            if distance > 20 {
                return;
            }
            let improvement = improvement(node.cost, next.cost, distance);
            if improvement < 100 {
                return;
            }
            improvements
                    .entry(improvement)
                    .and_modify(|amount| *amount += 1)
                    .or_insert(1);
        });
    });
    improvements
}

fn improvement(x_cost: u32, y_cost: u32, dijkstra_cost: u32) -> u32 {
    (x_cost - y_cost).saturating_sub(dijkstra_cost)
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

fn manhattan_distance(x: &Node, y: &Node) -> u32{
    x.pos.0.abs_diff(y.pos.0) + x.pos.1.abs_diff(y.pos.1)
}

#[allow(dead_code)]
fn print_map(map: &Vec<Vec<u8>>) {
    println!("Map:");
    map.iter().for_each(|line| println!("{:?}", line));
}

#[allow(dead_code)]
fn print_cost_map(map: &Vec<Vec<u8>>, nodes: &Vec<Node>) {
    println!("Map:");
        map.iter().enumerate().for_each(|(i, line)| {
            line.iter().enumerate().for_each(|(j, _)| {
                let mut printed = false;
                for node in nodes {
                    if node.pos == (i as u32, j as u32) {
                        print!(" {:02} ", node.cost);
                        printed = true;
                        break;
                    }
                }
                if !printed {
                    print!(" -- ")
                }
            });
            println!();
        }
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[ignore]
    #[test]
    fn test_improvement() {
        let expected = 76;
        let result = improvement(0, 82, 6);
        assert_eq!(result, expected);

        let expected = 2;
        let result = improvement(1, 5, 2);
        assert_eq!(result, expected);

        let expected = 12;
        let result = improvement(12, 26, 2);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_manhattan() {
        let node1 = Node {
            pos: (1, 1),
            ..Node::default()
        };
        let node2 = Node {
            pos: (2, 5),
            ..Node::default()
        };
        let expected = 5;
        let result = manhattan_distance(&node1, &node2);
        assert_eq!(result, expected);
    }
}
