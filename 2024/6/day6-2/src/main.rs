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

    let mut guard_pos: (i32, i32) = (0, 0);
    
    for (i, row) in input.iter().enumerate() {
        if let Some(j) = row.iter().position(|&num| num == 2) {
            guard_pos = (i as i32, j as i32);
            break;
        }
    }

    let mut count = 0;

    for (i, val_vec) in input.iter().enumerate() {
        for (j, val_int) in val_vec.iter().enumerate() {
            if val_int != &0 {
                continue;
            }

            let mut input_blocked = input.clone();
            input_blocked[i][j] = 1;
            if simulate_route(input_blocked, guard_pos) {
                count += 1
            }
        }
    } 
    count
}

fn simulate_route(input: Vec<Vec<i32>>, mut guard_pos: (i32, i32)) -> bool {
    //130, 130

    let mut turn_points: Vec<(i32, i32)> = vec![];

    let mut t = (0, 0);

    let mut dir = Direction::Up;

    let mut edit_next = false;
    loop {
        if dir == Direction::Right {
            // Check if within bounds before accessing
            if guard_pos.1 + 1 < input[0].len() as i32 {
                if input[guard_pos.0 as usize][(guard_pos.1 + 1) as usize] == 1 {
                    if turn_points.contains(&(guard_pos.0, guard_pos.1)) {
                        return true
                    }
                    t = guard_pos;
                    edit_next = true;
                    dir = Direction::Down;
                }
                else {
                    if edit_next {
                        edit_next = false;
                        turn_points.push((t.0, t.1));
                    }
                    guard_pos = (guard_pos.0, guard_pos.1 + 1);
                }
            }
            else {
                break;
            }
        } else if dir == Direction::Down {
            if guard_pos.0 + 1 < input.len() as i32 {
                if input[(guard_pos.0 + 1) as usize][guard_pos.1 as usize] == 1 {
                    if turn_points.contains(&(guard_pos.0, guard_pos.1)) {
                        return true
                    }
                    t = guard_pos;
                    edit_next = true;
                    dir = Direction::Left;
                }
                else {
                    if edit_next {
                        edit_next = false;
                        turn_points.push((t.0, t.1));
                    }
                    guard_pos = (guard_pos.0 + 1, guard_pos.1);
                }
            }
            else {
                break;
            }
        } else if dir == Direction::Left {
            if guard_pos.1 > 0 {
                if input[guard_pos.0 as usize][(guard_pos.1 - 1) as usize] == 1 {
                    if turn_points.contains(&(guard_pos.0, guard_pos.1)) {
                        return true
                    }
                    t = guard_pos;
                    edit_next = true;
                    dir = Direction::Up;
                }
                else {
                    if edit_next {
                        edit_next = false;
                        turn_points.push((t.0, t.1));
                    }
                    guard_pos = (guard_pos.0, guard_pos.1 - 1);
                }
            }
            else {
                break;
            }
        } else if dir == Direction::Up {
            if guard_pos.0 > 0 {
                if input[(guard_pos.0 - 1) as usize][guard_pos.1 as usize] == 1 {
                    t = guard_pos;
                    edit_next = true;
                    dir = Direction::Right;
                }
                else {
                    if edit_next {
                        edit_next = false;
                        turn_points.push((t.0, t.1));
                    }
                    guard_pos = (guard_pos.0 - 1, guard_pos.1);
                }
            }
            else {
                break;
            }
        }
    }

    false
}