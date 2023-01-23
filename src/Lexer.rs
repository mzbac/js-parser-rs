use std::iter::Peekable;
use std::str::Chars;

#[derive(Debug, PartialEq)]
enum Token {
    Number(f64),
    String(String),
    Identifier(String),
    Plus,
    Minus,
    Star,
    Slash,
    LParen,
    RParen,
    LBrace,
    RBrace,
    LBracket,
    RBracket,
    Semicolon,
    Comma,
    Dot,
    Eq,
    Neq,
    Lt,
    Gt,
    Lte,
    Gte,
    Assign,
    And,
    Or,
    Not,
    Null,
    True,
    False,
    Var,
    If,
    Else,
    While,
    Function,
    Return,
    This,
}

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
                Some(Token::LParen)
            }
            ')' => {
                self.pos += 1;
                Some(Token::RParen)
            }
            '{' => {
                self.pos += 1;
                Some(Token::LBrace)
            }
            '}' => {
                self.pos += 1;
                Some(Token::RBrace)
            }
            '[' => {
                self.pos += 1;
                Some(Token::LBracket)
            }
            ']' => {
                self.pos += 1;
                Some(Token::RBracket)
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
                    Some(Token::Eq)
                } else {
                    self.pos += 1;
                    Some(Token::Assign)
                }
            }
            '!' => {
                if self.peek_n(1) == Some('=') {
                    self.pos += 2;
                    Some(Token::Neq)
                } else {
                    self.pos += 1;

                    Some(Token::Not)
                }
            }
            '<' => {
                if self.peek_n(1) == Some('=') {
                    self.pos += 2;
                    Some(Token::Lte)
                } else {
                    self.pos += 1;
                    Some(Token::Lt)
                }
            }
            '>' => {
                if self.peek_n(1) == Some('=') {
                    self.pos += 2;
                    Some(Token::Gte)
                } else {
                    self.pos += 1;
                    Some(Token::Gt)
                }
            }
            '&' => {
                if self.peek_n(1) == Some('&') {
                    self.pos += 2;
                    Some(Token::And)
                } else {
                    self.pos += 1;
                    Some(Token::And)
                }
            }
            '|' => {
                if self.peek_n(1) == Some('|') {
                    self.pos += 2;
                    Some(Token::Or)
                } else {
                    self.pos += 1;
                    Some(Token::Or)
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
        assert_eq!(lexer.next_token(), Some(Token::Assign));
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
        assert_eq!(lexer.next_token(), Some(Token::Assign));
        assert_eq!(lexer.next_token(), Some(Token::String("John".to_string())));
        assert_eq!(lexer.next_token(), Some(Token::Semicolon));
        assert_eq!(lexer.next_token(), None);
    }

    #[test]
    fn test_comparison_tokens() {
        let source = "x > 10 && y < 20";
        let mut lexer = Lexer::new(source);

        assert_eq!(lexer.next_token(), Some(Token::Identifier("x".to_string())));
        assert_eq!(lexer.next_token(), Some(Token::Gt));
        assert_eq!(lexer.next_token(), Some(Token::Number(10.0)));
        assert_eq!(lexer.next_token(), Some(Token::And));
        assert_eq!(lexer.next_token(), Some(Token::Identifier("y".to_string())));
        assert_eq!(lexer.next_token(), Some(Token::Lt));
        assert_eq!(lexer.next_token(), Some(Token::Number(20.0)));
        assert_eq!(lexer.next_token(), None);
    }
}
