use std::error::Error;
use std::{fs, vec};

fn main() -> Result<(), Box<dyn Error>> {
    let input: String = fs::read_to_string("input.txt")?;
    let parsed_input = parse_input(&input);
    let ans = calculate(parsed_input);
    println!("{}", ans);
    Ok(())
}

fn parse_input(input: &String) -> Vec<Vec<i32>> {
    let ans = input.lines().into_iter().map(|line|line.chars().into_iter().map(|c| match c {
        '.' => 0,
        '#' => 1,
        '^' => 2,
        _ => 3,
    }
 ).collect::<Vec<i32>>()).collect::<Vec<Vec<i32>>>();
    ans
}

#[derive(Debug, PartialEq, Eq)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

fn calculate(input: Vec<Vec<i32>>) -> i32 {
    let mut walked_area: Vec<Vec<i32>> = input.iter()
        .map(|row| vec![0; row.len()])
        .collect();

    let mut guard_pos: (i32, i32) = (0, 0);
    
    for (i, row) in input.iter().enumerate() {
        if let Some(j) = row.iter().position(|&num| num == 2) {
            guard_pos = (i as i32, j as i32);
            break;
        }
    }

    let mut dir = Direction::Up;

    loop {
        if dir == Direction::Right {
            // Check if within bounds before accessing
            if guard_pos.1 + 1 < input[0].len() as i32 {
                if input[guard_pos.0 as usize][(guard_pos.1 + 1) as usize] == 1 {
                    guard_pos = (guard_pos.0 + 1, guard_pos.1);
                    dir = Direction::Down;
                }
                else {
                    guard_pos = (guard_pos.0, guard_pos.1 + 1);
                }
            }
            else {
                break;
            }
            walked_area[guard_pos.0 as usize][guard_pos.1 as usize] = 1;
        } else if dir == Direction::Down {
            if guard_pos.0 + 1 < input.len() as i32 {
                if input[(guard_pos.0 + 1) as usize][guard_pos.1 as usize] == 1 {
                    guard_pos = (guard_pos.0, guard_pos.1 - 1);
                    dir = Direction::Left;
                }
                else {
                    guard_pos = (guard_pos.0 + 1, guard_pos.1);
                }
            }
            else {
                break;
            }
            walked_area[guard_pos.0 as usize][guard_pos.1 as usize] = 1;
        } else if dir == Direction::Left {
            if guard_pos.1 > 0 {
                if input[guard_pos.0 as usize][(guard_pos.1 - 1) as usize] == 1 {
                    guard_pos = (guard_pos.0 - 1, guard_pos.1);
                    dir = Direction::Up;
                }
                else {
                    guard_pos = (guard_pos.0, guard_pos.1 - 1);
                }
            }
            else {
                break;
            }
            walked_area[guard_pos.0 as usize][guard_pos.1 as usize] = 1;
        } else if dir == Direction::Up {
            if guard_pos.0 > 0 {
                if input[(guard_pos.0 - 1) as usize][guard_pos.1 as usize] == 1 {
                    guard_pos = (guard_pos.0, guard_pos.1 + 1);
                    dir = Direction::Right;
                }
                else {
                    guard_pos = (guard_pos.0 - 1, guard_pos.1);
                }
            }
            else {
                break;
            }
            walked_area[guard_pos.0 as usize][guard_pos.1 as usize] = 1;
        }
    }
    //130, 130
    println!("{:?}", walked_area);

    walked_area.iter().flat_map(|v| v.iter()).copied().sum::<i32>()
}