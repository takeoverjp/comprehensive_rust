use anyhow::anyhow;
use anyhow::Context;
use anyhow::Result;
use std::iter::Peekable;
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

#[derive(Debug, Error)]
enum TokenizeError {
    #[error("Unexpected character '{0}'")]
    UnexpectedChar(char),
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
    type Item = Result<Token>;

    fn next(&mut self) -> Option<Result<Token>> {
        let c = self.0.next()?;
        match c {
            '0'..='9' => Some(Ok(self.collect_number(c))),
            'a'..='z' => Some(Ok(self.collect_identifier(c))),
            '+' => Some(Ok(Token::Operator(Op::Add))),
            '-' => Some(Ok(Token::Operator(Op::Sub))),
            _ => Some(Err(anyhow!(TokenizeError::UnexpectedChar(c)))),
        }
    }
}

#[derive(Debug, Error)]
enum ParseError {
    #[error("Unexpected end of input")]
    UnexpectedEof,
    #[error("Invalid 32-bit integer")]
    InvalidInt,
    #[error("Unexpected token \"{0}\"")]
    UnexpectedToken(String),
}

fn parse(input: &str) -> Result<Expression> {
    let mut tokens = tokenize(input);

    fn parse_expr(tokens: &mut Tokenizer) -> Result<Expression> {
        let tok = tokens.next().ok_or(ParseError::UnexpectedEof)??;
        let expr = match tok {
            Token::Number(num) => {
                let v = num.parse().map_err(|_| ParseError::InvalidInt)?;
                Expression::Number(v)
            }
            Token::Identifier(ident) => Expression::Var(ident),
            Token::Operator(_) => {
                return Err(ParseError::UnexpectedToken(format!("{tok:?}")))
                    .context("first token is operator");
            }
        };
        // バイナリ演算が存在する場合はパースします。
        match tokens.next() {
            None => Ok(expr),
            Some(tok) => match tok? {
                Token::Operator(op) => Ok(Expression::Operation(
                    Box::new(expr),
                    op,
                    Box::new(parse_expr(tokens)?),
                )),
                tok => Err(ParseError::UnexpectedToken(format!("{tok:?}")))
                    .context("second token is not operator"),
            },
        }
    }

    parse_expr(&mut tokens)
}

fn main() {
    println!("{:?}", parse("10+foo+20-30"));
    println!("{:?}", parse("10++foo+20-30").unwrap_err());
    println!("{:?}", parse("10*foo+20-30").unwrap_err());
}
