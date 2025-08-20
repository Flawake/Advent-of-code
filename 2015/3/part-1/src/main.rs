use std::collections::HashSet;

enum Direction {
    North,
    East,
    South,
    West,
}

impl From<char> for Direction {
    fn from(c: char) -> Direction {
        match c {
            '^' => Direction::North,
            '>' => Direction::East,
            'v' => Direction::South,
            '<' => Direction::West,
            _ => panic!("Invalid direction character: {}", c),
        }
    }
}

#[derive(Default, Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn move_position(&mut self, dir: Direction) {
        match dir {
            Direction::North => self.x += 1,
            Direction::East => self.y += 1,
            Direction::South => self.x -= 1,
            Direction::West => self.y -= 1,
        }
    }
}

fn main() {
    let input = include_str!("input.txt");

    println!("{}", calculate(input));
}

fn calculate(input: &'static str) -> u32 {
    let mut santa_pos = Position::default();
    let mut visited_homes = HashSet::new();
    visited_homes.insert(santa_pos);
    input.chars().for_each(|c| {
        santa_pos.move_position(c.into());
        visited_homes.insert(santa_pos);
    });
    visited_homes.len() as u32
}
