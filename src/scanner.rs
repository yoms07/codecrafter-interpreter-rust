use std::{collections::HashMap, fmt::format, iter::Map, process};

use crate::token::{Literal, Token, TokenType};

pub struct Scanner<'a> {
    file_content: &'a str,
    start: usize,
    current: usize,
    line: u64,
    end: usize,
    tokens: Vec<Token<'a>>,
    pub has_error: bool,
    keywords_map: HashMap<&'static str, TokenType>,
}

impl<'a> Scanner<'a> {
    fn build_map(&mut self) {
        self.keywords_map.insert("and", TokenType::And);
        self.keywords_map.insert("class", TokenType::Class);
        self.keywords_map.insert("else", TokenType::Else);
        self.keywords_map.insert("false", TokenType::False);
        self.keywords_map.insert("for", TokenType::For);
        self.keywords_map.insert("fun", TokenType::Fun);
        self.keywords_map.insert("if", TokenType::If);
        self.keywords_map.insert("nil", TokenType::Nil);
        self.keywords_map.insert("or", TokenType::Or);
        self.keywords_map.insert("print", TokenType::Print);
        self.keywords_map.insert("return", TokenType::Return);
        self.keywords_map.insert("super", TokenType::Super);
        self.keywords_map.insert("this", TokenType::This);
        self.keywords_map.insert("true", TokenType::True);
        self.keywords_map.insert("var", TokenType::Var);
        self.keywords_map.insert("while", TokenType::While);
    }

    pub fn new(file_content: &'a String) -> Self {
        let mut result = Self {
            file_content: file_content,
            start: 0,
            current: 0,
            line: 1,
            end: file_content.len(),
            tokens: Vec::new(),
            has_error: false,
            keywords_map: HashMap::new(),
        };

        result.build_map();
        result
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
                '"' => {
                    self.string();
                }
                ' ' | '\r' | '\t' => {}
                '\n' => self.line += 1,
                _ => {
                    if self.is_digit(c) {
                        self.number();
                    } else if self.is_alpha(c) {
                        self.identifier();
                    } else {
                        self.error(self.line, &format!("Unexpected character: {}", c));
                    }
                }
            }
        }
    }

    fn peek(&self) -> Option<char> {
        self.file_content[self.current..].chars().next()
    }

    fn peek_next(&self) -> Option<char> {
        let mut chars = self.file_content[self.current..].chars();
        chars.next()?;
        chars.next()
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

    fn string(&mut self) {
        while let Some(c) = self.peek() {
            if c == '"' {
                break;
            }
            if c == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            self.error(self.line, "Unterminated string.");
            return;
        }

        // the closing "
        self.advance();

        let value = &self.file_content[self.start + 1..self.current - 1];
        self.add_token(TokenType::String, Some(Literal::String(value.to_string())));
    }

    fn number(&mut self) {
        while let Some(c) = self.peek() {
            if self.is_digit(c) {
                self.advance();
            } else {
                break;
            }
        }

        match (self.peek(), self.peek_next()) {
            (Some('.'), Some(next)) if self.is_digit(next) => {
                self.advance();
                while let Some(c) = self.peek() {
                    if self.is_digit(c) {
                        self.advance();
                    } else {
                        break;
                    }
                }
            }
            _ => {}
        }

        let value = &self.file_content[self.start..self.current];

        let num_value: f64 = value.parse().unwrap();
        self.add_token(TokenType::Number, Some(Literal::Number(num_value)));
    }

    fn identifier(&mut self) {
        while let Some(c) = self.peek() {
            if self.is_alphanumeric(c) {
                self.advance();
            } else {
                break;
            }
        }

        let text = &self.file_content[self.start..self.current];
        let token_type = self
            .keywords_map
            .get(text)
            .copied()
            .unwrap_or(TokenType::Identifier);

        self.add_token(token_type, None);
    }

    fn is_digit(&self, c: char) -> bool {
        c >= '0' && c <= '9'
    }

    fn is_alpha(&self, c: char) -> bool {
        return (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || (c == '_');
    }

    fn is_alphanumeric(&self, c: char) -> bool {
        self.is_alpha(c) || self.is_digit(c)
    }

    pub fn error(&mut self, line: u64, msg: &str) {
        Scanner::report(line, "", msg);
        self.has_error = true;
    }

    fn report(line: u64, whre: &str, msg: &str) {
        eprintln!("[line {}] Error{}: {}", line, whre, msg)
    }
}
