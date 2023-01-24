use std::iter::Peekable;
use std::str::Chars;

use super::Token::{Token};


struct Lexer<'a> {
    source: Peekable<Chars<'a>>,
    pos: usize,
}

impl<'a> Lexer<'a> {
    fn new(source: &'a str) -> Self {
        Self {
            source: source.chars().peekable(),
            pos: 0,
        }
    }

    fn next_token(&mut self) -> Option<Token> {
        self.skip_whitespace();

        let ch = self.peek();
        if ch.is_none() {
            return None;
        }

        let ch = ch.unwrap();

        if ch.is_ascii_digit() {
            return Some(self.scan_number());
        } else if ch == '"' || ch == '\'' {
            return Some(self.scan_string());
        } else if ch.is_ascii_alphabetic() {
            return Some(self.scan_identifier());
        }

        match ch {
            '+' => {
                self.pos += 1;
                Some(Token::Plus)
            }
            '-' => {
                self.pos += 1;
                Some(Token::Minus)
            }
            '*' => {
                self.pos += 1;
                Some(Token::Star)
            }
            '/' => {
                self.pos += 1;
                Some(Token::Slash)
            }
            '(' => {
                self.pos += 1;
                Some(Token::LeftParen)
            }
            ')' => {
                self.pos += 1;
                Some(Token::RightParen)
            }
            '{' => {
                self.pos += 1;
                Some(Token::LeftBrace)
            }
            '}' => {
                self.pos += 1;
                Some(Token::RightBrace)
            }
            '[' => {
                self.pos += 1;
                Some(Token::LeftBracket)
            }
            ']' => {
                self.pos += 1;
                Some(Token::RightBracket)
            }
            ';' => {
                self.pos += 1;
                Some(Token::Semicolon)
            }
            ',' => {
                self.pos += 1;
                Some(Token::Comma)
            }
            '.' => {
                self.pos += 1;
                Some(Token::Dot)
            }
            '=' => {
                if self.peek_n(1) == Some('=') {
                    self.pos += 2;
                    Some(Token::EqualEqual)
                } else {
                    self.pos += 1;
                    Some(Token::Equal)
                }
            }
            '!' => {
                if self.peek_n(1) == Some('=') {
                    self.pos += 2;
                    Some(Token::BangEqual)
                } else {
                    self.pos += 1;
                    Some(Token::Bang)
                }
            }
            '<' => {
                if self.peek_n(1) == Some('=') {
                    self.pos += 2;
                    Some(Token::LessEqual)
                } else {
                    self.pos += 1;
                    Some(Token::Less)
                }
            }
            '>' => {
                if self.peek_n(1) == Some('=') {
                    self.pos += 2;
                    Some(Token::GreaterEqual)
                } else {
                    self.pos += 1;
                    Some(Token::Greater)
                }
            }
            '&' => {
                if self.peek_n(1) == Some('&') {
                    self.pos += 2;
                    Some(Token::AmpersandAmpersand)
                } else {
                    self.pos += 1;
                    Some(Token::Ampersand)
                }
            }
            '|' => {
                if self.peek_n(1) == Some('|') {
                    self.pos += 2;
                    Some(Token::PipePipe)
                } else {
                    self.pos += 1;
                    Some(Token::Pipe)
                }
            }
            _ => None,
        }
    }

    fn peek(&mut self) -> Option<char> {
        let mut chars = self.source.clone();

        for _ in 0..self.pos {
            chars.next();
        }
        chars.peek().cloned()
    }

    fn peek_n(&self, n: usize) -> Option<char> {
        let mut chars = self.source.clone();
        for _ in 0..(self.pos + n) {
            chars.next();
        }
        chars.peek().cloned()
    }

    fn scan_number(&mut self) -> Token {
        let mut number = String::new();

        while let Some(ch) = self.peek() {
            if ch.is_ascii_digit() || ch == '.' {
                number.push(ch);
                self.pos += 1;
            } else {
                break;
            }
        }

        Token::Number(number.parse().unwrap())
    }

    fn scan_string(&mut self) -> Token {
        let mut string = String::new();
        let quote = self.next().unwrap();

        while let Some(ch) = self.next() {
            if ch == quote {
                break;
            }

            string.push(ch);
        }

        Token::String(string)
    }

    fn scan_identifier(&mut self) -> Token {
        let mut identifier = String::new();

        while let Some(ch) = self.peek() {
            if ch.is_ascii_alphabetic() || ch.is_ascii_digit() || ch == '_' {
                identifier.push(ch);
                self.pos += 1;
            } else {
                break;
            }
        }

        match identifier.as_str() {
            "var" => Token::Var,
            "if" => Token::If,
            "else" => Token::Else,
            "while" => Token::While,
            "function" => Token::Function,
            "return" => Token::Return,
            "this" => Token::This,
            "null" => Token::Null,
            "True" => Token::True,
            "False" => Token::False,
            _ => Token::Identifier(identifier),
        }
    }

    fn next(&mut self) -> Option<char> {
        let ch = self.peek();
        if ch.is_some() {
            self.pos += 1;
        }
        ch
    }

    fn skip_whitespace(&mut self) {
        while let Some(ch) = self.peek() {
            if ch.is_whitespace() {
                self.pos += 1;
            } else {
                break;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next_token() {
        let source = "var x = 10;";
        let mut lexer = Lexer::new(source);

        assert_eq!(lexer.next_token(), Some(Token::Var));
        assert_eq!(lexer.next_token(), Some(Token::Identifier("x".to_string())));
        assert_eq!(lexer.next_token(), Some(Token::Equal));
        assert_eq!(lexer.next_token(), Some(Token::Number(10.0)));
        assert_eq!(lexer.next_token(), Some(Token::Semicolon));
        assert_eq!(lexer.next_token(), None);
    }

    #[test]
    fn test_string_token() {
        let source = "var name = \"John\";";
        let mut lexer = Lexer::new(source);

        assert_eq!(lexer.next_token(), Some(Token::Var));
        assert_eq!(
            lexer.next_token(),
            Some(Token::Identifier("name".to_string()))
        );
        assert_eq!(lexer.next_token(), Some(Token::Equal));
        assert_eq!(lexer.next_token(), Some(Token::String("John".to_string())));
        assert_eq!(lexer.next_token(), Some(Token::Semicolon));
        assert_eq!(lexer.next_token(), None);
    }

    #[test]
    fn test_comparison_tokens() {
        let source = "x > 10 && y < 20";
        let mut lexer = Lexer::new(source);

        assert_eq!(lexer.next_token(), Some(Token::Identifier("x".to_string())));
        assert_eq!(lexer.next_token(), Some(Token::Greater));
        assert_eq!(lexer.next_token(), Some(Token::Number(10.0)));
        assert_eq!(lexer.next_token(), Some(Token::AmpersandAmpersand));
        assert_eq!(lexer.next_token(), Some(Token::Identifier("y".to_string())));
        assert_eq!(lexer.next_token(), Some(Token::Less));
        assert_eq!(lexer.next_token(), Some(Token::Number(20.0)));
        assert_eq!(lexer.next_token(), None);
    }
}
