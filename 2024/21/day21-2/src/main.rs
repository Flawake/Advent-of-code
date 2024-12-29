use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::error::Error;
use std::hash::Hasher;
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

//  A | <vA | <A | A | >>^A | vA | A | <^A | >A | <v<A | >>^A | vA | ^A | <vA | >^A | <v<A | >^A | >A | A | vA | ^A | <v<A | >A | >^A | A | A | vA | <^A | >A
//  A | v<<A                | >>^A              | <A          | >A      | vA        | <^A             | A | >A      | <vA             | A | A | >^A
//  A | <A                                      | ^A                    | >^^A                                      | vvvA
//  A | 0                                         2                       9                                           A

//  A -> 0                                  | 0 -> 2                | 2 -> 9                                    | 9 -> A

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Pos {
    x: u8,
    y: u8,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Key {
    val: Option<char>,
    pos: Pos,
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct PathKey {
    prev_key_pos: Vec<Pos>,
    cost: u8,
    key: Key,
}

#[derive(Debug)]
struct Table {
    keys: Vec<Key>,
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Path {
    start_pos: Pos,
    end_pos: Pos,
}

#[derive(Debug, Clone)]
struct Route {
    route: Vec<PathKey>,
}

#[derive(Debug, Clone)]
struct Routes {
    routes: Vec<Route>,
}

impl Hash for PathKey {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.key.hash(state);
    }
}

impl Default for Key {
    fn default() -> Self {
        Key {
            val: None,
            pos: Pos {
                x: u8::MAX,
                y: u8::MAX,
            },
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
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for PathKey {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Routes {
    fn from_world(&mut self, keys: &HashMap<Pos, PathKey>, node: PathKey, route: &mut Route) {
        node.clone().prev_key_pos.iter().for_each(|pos| {
            let prev_node = keys.get(pos).unwrap().clone();
            route.route.push(prev_node.clone());
            self.from_world(keys, prev_node.clone(), route);
            if prev_node.prev_key_pos.is_empty() {
                route.route.reverse();
                self.routes.push(route.clone());
                route.route.reverse();
            }
            route.route.pop();
        });
    }
}

impl Table {
    fn precompute_fastest_paths(&self) -> HashMap<Path, Routes> {
        let directions = [Dir::North, Dir::East, Dir::South, Dir::West];

        let mut precomputed_values = HashMap::new();

        self.keys.iter().for_each(|from_key| {
            self.keys.iter().for_each(|to_key| {
                let from_pos = from_key.pos;
                let end_pos = to_key.pos;
                if !self.contains_pos(from_pos) || !self.contains_pos(end_pos) {
                    return;
                }

                let mut heap = BinaryHeap::new();

                let mut keys: HashMap<Pos, PathKey> = HashMap::new();

                let first_key = PathKey {
                    key: Key {
                        pos: from_pos,
                        ..Key::default()
                    },
                    cost: 0,
                    ..PathKey::default()
                };

                for i in 0..self.keys.len() {
                    keys.insert(
                        self.keys[i].pos,
                        PathKey {
                            key: Key {
                                pos: self.keys[i].pos,
                                ..Key::default()
                            },
                            ..PathKey::default()
                        },
                    );
                }

                keys.insert(first_key.key.pos, first_key.clone());
                heap.push(first_key);

                while let Some(key) = heap.pop() {
                    directions.iter().for_each(|dir| {
                        let next_pos = self.next(key.key.pos, dir);
                        if next_pos == None {
                            return;
                        }
                        let next_pos = next_pos.unwrap();

                        let mut next_key = keys.get(&next_pos).unwrap().clone();

                        let new_cost = key.cost + 1;
                        if new_cost < next_key.cost {
                            next_key.cost = new_cost;
                            next_key.prev_key_pos = vec![key.key.pos];
                            keys.insert(next_pos, next_key.clone());
                            heap.push(next_key);
                        } else if new_cost == next_key.cost
                            && !next_key.prev_key_pos.contains(&key.key.pos)
                        {
                            next_key.cost = new_cost;
                            next_key.prev_key_pos.push(key.key.pos);
                            keys.insert(next_pos, next_key.clone());
                            heap.push(next_key);
                        }
                    });
                }

                let last_key = keys.get(&end_pos).unwrap().clone();
                let mut current_route: Route = Route {
                    route: vec![last_key.clone()],
                };
                let mut final_routes = Routes { routes: Vec::new() };
                if last_key.prev_key_pos.len() == 0 {
                    final_routes.routes.push(current_route.clone());
                }
                final_routes.from_world(&keys, last_key.clone(), &mut current_route);
                precomputed_values.insert(
                    Path {
                        start_pos: from_key.pos,
                        end_pos: to_key.pos,
                    },
                    final_routes.clone(),
                );
            });
        });
        precomputed_values
    }

    pub fn next(&self, pos: Pos, dir: &Dir) -> Option<Pos> {
        match dir {
            Dir::North => {
                let next = Pos::from(pos.x, pos.y.wrapping_sub(1));
                if self.contains_pos(next) {
                    return Some(next);
                }
                return None;
            }
            Dir::East => {
                let next = Pos::from(pos.x + 1, pos.y);
                if self.contains_pos(next) {
                    return Some(next);
                }
                return None;
            }
            Dir::South => {
                let next = Pos::from(pos.x, pos.y + 1);
                if self.contains_pos(next) {
                    return Some(next);
                }
                return None;
            }
            Dir::West => {
                let next = Pos::from(pos.x.wrapping_sub(1), pos.y);
                if self.contains_pos(next) {
                    return Some(next);
                }
                return None;
            }
        }
    }

    fn contains_pos(&self, pos: Pos) -> bool {
        self.keys.iter().any(|key| key.pos == pos)
    }

    fn char_to_pos(&self, c: char) -> Pos {
        self.keys
            .iter()
            .find_map(|key| {
                if key.val == Some(c) {
                    Some(key.pos)
                } else {
                    None
                }
            })
            .unwrap()
    }
}

impl Pos {
    fn from(x: u8, y: u8) -> Self {
        Pos { x, y }
    }

    fn adjesant_to_dir(self, other: Pos) -> Dir {
        match (
            other.x as isize - self.x as isize,
            other.y as isize - self.y as isize,
        ) {
            (0, -1) => Dir::North,
            (1, 0) => Dir::East,
            (0, 1) => Dir::South,
            (-1, 0) => Dir::West,
            _ => unreachable!(),
        }
    }
}

impl Hash for Key {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.pos.hash(state);
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Dir {
    North,
    East,
    South,
    West,
}

fn main() -> Result<(), Box<dyn Error>> {
    let numeric_keys = Table {
        keys: vec![
            Key {
                val: Some('7'),
                pos: Pos::from(0, 0),
            },
            Key {
                val: Some('8'),
                pos: Pos::from(1, 0),
            },
            Key {
                val: Some('9'),
                pos: Pos::from(2, 0),
            },
            Key {
                val: Some('4'),
                pos: Pos::from(0, 1),
            },
            Key {
                val: Some('5'),
                pos: Pos::from(1, 1),
            },
            Key {
                val: Some('6'),
                pos: Pos::from(2, 1),
            },
            Key {
                val: Some('1'),
                pos: Pos::from(0, 2),
            },
            Key {
                val: Some('2'),
                pos: Pos::from(1, 2),
            },
            Key {
                val: Some('3'),
                pos: Pos::from(2, 2),
            },
            Key {
                val: Some('0'),
                pos: Pos::from(1, 3),
            },
            Key {
                val: Some('A'),
                pos: Pos::from(2, 3),
            },
        ],
    };

    let directional_keys = Table {
        keys: vec![
            Key {
                val: Some('^'),
                pos: Pos::from(1, 0),
            },
            Key {
                val: Some('A'),
                pos: Pos::from(2, 0),
            },
            Key {
                val: Some('<'),
                pos: Pos::from(0, 1),
            },
            Key {
                val: Some('v'),
                pos: Pos::from(1, 1),
            },
            Key {
                val: Some('>'),
                pos: Pos::from(2, 1),
            },
        ],
    };

    let input: String = fs::read_to_string("input.txt")?;
    let parsed_input = parse_input(&input);
    let fastest_numeric_routes = numeric_keys.precompute_fastest_paths();
    let fastest_directional_routes = directional_keys.precompute_fastest_paths();
    let ans = calculate(
        &parsed_input,
        fastest_numeric_routes,
        fastest_directional_routes,
        &numeric_keys,
        &directional_keys,
    );
    println!("{}", ans);
    Ok(())
}

fn parse_input(input: &String) -> Vec<String> {
    input.lines().map(|str| str.to_string()).collect()
}

fn calculate(
    input: &Vec<String>,
    fastest_numeric_routes: HashMap<Path, Routes>,
    fastest_directional_routes: HashMap<Path, Routes>,
    numeric_keys: &Table,
    directional_keys: &Table,
) -> u64 {
    let mut memoized = HashMap::new();
    input.iter().fold(0, |acc, line| {
        acc + (find_most_efficient_path(
            0,
            26,
            line.chars().collect(),
            &fastest_numeric_routes,
            &fastest_directional_routes,
            &numeric_keys,
            &directional_keys,
            &mut memoized
        ) * code_to_numeric(line))
    })
}

fn find_most_efficient_path(
    depth: u8,
    max_depth: u8,
    route: Vec<char>,
    fastest_numeric_routes: &HashMap<Path, Routes>,
    fastest_directional_routes: &HashMap<Path, Routes>,
    numeric_keys: &Table,
    directional_keys: &Table,
    memoized: &mut HashMap<(Vec<char>, u8), u64>,
) -> u64 {
    let mut input = vec!['A'];
    input.append(&mut route.clone());
    if depth != 0 {
        input.append(&mut vec!['A']);
    }
    if depth == max_depth {
        if depth != 0 {
            return input.len() as u64 - 1;
        }
        return input.len() as u64;
    }
    let table = if depth == 0 {
        numeric_keys
    } else {
        directional_keys
    };

    let score = input
        .windows(2)
        .map(|window| {
            let (first, second) = (window[0], window[1]);
            let pos_first = table.char_to_pos(first);
            let pos_second = table.char_to_pos(second);
            let path = Path {
                start_pos: pos_first,
                end_pos: pos_second,
            };

            let fastest_routes = if depth == 0 {
                fastest_numeric_routes.get(&path).unwrap()
            } else {
                fastest_directional_routes.get(&path).unwrap()
            };

            fastest_routes
                .routes
                .iter()
                .map(|route| {
                    let route = route
                        .route
                        .windows(2)
                        .map(|window| {
                            let cur = window[0].clone();
                            let next = window[1].clone();

                            let dir = cur.key.pos.adjesant_to_dir(next.key.pos);
                            match dir {
                                Dir::North => '^',
                                Dir::East => '>',
                                Dir::South => 'v',
                                Dir::West => '<',
                            }
                        })
                        .collect::<Vec<char>>();
                    if let Some(mem) = memoized.get(&(route.clone(), depth)) {
                        return *mem;
                    }
                    let points = find_most_efficient_path(
                        depth + 1,
                        max_depth,
                        route.clone(),
                        &fastest_numeric_routes,
                        &fastest_directional_routes,
                        numeric_keys,
                        directional_keys,
                        memoized,
                    );
                    memoized.insert((route, depth), points);
                    points
                })
                .min()
                .unwrap()
        })
        .sum();

    score
}

fn code_to_numeric(code: &String) -> u64 {
    code.chars()
        .into_iter()
        .filter(|c| c.is_numeric())
        .map(|c| c.to_digit(10).unwrap() as u64)
        .fold(0, |acc, num| acc * 10 + num)
}

#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn test_code_to_numeric() {
        let expected = 27;
        let result = code_to_numeric(&"027U".to_string());
        assert_eq!(result, expected);

        let expected = 7;
        let result = code_to_numeric(&"t7U".to_string());
        assert_eq!(result, expected);

        let expected = 70;
        let result = code_to_numeric(&"t70U".to_string());
        assert_eq!(result, expected);

        let expected = 932;
        let result = code_to_numeric(&"AS000932".to_string());
        assert_eq!(result, expected);
    }

    #[test]
    fn test_next_dir() {
        let numeric_keys = Table {
            keys: vec![
                Key {
                    val: Some('7'),
                    pos: Pos::from(0, 0),
                },
                Key {
                    val: Some('8'),
                    pos: Pos::from(1, 0),
                },
                Key {
                    val: Some('9'),
                    pos: Pos::from(2, 0),
                },
                Key {
                    val: Some('4'),
                    pos: Pos::from(0, 1),
                },
                Key {
                    val: Some('5'),
                    pos: Pos::from(1, 1),
                },
                Key {
                    val: Some('6'),
                    pos: Pos::from(2, 1),
                },
                Key {
                    val: Some('1'),
                    pos: Pos::from(0, 2),
                },
                Key {
                    val: Some('2'),
                    pos: Pos::from(1, 2),
                },
                Key {
                    val: Some('3'),
                    pos: Pos::from(2, 2),
                },
                Key {
                    val: Some('0'),
                    pos: Pos::from(1, 3),
                },
                Key {
                    val: Some('A'),
                    pos: Pos::from(2, 3),
                },
            ],
        };

        let expected = Some(Pos::from(1, 1));
        let result = numeric_keys.next(Pos::from(1, 2), &Dir::North);
        assert_eq!(result, expected);

        let expected = None;
        let result = numeric_keys.next(Pos::from(1, 0), &Dir::North);
        assert_eq!(result, expected);

        let expected = None;
        let result = numeric_keys.next(Pos::from(2, 1), &Dir::East);
        assert_eq!(result, expected);

        let expected = Some(Pos::from(0, 1));
        let result = numeric_keys.next(Pos::from(1, 1), &Dir::West);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_positions_to_dir() {
        let expected = Dir::North;
        let cur_pos = Pos { x: 0, y: 1 };
        let next_pos = Pos { x: 0, y: 0 };
        let result = cur_pos.adjesant_to_dir(next_pos);
        assert_eq!(result, expected);

        let expected = Dir::East;
        let cur_pos = Pos { x: 0, y: 0 };
        let next_pos = Pos { x: 1, y: 0 };
        let result = cur_pos.adjesant_to_dir(next_pos);
        assert_eq!(result, expected);

        let expected = Dir::South;
        let cur_pos = Pos { x: 0, y: 0 };
        let next_pos = Pos { x: 0, y: 1 };
        let result = cur_pos.adjesant_to_dir(next_pos);
        assert_eq!(result, expected);

        let expected = Dir::West;
        let cur_pos = Pos { x: 1, y: 0 };
        let next_pos = Pos { x: 0, y: 0 };
        let result = cur_pos.adjesant_to_dir(next_pos);
        assert_eq!(result, expected);
    }
}
