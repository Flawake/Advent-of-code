use std::collections::HashMap;
use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let input_map: String = fs::read_to_string("input_map.txt")?;
    let input_moves: String = fs::read_to_string("input_moves.txt")?;
    let mut parsed_input = parse_input(&input_map, &input_moves);
    let ans = calculate(&mut parsed_input.0, parsed_input.1);
    println!("{}", ans);
    Ok(())
}

fn parse_input(map: &String, moves: &String) -> (Vec<Vec<u8>>, Vec<u8>) {
    let map_vec = map
        .lines()
        .into_iter()
        .map(|map_line| {
            map_line
                .chars()
                .into_iter()
                .flat_map(|c| match c {
                    '#' => b"##".to_vec(),
                    'O' => b"[]".to_vec(),
                    '.' => b"..".to_vec(),
                    '@' => b"@.".to_vec(),
                    _ => unreachable!()
                })
                .collect()
        })
        .collect();

    let moves_vec = moves
        .bytes()
        .into_iter()
        .filter(|c| [b'^', b'<', b'v', b'>'].contains(c))
        .collect();
    (map_vec, moves_vec)
}

fn calculate(map: &mut Vec<Vec<u8>>, moves: Vec<u8>) -> usize {
    let mut player_pos = find_player_pos(&map);

    moves.iter().for_each(|&cur_move| match cur_move {
        b'^' => {
            player_pos = try_move_heigth(map, player_pos, (-1, 0));
        }
        b'>' => {
            player_pos = try_move_side(map, player_pos, (0, 1));
        }
        b'v' => {
            player_pos = try_move_heigth(map, player_pos, (1, 0));
        }
        b'<' => {
            player_pos = try_move_side(map, player_pos, (0, -1));
        }
        _ => unreachable!(),
    });

    map.iter().enumerate().fold(0, |acc_outer, (x, line)| {
        acc_outer + line.iter().enumerate().fold(0, |acc_inner, (y, &v)| {
            if v == b'[' {
                acc_inner + 100 * x + y
            }
            else {
                acc_inner
            }
        })
    })
}

fn find_player_pos(map: &Vec<Vec<u8>>) -> (usize, usize) {
    map.iter()
        .enumerate()
        .find_map(|(x, vec)| {
            vec.iter()
                .enumerate()
                .find_map(|(y, &c)| if c == b'@' { Some((x, y)) } else { None })
        })
        .unwrap()
}

fn try_move_side(map: &mut Vec<Vec<u8>>, player_pos: (usize, usize), dir: (isize, isize)) -> (usize, usize) {
    let mut new_pos = (player_pos.0 as isize + dir.0, player_pos.1 as isize + dir.1);
    if map[new_pos.0 as usize][new_pos.1 as usize] == b'#' {
        return (player_pos.0, player_pos.1);
    }
    if map[new_pos.0 as usize][new_pos.1 as usize] == b'.' {
        map[player_pos.0][player_pos.1] = b'.';
        map[new_pos.0 as usize][new_pos.1 as usize] = b'@';
        return (new_pos.0 as usize, new_pos.1 as usize);
    }
    //Check if there is space to the left/right
    while map[new_pos.0 as usize][new_pos.1 as usize] != b'.' {
        if map[new_pos.0 as usize][new_pos.1 as usize] == b'#' {
            return (player_pos.0, player_pos.1);
        }
        new_pos.0 += dir.0;
        new_pos.1 += dir.1;
        if !in_bounds(map, new_pos) {
            return (player_pos.0, player_pos.1);
        }
    }

    let mut new_pos = (player_pos.0 as isize + dir.0, player_pos.1 as isize + dir.1);
    //Actually move the boxes now
    let mut last_was_open = false;
    while map[new_pos.0 as usize][new_pos.1 as usize] != b'.' {
        if map[new_pos.0 as usize][new_pos.1 as usize] == b']' {
            map[new_pos.0 as usize][new_pos.1 as usize] = b'[';
            last_was_open = true;
        }
        else {
            last_was_open = false;
            map[new_pos.0 as usize][new_pos.1 as usize] = b']';
        }
        new_pos.0 += dir.0;
        new_pos.1 += dir.1;
    }
    if last_was_open {
        map[new_pos.0 as usize][new_pos.1 as usize] = b']';
    }
    else {
        map[new_pos.0 as usize][new_pos.1 as usize] = b'[';
    }
    map[(player_pos.0 as isize + dir.0) as usize][(player_pos.1 as isize + dir.1) as usize] = b'@';
    map[player_pos.0][player_pos.1] = b'.';
    return ((player_pos.0 as isize + dir.0) as usize, (player_pos.1 as isize + dir.1) as usize);
}

