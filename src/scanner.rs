use std::{iter::Peekable, str::Chars};

use crate::token::{Token, TokenType};

pub struct Scanner<'a> {
    chars: Peekable<Chars<'a>>,
    line: usize,
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a str) -> Self {
        Scanner {
            chars: source.chars().peekable(),
            line: 1,
        }
    }

    #[inline(always)]
    fn match_next(&mut self, expected: char) -> bool {
        if self.chars.peek() == Some(&expected) {
            self.chars.next();
            return true;
        }
        false
    }

    #[inline(always)]
    fn skip_until(&mut self, target: char) {
        while let Some(&c) = self.chars.peek() {
            self.chars.next();
            if c == target {
                break;
            }
        }
    }

    #[inline(always)]
    fn read_util<F>(&mut self, condition: F) -> String
    where
        F: Fn(&char) -> bool,
    {
        let mut value = String::new();
        while let Some(&ch) = self.chars.peek() {
            if !condition(&ch) {
                return value;
            }
            value.push(ch);
            self.chars.next();
        }
        value
    }

    #[inline(always)]
    fn read_number(&mut self, first_char: char) -> TokenType {
        let mut number = String::from(first_char);
        number.push_str(&self.read_util(|ch| ch.is_ascii_digit()));
        if self.chars.peek() == Some(&'.') {
            self.chars.next();
            number.push('.');
            number.push_str(&self.read_util(|ch| ch.is_ascii_digit()));
        }
        TokenType::Number(number.parse::<f64>().unwrap())
    }
}

impl<'a> Iterator for Scanner<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        let ch = self.chars.next()?;
        let token_type = match ch {
            '(' => TokenType::LeftParen,
            ')' => TokenType::RightParen,
            '{' => TokenType::LeftBrace,
            '}' => TokenType::RightBrace,
            ',' => TokenType::Comma,
            '.' => TokenType::Dot,
            '-' => TokenType::Minus,
            '+' => TokenType::Plus,
            ';' => TokenType::Semicolon,
            '*' => TokenType::Star,
            '!' if self.match_next('=') => TokenType::BangEqual,
            '!' => TokenType::Bang,
            '=' if self.match_next('=') => TokenType::EqualEqual,
            '=' => TokenType::Equal,
            '<' if self.match_next('=') => TokenType::LessEqual,
            '<' => TokenType::Less,
            '>' if self.match_next('=') => TokenType::GreaterEqual,
            '>' => TokenType::Greater,
            '/' if self.match_next('/') => {
                self.skip_until('\n');
                self.line += 1;
                return self.next();
            }
            '/' => TokenType::Slash,
            '\n' => {
                self.line += 1;
                return self.next();
            }
            ' ' | '\r' | '\t' => return self.next(),
            '"' => {
                let value = self.read_util(|c| *c != '"');
                self.chars.next();
                TokenType::String(value)
            }
            ch if ch.is_ascii_digit() => self.read_number(ch),
            _ => panic!("Unexpected character: {ch}"),
        };
        Some(Token::new(token_type, self.line))
    }
}
