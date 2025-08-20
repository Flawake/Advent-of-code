fn main() {
    let input = include_str!("input.txt");
    println!("{}", calculate(input));
}

const VOWELS: [char; 5] = ['a', 'e', 'i', 'o', 'u'];
const FORBIDDEN_STRINGS: [&str; 4] = ["ab", "cd", "pq", "xy"];

fn is_string_nice(string: &str) -> bool {
    if string
        .chars()
        .into_iter()
        .fold(0, |acc, c| acc + VOWELS.contains(&c) as u8)
        < 3
    {
        return false;
    }
    if !string
        .chars()
        .zip(string.chars().skip(1))
        .any(|(a, b)| a == b)
    {
        return false;
    }
    if FORBIDDEN_STRINGS.iter().any(|s| string.contains(s)) {
        return false;
    }
    true
}

fn calculate(input: &'static str) -> u32 {
    input
        .lines()
        .fold(0, |acc, line| acc + is_string_nice(line) as u32)
}
