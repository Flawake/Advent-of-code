use std::error::Error;
use std::{fs, vec};

#[derive(Debug)]
struct Robot {
    pos: (i32, i32),
    velocity: (i32, i32),
}

fn main() -> Result<(), Box<dyn Error>> {
    let input: String = fs::read_to_string("input.txt")?;
    let parsed_input = parse_input(&input);
    let ans = calculate(parsed_input);
    println!("{}", ans);
    Ok(())
}

fn parse_input(input: &String) -> Vec<Robot> {
    input.lines().into_iter().map(|s| {
        let v: Vec<&str> = s.split_whitespace().collect();
        let v1: Vec<&str> = v[0].split(",").collect();
        let v2: Vec<&str> = v[1].split(",").collect();
        Robot {
            pos: (v1[0][2..].trim().parse::<i32>().unwrap(), v1[1].trim().parse::<i32>().unwrap()),
            velocity: (v2[0][2..].trim().parse::<i32>().unwrap(), v2[1].trim().parse::<i32>().unwrap()),
        }
    }).collect()
}

fn calculate(robots: Vec<Robot>) -> u32 {
    let mut map= vec![vec![0;101];104];
    robots.iter().for_each(|robot| {
        let x = ((robot.pos.0 + (robot.velocity.0 * 100)) % 101 + 101) % 101;
        let y = ((robot.pos.1 + (robot.velocity.1 * 100)) % 103 + 103) % 103;
        map[y as usize][x as usize] = map[y as usize][x as usize] + 1;
    });
    
    let mut total1 = 0;
    let mut total2 = 0;
    let mut total3 = 0;
    let mut total4 = 0;

    for x in 0..51 {
        for y in 0..50 {
            total1 += map[x][y];
        }
    }

    for x in 52..103 {
        for y in 0..50 {
            total2 += map[x][y];
        }
    }

    for x in 0..51 {
        for y in 51..101 {
            total3 += map[x][y];
        }
    }

    for x in 52..103 {
        for y in 51..101 {
            total4 += map[x][y];
        }
    }
    return total1 * total2 * total3 * total4;
}
