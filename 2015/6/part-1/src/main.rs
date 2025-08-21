enum TokenType<'a> {
    Literal(&'a str),
    Int(u32),
}

struct Tokens<'a> (Vec<TokenType<'a>>);

impl<'a> From<&'a str> for TokenType<'a> {
    fn from(value: &'a str) -> Self {
        if let Ok(val) = value.parse::<u32>() {
            return TokenType::Int(val)
        }
        else {
            return TokenType::Literal(value)
        }
    }
}

impl<'a> FromIterator for Tokens<'a> {
    fn from_iter<T: IntoIterator<Item = A>>(iter: T) -> Self {
        todo!()
    }
}

fn main() {
    let input = include_str!("input.txt");
    println!("{}", calculate(input));
}

fn tokenize(input: &'static str) -> Tokens {
    input.split_whitespace().map(|str| str.into()).collect()
}

fn calculate(input: &'static str) -> u32 {
    input
        .lines()
        .fold(0, |acc, line| acc + is_string_nice(line) as u32)
}
