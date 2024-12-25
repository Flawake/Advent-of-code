use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::error::Error;
use std::hash::Hasher;
use std::u8;
use std::{fs, hash::Hash, vec};

// fact1: Each layer above an other layer, has as many A's as there are moves in the lower layer.
// fact2: If a layer presses an A, that means that all layers above that layer are simultaniously also pressing an A.
// This means that you can see each A as a reset of the positions, this can maybe decrease computation time, maybe with memoization?

// Highest layer
//  <vA<AA>>^AvAA<^A>A<v<A>>^AvA^A<vA>^A<v<A>^A>AAvA^A<v<A>A>^AAAvA<^A>A
//  v<<A>>^A<A>AvA<^AA>A<vAAA>^A
//  <A^A>^^AvvvA
//  029A
// Lowest layer

//  <vA | <A | A | >>^A | vA | A | <^A | >A | <v<A | >>^A | vA | ^A | <vA | >^A | <v<A | >^A | >A | A | vA | ^A | <v<A | >A | >^A | A | A | vA | <^A | >A
//  v<<A                | >>^A              | <A          | >A      | vA        | <^A             | A | >A      | <vA             | A | A | >^A
//  <A                                      | ^A                    | >^^A                                      | vvvA
//  0                                       | 2                     | 9                                         | A

//  A -> 0                                  | 0 -> 2                | 2 -> 9                                    | 9 -> A

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Pos {
    x: u8,
    y: u8,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Key {
    val: char,
    pos: Pos,
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct PathKey {
    prev_key_pos: Vec<Pos>,
    cost: u8,
    key: Key,
}

impl Pos {
    const fn from(x: u8, y: u8) -> Self {
        Pos {
            x: x,
            y: y
        }
    }
}

impl Hash for Key {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.pos.hash(state);
    }
}

impl Hash for PathKey {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.key.hash(state);
    }
}

impl Default for Key {
    fn default() -> Self {
        Key {
            val: ' ',
            pos: Pos {
                x: u8::MAX,
                y: u8::MAX,
            }
        }
    }
}

impl Default for PathKey {
    fn default() -> Self {
        PathKey {
            prev_key_pos: Vec::new(),
            cost: u8::MAX,
            key: Key::default(),
        }
    }
}

impl Ord for PathKey {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost) // Reverse order for min-heap
    }
}

