use std::{fs, os::windows::fs::symlink_dir};

fn main() {
    let input: String = fs::read_to_string("input.txt").unwrap();
    let data = parse_input(input);
    let ans = calculate_answer(data);
    println!("{:?}", ans);
}

fn calculate_answer(input: Vec<(u32, u32)>) -> u32 {
    input.iter().map(|x|x.0 * x.1).sum()
}

fn parse_input(input: String) -> Vec<(u32, u32)> {
    let mut sums = vec![];
    let mut s = 0;

    let mut do_start = 0;

    let mut do_use = true;

    for (i, val) in input.chars().enumerate() {
        if val == 'd' {
            do_start = i;
        }
        if val == ')' {
            if input[do_start..i + 1].starts_with("do()") {
                println!("{}", input[do_start..i + 1].to_string());
                do_start = i + 1;
                do_use = true;
                continue;
            }
            else if input[do_start..i + 1].starts_with("don't()") {
                do_use = false;
                do_start = i + 1;
                continue;
            }
        }
        
        if val == 'm' {
            s = i;
        }
        if val == ')' {
            if do_use == false {
                s = i + 1;
                continue;
            }
            if let Some(values) = find_mul(&input[s..i + 1]) {
                sums.push(values);
                s = i + 1
            }
        }
    }
    return sums;
}

fn find_mul(substr: &str) -> Option<(u32, u32)> {
    let x;
    let y;

    let string: String = substr.into();

    if !substr.starts_with("mul(") {
        return  None;
    }

    let mut s = 4;

    loop {
        if string.chars().nth(s).unwrap().to_digit(10).is_some() {
            s += 1
        }
        else if string.chars().nth(s).unwrap() == ',' {
            x = string[4..s].to_string().parse::<u32>().unwrap();
            s += 1;
            break;
        }
        else {
            return None
        }
    }

    let mut e = s;

    loop {
        if string.chars().nth(e).unwrap().to_digit(10).is_some() {
            e += 1
        }
        else if string.chars().nth(e).unwrap() == ')' {
            y = string[s..e].to_string().parse::<u32>().unwrap();
            break;
        }
        else {
            return None
        }
    }

    Some((x, y))
}