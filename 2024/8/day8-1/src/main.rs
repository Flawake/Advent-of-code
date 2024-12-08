use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let input: String = fs::read_to_string("input.txt")?;
    let mut parsed_input = parse_input(&input);
    let ans = calculate(&mut parsed_input);
    println!("{}", ans);
    Ok(())
}

fn parse_input(input: &String) -> Vec<Vec<u8>> {
    let ans = input.lines().into_iter().map(|line|line.chars().into_iter().map(|c| match c {
        '.' => 0,
        _ => {c as u8},
    }
 ).collect::<Vec<u8>>()).collect::<Vec<Vec<u8>>>();
    ans
}

fn calculate(input: &mut Vec<Vec<u8>>) -> i32 {
    let mut total = 0;
    let mut new_map = input.clone();
    input.clone().iter().enumerate().for_each(|(i, val_vec_inner)|val_vec_inner.iter().enumerate().for_each(|(j, val_inner)|{
        input.clone().iter().enumerate().for_each(|(k, val_vec)|val_vec.iter().enumerate().for_each(|(l, val)|{
            let val_inner = val_inner.clone();
            let val = val.clone();
            if val == val_inner && val != 0 && val != '#' as u8 && (i, j) != (k, l) {
                
                let i_32 = i as i32;
                let j_32 = j as i32;
                let k_32 = k as i32;
                let l_32 = l as i32;


                //Right side
                if j_32 > l_32 && (j_32 - l_32).abs() + j_32.max(l_32) < input[0].len() as i32 {
                    //under side
                    if i_32 > k_32 && (i_32 - k_32).abs() + i_32 < input.len() as i32 {
                        if new_map[((i_32 - k_32).abs() + i_32) as usize][((j_32 - l_32).abs() + j_32.max(l_32)) as usize] != '#' as u8 {
                            new_map[((i_32 - k_32).abs() + i_32) as usize][((j_32 - l_32).abs() + j_32.max(l_32)) as usize] = '#' as u8;
                            total += 1;
                        }
                    }
                    //upper side
                    else if i_32 < k_32 && i_32 - (k_32 - i_32).abs() >= 0{
                        if new_map[(i_32 - (k_32 - i_32).abs()) as usize][((j_32 - l_32).abs() + j_32.max(l_32)) as usize] != '#' as u8 {
                            new_map[(i_32 - (k_32 - i_32).abs()) as usize][((j_32 - l_32).abs() + j_32.max(l_32)) as usize] = '#' as u8;
                            total += 1;
                        }
                    }
                }
                //Left side
                if j_32 < l_32 && j_32.min(l_32) - (j_32 - l_32).abs() >= 0 {
                    //under side
                    if i_32 > k_32 && (i_32 - k_32).abs() + i_32 < input.len() as i32 {
                        if new_map[((i_32 - k_32).abs() + i_32) as usize][(j_32.min(l_32) - (j_32 - l_32).abs()) as usize] != '#' as u8 {
                            new_map[((i_32 - k_32).abs() + i_32) as usize][(j_32.min(l_32) - (j_32 - l_32).abs()) as usize] = '#' as u8;
                            total += 1;
                        }
                    }
                    //upper side
                    else if i_32 < k_32 && i_32 - (k_32 - i_32).abs() >= 0{
                        if new_map[(i_32 - (k_32 - i_32).abs()) as usize][(j_32.min(l_32) - (j_32 - l_32).abs()) as usize] != '#' as u8 {
                            new_map[(i_32 - (k_32 - i_32).abs()) as usize][(j_32.min(l_32) - (j_32 - l_32).abs()) as usize] = '#' as u8;
                            total += 1;
                        }
                    }
                }
            }
        }));
    }));
    println!("{:?}", input);
    total
}