impl PartialOrd for PathKey {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

const NUMERIC_KEYS: [(char, Pos); 11] = [
    ('7', Pos::from(0, 0)),
    ('8', Pos::from(1, 0)),
    ('9', Pos::from(2, 0)),
    ('4', Pos::from(0, 1)),
    ('5', Pos::from(1, 1)),
    ('6', Pos::from(2, 1)),
    ('1', Pos::from(0, 2)),
    ('2', Pos::from(1, 2)),
    ('3', Pos::from(2, 2)),
    ('0', Pos::from(1, 3)),
    ('A', Pos::from(2, 3)),
];

const DIRECTIONAL_KEYS: [(char, Pos); 5] = [
    ('^', Pos::from(1, 0)),
    ('A', Pos::from(2, 0)),
    ('<', Pos::from(0, 1)),
    ('v', Pos::from(1, 1)),
    ('>', Pos::from(2, 1)),
];

enum Dir {
    North,
    East,
    South,
    West,
}

impl Dir {
    pub fn next(&self, pos: Pos, table: &[(char, Pos)]) -> Option<Pos> {
        match self {
            Dir::North => {
                let next = Pos::from(pos.x, pos.y.wrapping_sub(1));
                if table_contains_pos(next, table) {
                    return Some(next);
                }
                return None;
            }
            Dir::East => {
                let next =  Pos::from(pos.x + 1, pos.y);
                if table_contains_pos(next, table) {
                    return Some(next);
                }
                return None;
            }
            Dir::South => {
                let next =  Pos::from(pos.x, pos.y + 1);
                if table_contains_pos(next, table) {
                    return Some(next);
                }
                return None;
            }
            Dir::West => {
                let next =  Pos::from(pos.x.wrapping_sub(1), pos.y);
                if table_contains_pos(next, table) {
                    return Some(next);
                }
                return None;
            }
        }
    }
}

fn table_contains_pos(pos: Pos, table: &[(char, Pos)]) -> bool {
    table.iter().any(|(_, v)| *v == pos)
}

fn precompute_fastest_paths_numeric_keyboard() -> HashMap<(Pos, Pos), Vec<Vec<PathKey>>> {
    let directions = [Dir::North, Dir::East, Dir::South, Dir::West];
    let table = &NUMERIC_KEYS;

    let mut precomputed_values = HashMap::new();

    table.iter().for_each(|from| {
        table.iter().for_each(|to| {
            let from_pos = from.1;
            let end_pos = to.1;
            if from.1 == to.1 || !table_contains_pos(from_pos, table) || !table_contains_pos(end_pos, table) {
                return;
            }

            let mut paths = Vec::new();

            let mut heap = BinaryHeap::new();

            let mut keys: HashMap<Pos, PathKey> = HashMap::new();
            for i in 0..table.len() {
                keys.insert(
                    table[i].1,
                    PathKey {
                        key: Key {
                            pos: table[i].1,
                            ..Key::default()
                        },
                        ..PathKey::default()
                    },
                );
            }

            //Push the first pathKey on the heap, we might be able to get the key beforehand from the keyboard in the future.
            heap.push(PathKey {
                key: Key {
                    pos: from_pos,
                    ..Key::default()
                },
                ..PathKey::default()
            });

            while let Some(key) = heap.pop() {
                directions.iter().for_each(|dir| {
                    let next_pos = dir.next(from_pos, table);
                    if next_pos == None {
                        return;
                    }
                    let next_pos = next_pos.unwrap();

                    let mut next_key = keys.clone().get(&next_pos).unwrap().clone();

                    let new_cost = key.cost + 1;
                    if new_cost < next_key.cost {
                        next_key.cost = new_cost;
                        next_key.prev_key_pos = vec![key.key.pos];
                        keys.insert(next_pos, next_key.clone());
                        heap.push(next_key);
                    }
                    else if new_cost == next_key.cost {
                        next_key.cost = new_cost;
                        next_key.prev_key_pos.push(key.key.pos);
                        keys.insert(next_pos, next_key.clone());
                        heap.push(next_key);
                    }
                });
            }

            let last_key = keys.get(&end_pos).unwrap();
            let mut nodes: Vec<&PathKey> = vec![last_key];
            while !nodes.is_empty() {
                let node = nodes.pop().unwrap();
                node.prev_key_pos.iter().for_each(|key| {
                    nodes.push(keys.get(key).unwrap());
                    paths.push(keys.get(key).unwrap());
                });
            }

            precomputed_values.insert((from, to), paths.clone());
            println!("From  : {:?}", from);
            println!("To    : {:?}", to);
            println!("Paths : {:?}", paths);
        });
    });
    todo!()
}

fn key_to_pos(key: char, depth: u8) -> Pos {
    let table: &[(char, Pos)] = if depth == 0 {
        &DIRECTIONAL_KEYS
    } else {
        &NUMERIC_KEYS
    };

    table
        .iter()
        .find_map(|&(k, v)| if k == key { Some(v) } else { None })
        .unwrap()
}

fn main() -> Result<(), Box<dyn Error>> {
    let input: String = fs::read_to_string("input.txt")?;
    let parsed_input = parse_input(&input);
    let ans = calculate(&parsed_input);
    println!("{}", ans);
    Ok(())
}

fn parse_input(input: &String) -> Vec<String> {
    input.lines().map(|str| str.to_string()).collect()
}

fn calculate(input: &Vec<String>) -> u32 {
    input.iter().for_each(|line| {
        line.chars().into_iter().for_each(|char| {
            find_most_efficient_path(vec![char], 0);
        });
    });

    0
}

fn find_most_efficient_path(instructions: Vec<char>, depth: u8) -> Vec<char> {
    let robot_pos;
    if depth == 0 {
        robot_pos = key_to_pos(instructions[0], depth);
    }
    vec![]
}

fn code_to_numeric(code: &String) -> u32 {
    code.chars()
        .into_iter()
        .filter(|c| c.is_numeric())
        .map(|c| c.to_digit(10).unwrap())
        .fold(0, |acc, num| acc * 10 + num)
}

mod tests {
    use super::*;

    #[test]
    fn test_code_to_numeric() {
        let expected = 27;
        let result = code_to_numeric(&"027U".to_string());
        assert_eq!(expected, result);

        let expected = 7;
        let result = code_to_numeric(&"t7U".to_string());
        assert_eq!(expected, result);

        let expected = 70;
        let result = code_to_numeric(&"t70U".to_string());
        assert_eq!(expected, result);

        let expected = 932;
        let result = code_to_numeric(&"AS000932".to_string());
        assert_eq!(expected, result);
    }

    #[test]
    fn test_next_dir() {
        let expected = Some( Pos::from(1, 1));
        let result = Dir::North.next( Pos::from(1, 2), &NUMERIC_KEYS);
        assert_eq!(expected, result);

        let expected = None;
        let result = Dir::North.next( Pos::from(1, 0), &NUMERIC_KEYS);
        assert_eq!(expected, result);

        let expected = None;
        let result = Dir::East.next( Pos::from(2, 1), &DIRECTIONAL_KEYS);
        assert_eq!(expected, result);

        let expected = Some( Pos::from(0, 1));
        let result = Dir::West.next( Pos::from(1, 1), &DIRECTIONAL_KEYS);
        assert_eq!(expected, result);
    }
}
