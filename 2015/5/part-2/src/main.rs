use itertools::Itertools;

fn main() {
    let input = include_str!("input.txt");
    println!("{}", calculate(input));
}

fn is_string_nice(string: &str) -> bool {
    let chars: Vec<char> = string.chars().collect();

    if !chars.windows(2).enumerate().any(|(i, window)| {
        chars
            .iter()
            .skip(i + 2)
            .tuple_windows()
            .any(|(&c, &d)| window[0] == c && window[1] == d)
    }) {
        return false;
    }
    if !string.chars().tuple_windows().any(|(a, _, c)| a == c) {
        return false;
    }
    true
}

fn calculate(input: &'static str) -> u32 {
    input
        .lines()
        .fold(0, |acc, line| acc + is_string_nice(line) as u32)
}
