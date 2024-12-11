use std::error::Error;
use std::fs;

struct TrailHead {
    x: i32,
    y: i32,
    score: i32,
}

fn main() -> Result<(), Box<dyn Error>> {
    let input: String = fs::read_to_string("input.txt")?;
    let parsed_input = parse_input(&input);
    let trail_heads = find_trail_head(&parsed_input);
    let ans = calculate(parsed_input, trail_heads);
    println!("{}", ans);
    Ok(())
}

fn parse_input(input: &String) -> Vec<Vec<u8>> {
    input.lines().map(|line|line.chars().map(|c|c.to_digit(10).unwrap() as u8).collect()).collect()
}

fn find_trail_head(map: &Vec<Vec<u8>>) -> Vec<TrailHead> {
    let mut trail_heads = vec![];
    map.iter().enumerate().for_each(|(i, val_vec)| val_vec.iter().enumerate().for_each(|(j, val)| {
        if *val == 0 {
            trail_heads.push(TrailHead{x: i as i32, y: j as i32, score: 0});
        }
    }));
    trail_heads
}

fn calculate(map: Vec<Vec<u8>>, mut trail_heads: Vec<TrailHead>) -> i32 {
    trail_heads.iter_mut().for_each(|head| {
        println!("iter");
        head.score = find_nine_count(&map, (head.x as usize, head.y as usize));
    });
    trail_heads.iter().map(|head|head.score).sum()
}

fn find_nine_count(map: &Vec<Vec<u8>>, start_pos: (usize, usize)) -> i32 {
    let (row, col) = start_pos;

    if row >= map.len() || col >= map[0].len() {
        return 0;
    }

    let current_value = map[row][col];
    if current_value == 9 {
        return 1;
    }

    let mut count = 0;

    if col < map[0].len() - 1 && map[row][col + 1] == current_value + 1 {
        count += find_nine_count(map, (row, col + 1));
    }
    if col > 0 && map[row][col - 1] == current_value + 1 {
        count += find_nine_count(map, (row, col - 1));
    }
    if row < map.len() - 1 && map[row + 1][col] == current_value + 1 {
        count += find_nine_count(map, (row + 1, col));
    }
    if row > 0 && map[row - 1][col] == current_value + 1 {
        count += find_nine_count(map, (row - 1, col));
    }
    count
}
