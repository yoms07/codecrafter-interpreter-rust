use std::{fmt::format, process};

use crate::token::{Literal, Token, TokenType};

pub struct Scanner<'a> {
    file_content: &'a str,
    start: usize,
    current: usize,
    line: u64,
    end: usize,
    tokens: Vec<Token<'a>>,
    pub has_error: bool,
}

impl<'a> Scanner<'a> {
    pub fn new(file_content: &'a String) -> Self {
        Self {
            file_content: file_content,
            start: 0,
            current: 0,
            line: 1,
            end: file_content.len(),
            tokens: Vec::new(),
            has_error: false,
        }
    }
    pub fn scan_tokens(&mut self) -> &Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

        self.tokens
            .push(Token::new(TokenType::EOF, "", None, self.line));
        &self.tokens
    }

    fn scan_token(&mut self) {
        if let Some(c) = self.advance() {
            use TokenType::*;
            match c {
                '(' => self.add_token(LeftParen, None),
                ')' => self.add_token(RightParen, None),
                '{' => self.add_token(LeftBrace, None),
                '}' => self.add_token(RightBrace, None),
                ',' => self.add_token(Comma, None),
                '.' => self.add_token(Dot, None),
                '-' => self.add_token(Minus, None),
                '+' => self.add_token(Plus, None),
                ';' => self.add_token(Semicolon, None),
                '*' => self.add_token(Star, None),
                '!' => {
                    let token_type = if self.match_next('=') {
                        BangEqual
                    } else {
                        Bang
                    };
                    self.add_token(token_type, None);
                }

                '=' => {
                    let token_type = if self.match_next('=') {
                        EqualEqual
                    } else {
                        Equal
                    };
                    self.add_token(token_type, None);
                }

                '<' => {
                    let token_type = if self.match_next('=') {
                        LessEqual
                    } else {
                        Less
                    };
                    self.add_token(token_type, None);
                }

                '>' => {
                    let token_type = if self.match_next('=') {
                        GreaterEqual
                    } else {
                        Greater
                    };
                    self.add_token(token_type, None);
                }
                '/' => {
                    if self.match_next('/') {
                        while self.peek().is_some_and(|x| x != '\n') {
                            self.advance();
                        }
                    } else {
                        self.add_token(Slash, None);
                    }
                }
                _ => {
                    // later: handle whitespace, identifiers, etc.
                    self.error(self.line, &format!("Unexpected character: {}", c));
                }
            }
        }
    }

    fn peek(&self) -> Option<char> {
        self.file_content[self.current..].chars().next()
    }

    fn match_next(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }

        if self.peek() == Some(expected) {
            self.current += expected.len_utf8();
            return true;
        } else {
            return false;
        }
    }

    fn advance(&mut self) -> Option<char> {
        if let Some(c) = self.peek() {
            self.current += c.len_utf8();
            return Some(c);
        }
        {
            return None;
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.end
    }

    fn add_token(&mut self, token_type: TokenType, literal: Option<Literal>) {
        let text = &self.file_content[self.start..self.current];
        self.tokens
            .push(Token::new(token_type, text, literal, self.line));
    }

    pub fn error(&mut self, line: u64, msg: &str) {
        Scanner::report(line, "", msg);
        self.has_error = true;
    }

    fn report(line: u64, whre: &str, msg: &str) {
        eprintln!("[line {}] Error{}: {}", line, whre, msg)
    }
}
