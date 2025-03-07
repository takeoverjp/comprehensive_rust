use std::iter::Peekable;
use std::result::Result;
use std::str::Chars;
use thiserror::Error;

/// 算術演算子。
#[derive(Debug, PartialEq, Clone, Copy)]
enum Op {
    Add,
    Sub,
}

/// 式言語のトークン
#[derive(Debug, PartialEq)]
enum Token {
    Number(String),
    Identifier(String),
    Operator(Op),
}

/// 式言語の式
#[derive(Debug, PartialEq)]
enum Expression {
    /// 変数への参照。
    Var(String),
    /// リテラル数値。
    Number(u32),
    /// バイナリ演算。
    Operation(Box<Expression>, Op, Box<Expression>),
}

fn tokenize(input: &str) -> Tokenizer {
    return Tokenizer(input.chars().peekable());
}

struct Tokenizer<'a>(Peekable<Chars<'a>>);

impl<'a> Tokenizer<'a> {
    fn collect_number(&mut self, first_char: char) -> Token {
        let mut num = String::from(first_char);
        while let Some(&c @ '0'..='9') = self.0.peek() {
            num.push(c);
            self.0.next();
        }
        Token::Number(num)
    }

    fn collect_identifier(&mut self, first_char: char) -> Token {
        let mut ident = String::from(first_char);
        while let Some(&c @ ('a'..='z' | '_' | '0'..='9')) = self.0.peek() {
            ident.push(c);
            self.0.next();
        }
        Token::Identifier(ident)
    }
}

impl<'a> Iterator for Tokenizer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Token> {
        let c = self.0.next()?;
        match c {
            '0'..='9' => Some(self.collect_number(c)),
            'a'..='z' => Some(self.collect_identifier(c)),
            '+' => Some(Token::Operator(Op::Add)),
            '-' => Some(Token::Operator(Op::Sub)),
            _ => panic!("Unexpected character {c}"),
        }
    }
}

#[derive(Debug, Error)]
enum ParseError {
    #[error("Unexpected end of input")]
    UnexpectedEof,
    #[error("Invalid 32-bit integer")]
    InvalidInt,
    #[error("Unexpected token")]
    UnexpectedToken(String),
}

fn parse(input: &str) -> Result<Expression, ParseError> {
    let mut tokens = tokenize(input);

    fn parse_expr(tokens: &mut Tokenizer) -> Result<Expression, ParseError> {
        let tok = tokens.next().ok_or(ParseError::UnexpectedEof)?;
        let expr = match tok {
            Token::Number(num) => {
                let v = num.parse().map_err(|_| ParseError::InvalidInt)?;
                Expression::Number(v)
            }
            Token::Identifier(ident) => Expression::Var(ident),
            Token::Operator(_) => return Err(ParseError::UnexpectedToken(format!("{tok:?}"))),
        };
        // バイナリ演算が存在する場合はパースします。
        match tokens.next() {
            None => Ok(expr),
            Some(Token::Operator(op)) => Ok(Expression::Operation(
                Box::new(expr),
                op,
                Box::new(parse_expr(tokens)?),
            )),
            Some(tok) => Err(ParseError::UnexpectedToken(format!("{tok:?}"))),
        }
    }

    parse_expr(&mut tokens)
}

fn main() {
    let expr = parse("10+foo+20-30");
    println!("{expr:?}");
}
