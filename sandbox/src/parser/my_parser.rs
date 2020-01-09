use crate::lexicer::{Token, TokenType};
use std::io;
use std::iter::Peekable;

mod lexicer;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum ParseError {
    UnexpectedToken(Token),
    NotExpresion(Token),
    NotOperator(Token),
    UnclosedOpenParen(Token),
    RedundantExpression(Token),
    Eof,
}

fn parse(tokens: Vec<Token>) -> Result<u32, ParseError> {
    let mut tokens = tokens.into_iter().peekable();
    let ret = parse_expr(&mut tokens)?;
    match tokens.next() {
        Some(tok) => Err(ParseError::RedundantExpression(tok)),
        None => Ok(ret),
    }
}

fn parse_expr<Tokens>(tokens: &mut Peekable<Tokens>) -> Result<u32, ParseError>
where
    Tokens: Iterator<Item = Token>,
{
    parse_expr1(tokens)
}

fn parse_expr1<Tokens>(tokens: &mut Peekable<Tokens>) -> Result<u32, ParseError>
where
    Tokens: Iterator<Item = Token>,
{
    let mut number = parse_number(tokens)?;
    match tokens.next().map(|token| token.value) {
        Some(TokenType::Scond) => Ok(number),
        Some(TokenType::Minute) => Ok(number * 60),
        Some(TokenType::Hour) => Ok(number * 60 * 60),
        _ => Ok(number),
    }
}

fn parse_number<Tokens>(tokens: &mut Peekable<Tokens>) -> Result<u32, ParseError>
where
    Tokens: Iterator<Item = Token>,
{
    tokens
        .next()
        .ok_or(ParseError::Eof)
        .and_then(|token| match token.value {
            TokenType::Number(n) => Ok(n),
            _ => Ok(0), // TODO: ErrorHandling
        })
}

fn prompt(s: &str) -> io::Result<()> {
    use std::io::{stdout, Write};
    let stdout = stdout();
    let mut stdout = stdout.lock();
    stdout.write(s.as_bytes())?;
    stdout.flush()
}

fn main() {
    use std::io::{stdin, BufRead, BufReader};

    let stdin = stdin();
    let stdin = stdin.lock();
    let stdin = BufReader::new(stdin);
    let mut lines = stdin.lines();

    loop {
        prompt("> ").unwrap();
        if let Some(Ok(line)) = lines.next() {
            let tokens = lexicer::lex(&line).unwrap();
            let result = parse(tokens);
            println!("{}", result.expect("error "));
        } else {
            break;
        }
    }
}