fn try_move_heigth(map: &mut Vec<Vec<u8>>, player_pos: (usize, usize), dir: (isize, isize)) -> (usize, usize) {
    let new_pos = (player_pos.0 as isize + dir.0, player_pos.1 as isize + dir.1);
    let mut connected = HashMap::new();
    if map[new_pos.0 as usize][new_pos.1 as usize] == b'#' {
        return (player_pos.0, player_pos.1);
    }
    if map[new_pos.0 as usize][new_pos.1 as usize] == b'.' {
        map[player_pos.0][player_pos.1] = b'.';
        map[new_pos.0 as usize][new_pos.1 as usize] = b'@';
        return (new_pos.0 as usize, new_pos.1 as usize);
    }
    if map[new_pos.0 as usize][new_pos.1 as usize] == b'[' {
        connected.insert((new_pos.0, new_pos.1), b'[');
        connected.insert((new_pos.0, new_pos.1 + 1), b']');
    }
    else {
        connected.insert((new_pos.0, new_pos.1), b']');
        connected.insert((new_pos.0, new_pos.1 - 1), b'[');
    }
    let mut new_inserted = true;
    //Find all connecting boxes
    while new_inserted {
        new_inserted = false;
        let mut new_connections = Vec::new();
        connected.iter().for_each(|v| {
            if !connected.contains_key(&(v.0.0 + dir.0, v.0.1 + dir.1)) && [b'[', b']'].contains(&map[(v.0.0 + dir.0) as usize][(v.0.1 + dir.1) as usize]) {
                if map[(v.0.0 + dir.0) as usize][(v.0.1 + dir.1) as usize] == b'[' {
                    new_connections.push((((v.0.0 + dir.0), (v.0.1 + dir.1)), b'['));
                    new_connections.push((((v.0.0 + dir.0), (v.0.1 + dir.1 + 1)), b']'));
                }
                else {
                    new_connections.push((((v.0.0 + dir.0), (v.0.1 + dir.1)), b']'));
                    new_connections.push((((v.0.0 + dir.0), (v.0.1 + dir.1 - 1)), b'['));
                }
                new_inserted = true;
            }
        });
        for (pos, val) in new_connections {
            connected.insert(pos, val);
        }
    }
    //Check if one of the boxes collides with a wall
    if connected.iter().any(|val| {
        if map[(val.0.0 + dir.0) as usize][(val.0.1 + dir.1) as usize] == b'#' {
            return true;
        }
        return false;
    }) {
        return (player_pos.0, player_pos.1);
    }
    //remove all boxes from their original position
    connected.iter().for_each(|v| {
        map[(v.0.0) as usize][(v.0.1) as usize] = b'.';
    });
    //place all boxes one up
    connected.iter().for_each(|v| {
        map[(v.0.0 + dir.0) as usize][(v.0.1 + dir.1) as usize] = *v.1;
    });
    //move the player up
    map[(player_pos.0 as isize + dir.0) as usize][(player_pos.1 as isize + dir.1) as usize] = b'@';
    map[player_pos.0][player_pos.1] = b'.';
    return ((player_pos.0 as isize + dir.0) as usize, (player_pos.1 as isize + dir.1) as usize);
}

fn in_bounds(map: &Vec<Vec<u8>>, pos: (isize, isize)) -> bool {
    if pos.0 < 0 || pos.1 < 0 || pos.0 > map.len() as isize - 1 || pos.1 > map[0].len() as isize - 1 {
        return false;
    }
    true
}

#[allow(dead_code)]
fn print_map(map: &Vec<Vec<u8>>) {
    map.iter().for_each(|line| {
        line.iter().for_each(|char| {
            match char {
                b'.' => print!("."),
                b'[' => print!("["),
                b']' => print!("]"),
                b'@' => print!("@"),
                b'#' => print!("#"),
                _ => unreachable!(),
            }
        });
        println!();
    });
}
