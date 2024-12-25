use std::collections::BTreeMap;
use std::error::Error;
use std::fs;

#[derive(Debug)]
struct Lock {
    lock: Vec<u8>,
}

#[derive(Debug)]
struct Key {
    key: Vec<u8>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let input: String = fs::read_to_string("input.txt")?;
    let parsed_input = parse_input(&input);
    let ans = calculate(&parsed_input.0, &parsed_input.1);
    println!("{}", ans);
    Ok(())
}

fn parse_input(input: &String) -> (Vec<Lock>, Vec<Key>) {
    let items: Vec<&str> = input.split("\n\r\n").into_iter().map(|string|string.trim()).collect();

    let mut locks = Vec::new();
    let mut keys = Vec::new();

    items.into_iter().for_each(|item| {
        
        let mut values = BTreeMap::new();

        item.lines().into_iter().for_each(|line| {
            line.chars().into_iter().enumerate().for_each(|(i, char)| {
                if char == '#' {
                    values.entry(i).and_modify(|val| *val += 1).or_insert(0);
                }
            })
        });

        let values: Vec<u8> = values.iter().map(|v|*v.1).collect();
        if item.chars().nth(0) == Some('#') { 
            locks.push(Lock {
                lock: values,
            });
        }
        else {
            keys.push(Key {
                key: values,
            });
        }
    });
    (locks, keys)
}

fn calculate(locks: &Vec<Lock>, keys: &Vec<Key>) -> u32 {
    let mut total = 0;
    locks.iter().for_each(|lock| {
        keys.iter().for_each(|key| {
            if lock.lock.iter().enumerate().all(|(i, &val)| val + key.key[i] <= 5) {
                total += 1;
            };
        });
    });
    total
}