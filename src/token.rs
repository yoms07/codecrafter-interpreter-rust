use std::fmt::Display;

use std::fmt::Display as FmtDisplay;
use strum_macros::Display;

#[derive(Display)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
pub enum TokenType {
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // One or two character tokens.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals.
    Identifier,
    String,
    Number,

    // Keywords.
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    EOF,
}

#[derive(Clone, Debug)]
pub enum Literal {
    Number(f64),
    String(String),
    Boolean(bool),

    Nil,
}

impl std::fmt::Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Literal::Number(n) => {
                if n.fract() == 0.0 {
                    write!(f, "{:.1}", n)
                } else {
                    write!(f, "{}", n)
                }
            }
            Literal::String(s) => write!(f, "{}", s),
            Literal::Boolean(b) => write!(f, "{}", b),
            Literal::Nil => write!(f, "null"),
        }
    }
}

pub struct Token<'a> {
    token_type: TokenType,
    lexeme: &'a str,
    literal: Option<Literal>,
    line: u64,
}

impl<'a> Token<'a> {
    pub fn to_string(&self) -> String {
        let literal_str = self
            .literal
            .as_ref()
            .map(|lit| lit.to_string())
            .unwrap_or_else(|| "null".to_string());
        format!("{} {} {}", self.token_type, self.lexeme, literal_str)
    }

    pub fn new(
        token_type: TokenType,
        lexeme: &'a str,
        literal: Option<Literal>,
        line: u64,
    ) -> Self {
        Self {
            token_type: token_type,
            lexeme: lexeme,
            literal: literal,
            line: line,
        }
    }
}
