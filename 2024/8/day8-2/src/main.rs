use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input.txt")?;
    let mut parsed_input = parse_input(&input);
    let ans = calculate(&mut parsed_input);
    println!("{}", ans);
    Ok(())
}

fn parse_input(input: &String) -> Vec<Vec<u8>> {
    let ans = input
        .lines()
        .into_iter()
        .map(|line| {
            line.chars()
                .into_iter()
                .map(|c| match c {
                    '.' => 0,
                    _ => c as u8,
                })
                .collect()
        })
        .collect();
    ans
}

fn calculate(input: &mut Vec<Vec<u8>>) -> i32 {
    let mut new_map = input.clone();

    input.iter().enumerate().for_each(|(i, row)| {
        row.iter().enumerate().for_each(|(j, &val)| {
            if val == 0 as u8 {
                return;
            }

            input.iter().enumerate().for_each(|(k, other_row)| {
                other_row.iter().enumerate().for_each(|(l, &val_inner)| {
                    if val != val_inner || (i, j) == (k, l) {
                        return;
                    }
                    
                    let (i, j, k, l) = (i as i32, j as i32, k as i32, l as i32);
                    let (dx, dy) = (k - i, l - j);

                    let mut x = i;
                    let mut y = j;

                    loop {
                        x += dx;
                        y += dy;

                        if !in_bounds(y, x, input.len() as i32, input[0].len() as i32) {
                            break;
                        }

                        let nx = x as usize;
                        let ny = y as usize;

                        if new_map[nx][ny] != '#' as u8 {
                            new_map[nx][ny] = '#' as u8;
                        }
                    }
                });
            });
        });
    });

    for row in &new_map {
        println!("{:?}", row);
    }

    new_map.iter().fold(0, |acc, x| {
        x.iter().filter(|&&v| v == '#' as u8).count() as i32 + acc
    })
}

fn in_bounds(y: i32, x: i32, height: i32, width: i32) -> bool {
    y >= 0 && x >= 0 && y < height && x < width
}
