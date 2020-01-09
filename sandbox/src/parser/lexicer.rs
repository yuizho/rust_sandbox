#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Loc(usize, usize);
impl Loc {
    fn merge(&self, other_loc: &Loc) -> Loc {
        use std::cmp::{max, min};
        Loc(min(self.0, other_loc.0), max(self.1, other_loc.1))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Annot<T> {
    pub value: T,
    pub loc: Loc,
}
impl<T> Annot<T> {
    fn new(value: T, loc: Loc) -> Self {
        Self { value, loc }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TokenType {
    Number(u32),
    Hour,
    Minute,
    Scond,
}
pub type Token = Annot<TokenType>;
impl Token {
    fn number(num: u32, loc: Loc) -> Self {
        Self::new(TokenType::Number(num), loc)
    }

    fn hour(loc: Loc) -> Self {
        Self::new(TokenType::Hour, loc)
    }

    fn minute(loc: Loc) -> Self {
        Self::new(TokenType::Minute, loc)
    }

    fn second(loc: Loc) -> Self {
        Self::new(TokenType::Scond, loc)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum LexErrorType {
    InvalidChar(char),
    Eof,
}
pub type LexError = Annot<LexErrorType>;
impl LexError {
    fn invalid_char(c: char, loc: Loc) -> Self {
        LexError::new(LexErrorType::InvalidChar(c), loc)
    }

    fn eof(loc: Loc) -> Self {
        LexError::new(LexErrorType::Eof, loc)
    }
}

pub fn lex(input: &str) -> Result<Vec<Token>, LexError> {
    let mut tokens = Vec::new();
    let input = input.as_bytes();
    let mut pos = 0;
    macro_rules! lex_a_token {
        ($lexer:expr) => {{
            let (tok, p) = $lexer?;
            tokens.push(tok);
            pos = p;
        }};
    }
    while pos < input.len() {
        match input[pos] {
            b'0'...b'9' => lex_a_token!(lex_number(input, pos)),
            b'h' => lex_a_token!(lex_hour(input, pos)),
            b'm' => lex_a_token!(lex_minute(input, pos)),
            b's' => lex_a_token!(lex_second(input, pos)),
            b => return Err(LexError::invalid_char(b as char, Loc(pos, pos + 1))),
        }
    }
    Ok(tokens)
}

fn lex_number(input: &[u8], mut potision: usize) -> Result<(Token, usize), LexError> {
    use std::str::from_utf8;
    let start = potision;
    while potision < input.len() && b"1234567890".contains(&input[potision]) {
        potision += 1;
    }
    // TODO: Error Handling
    let n = from_utf8(&input[start..potision]).unwrap().parse().unwrap();
    Ok((Token::number(n, Loc(start, potision)), potision))
}

fn lex_hour(input: &[u8], start: usize) -> Result<(Token, usize), LexError> {
    consume_byte(input, start, b'h').map(|(_, end)| (Token::hour(Loc(start, end)), end))
}

fn lex_minute(input: &[u8], start: usize) -> Result<(Token, usize), LexError> {
    consume_byte(input, start, b'm').map(|(_, end)| (Token::minute(Loc(start, end)), end))
}

fn lex_second(input: &[u8], start: usize) -> Result<(Token, usize), LexError> {
    consume_byte(input, start, b's').map(|(_, end)| (Token::second(Loc(start, end)), end))
}

fn consume_byte(input: &[u8], position: usize, char_byte: u8) -> Result<(u8, usize), LexError> {
    if input.len() <= position {
        return Err(LexError::eof(Loc(position, position)));
    }
    if input[position] != char_byte {
        return Err(LexError::invalid_char(
            input[position] as char,
            Loc(position, position + 1),
        ));
    }
    Ok((char_byte, position + 1))
}
