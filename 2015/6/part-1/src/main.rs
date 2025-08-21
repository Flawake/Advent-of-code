#[derive(PartialEq, Eq)]
enum Keyword {
    Turn,
    Toggle,
    On,
    Off,
    Through,
}

impl TryFrom<&str> for Keyword {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "Turn" => Ok(Keyword::Turn),
            "Toggle" => Ok(Keyword::Toggle),
            "On" => Ok(Keyword::On),
            "Off" => Ok(Keyword::Off),
            "Through" => Ok(Keyword::Through),
            _ => Err(())
        }
    }
}

#[derive(PartialEq, Eq)]
enum TokenType<'a> {
    Keyword(Keyword),
    Int(u32),
    Comma,
    BlankChar,
    Literal(&'a str),
}

struct Tokens<'a> (Vec<TokenType<'a>>);

impl<'a> Tokens<'a> {
    fn new() -> Self {
        Tokens(Vec::new())
    }
}

impl<'a> TokenType<'a> {
    fn try_make_tokentype(token: &'a str) -> Result<Self, ()> {
        if let Ok(keyword) = Keyword::try_from(token) {
            return Ok(TokenType::Keyword(keyword));
        }
        Err(())
    }
}

impl<'a> TryFrom<&'a str> for TokenType<'a> {
    type Error = ();
    fn try_from(value: &'a str) -> Result<TokenType<'_>, Self::Error> {
        if let Ok(keyword) = Keyword::try_from(value) {
            return Ok(TokenType::Keyword(keyword));
        }
        if let Ok(val_u32) = value.parse::<u32>() {
            return Ok(TokenType::Int(val_u32));
        }
        if value == "," {
            return Ok(TokenType::Comma);
        }
        if matches!(value, " " | "\n" | "\r" | "\t" | "\r\n") {
            return Ok(TokenType::BlankChar);
        }
        return Ok(Self::Literal(value));
    }
}

impl<'a> FromIterator<TokenType<'a>> for Tokens<'a> {
    fn from_iter<T: IntoIterator<Item = TokenType<'a>>>(iter: T) -> Self {
        iter.into_iter().collect()
    }
}

fn main() {
    let input = include_str!("input.txt");
    println!("{}", calculate(input));
}

fn tokenize(input: &'static str) -> Tokens {
    let mut start_pointer = 0;
    let mut tokens = Tokens::new();
    for (i, c) in input.char_indices() {
        // We should convert the strslice to a token and increase the start pointer if one of these conditions are met, unless otherwise specified
        // current char is a blankchar
        // current char is a comma
        if let Ok(token) = input[i..i].try_into() {
            if token == TokenType::BlankChar || token == TokenType::Comma  {
                let current_string = &input[start_pointer..i - 1];
                if let Ok(first_token) = current_string.try_into() {
                    start_pointer = i + 1;
                    tokens.0.push(first_token);
                    if i == input.len() {
                        tokens.0.push(token);
                    }
                }
            }
        }
    }
    tokens
}

fn calculate(input: &'static str) -> u32 {
    0
}
