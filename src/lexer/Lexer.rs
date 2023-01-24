use std::iter::Peekable;
use std::str::Chars;

use super::Token::Token;

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
                if self.peek_n(1) == Some('/') {
                    self.pos += 2;
                    self.skip_comment();
                    None
                } else if self.peek_n(1) == Some('*') {
                    self.pos += 2;
                    self.skip_comment_block();
                    None
                } else {
                    self.pos += 1;
                    Some(Token::Slash)
                }
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

    fn skip_comment(&mut self) {
        while let Some(ch) = self.peek() {
            if ch != '\n' {
                self.pos += 1;
            } else {
                self.pos += 1;
                break;
            }
        }
    }
    fn skip_comment_block(&mut self) {
        while let Some(ch) = self.peek() {
            if ch == '*' && self.peek_n(1) == Some('/') {
                self.pos += 2;
                break;
            } else {
                self.pos += 1;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_number() {
        let mut lexer = Lexer::new("123");
        let token = lexer.next_token();
        assert_eq!(token, Some(Token::Number(123.0)));
    }

    #[test]
    fn test_string() {
        let mut lexer = Lexer::new("\"hello world\"");
        let token = lexer.next_token();
        assert_eq!(token, Some(Token::String("hello world".to_string())));
    }

    #[test]
    fn test_identifier() {
        let mut lexer = Lexer::new("var x = 10");
        let token1 = lexer.next_token();
        let token2 = lexer.next_token();
        let token3 = lexer.next_token();
        let token4 = lexer.next_token();
        assert_eq!(token1, Some(Token::Var));
        assert_eq!(token2, Some(Token::Identifier("x".to_string())));
        assert_eq!(token3, Some(Token::Equal));
        assert_eq!(token4, Some(Token::Number(10.0)));
    }

    #[test]
    fn test_operators() {
        let mut lexer = Lexer::new("a + b");
        let token1 = lexer.next_token();
        let token2 = lexer.next_token();
        let token3 = lexer.next_token();
        assert_eq!(token1, Some(Token::Identifier("a".to_string())));
        assert_eq!(token2, Some(Token::Plus));
        assert_eq!(token3, Some(Token::Identifier("b".to_string())));
    }

    #[test]
    fn test_punctuation() {
        let mut lexer = Lexer::new("if (x < 10) {");
        let token1 = lexer.next_token();
        let token2 = lexer.next_token();
        let token3 = lexer.next_token();
        let token4 = lexer.next_token();
        let token5 = lexer.next_token();
        let token6 = lexer.next_token();
        let token7 = lexer.next_token();
        assert_eq!(token1, Some(Token::If));
        assert_eq!(token2, Some(Token::LeftParen));
        assert_eq!(token3, Some(Token::Identifier("x".to_string())));
        assert_eq!(token4, Some(Token::Less));
        assert_eq!(token5, Some(Token::Number(10.0)));
        assert_eq!(token6, Some(Token::RightParen));
        assert_eq!(token7, Some(Token::LeftBrace));
    }
    #[test]
    fn test_comments() {
        let mut lexer = Lexer::new("// this is a comment");
        let token = lexer.next_token();
        assert_eq!(token, None);
    }

    #[test]
    fn test_multiline_code() {
        let source = "var x = 10;
                      var y = 20;
                      var z = x + y;
                      return z;";
        let mut lexer = Lexer::new(source);

        let token1 = lexer.next_token();
        let token2 = lexer.next_token();
        let token3 = lexer.next_token();
        let token4 = lexer.next_token();
        let token5 = lexer.next_token();
        let token6 = lexer.next_token();
        let token7 = lexer.next_token();
        let token8 = lexer.next_token();
        let token9 = lexer.next_token();
        let token10 = lexer.next_token();
        let token11 = lexer.next_token();
        let token12 = lexer.next_token();
        let token13 = lexer.next_token();

        assert_eq!(token1, Some(Token::Var));
        assert_eq!(token2, Some(Token::Identifier("x".to_string())));
        assert_eq!(token3, Some(Token::Equal));
        assert_eq!(token4, Some(Token::Number(10.0)));
        assert_eq!(token5, Some(Token::Semicolon));
        assert_eq!(token6, Some(Token::Var));
        assert_eq!(token7, Some(Token::Identifier("y".to_string())));
        assert_eq!(token8, Some(Token::Equal));
        assert_eq!(token9, Some(Token::Number(20.0)));
        assert_eq!(token10, Some(Token::Semicolon));
        assert_eq!(token11, Some(Token::Var));
        assert_eq!(token12, Some(Token::Identifier("z".to_string())));
        assert_eq!(token13, Some(Token::Equal));

        let token14 = lexer.next_token();
        let token15 = lexer.next_token();
        let token16 = lexer.next_token();
        let token17 = lexer.next_token();
        let token18 = lexer.next_token();
        assert_eq!(token14, Some(Token::Identifier("x".to_string())));
        assert_eq!(token15, Some(Token::Plus));
        assert_eq!(token16, Some(Token::Identifier("y".to_string())));
        assert_eq!(token17, Some(Token::Semicolon));
        assert_eq!(token18, Some(Token::Return));
    }

    #[test]
    fn test_multiline_code_with_comments() {
        let source = "var x = 10; // x is assigned the value of 10
                      /* This is a block comment
                      var y = 20;
                      */ var z = x + 15; // z is assigned the value of x + 15
                      return z;";
        let mut lexer = Lexer::new(source);

        let token1 = lexer.next_token();
        let token2 = lexer.next_token();
        let token3 = lexer.next_token();
        let token4 = lexer.next_token();
        assert_eq!(token1, Some(Token::Var));
        assert_eq!(token2, Some(Token::Identifier("x".to_string())));
        assert_eq!(token3, Some(Token::Equal));
        assert_eq!(token4, Some(Token::Number(10.0)));

        let token5 = lexer.next_token();
        assert_eq!(token5, Some(Token::Semicolon));

        let comment = lexer.next_token();
        assert_eq!(comment, None);

        let comment_block = lexer.next_token();
        assert_eq!(comment_block, None);

        let token6 = lexer.next_token();
        let token7 = lexer.next_token();
        let token8 = lexer.next_token();
        let token9 = lexer.next_token();
        let token10 = lexer.next_token();
        let token11 = lexer.next_token();
        let token12 = lexer.next_token();

        assert_eq!(token6, Some(Token::Var));
        assert_eq!(token7, Some(Token::Identifier("z".to_string())));
        assert_eq!(token8, Some(Token::Equal));
        assert_eq!(token9, Some(Token::Identifier("x".to_string())));
        assert_eq!(token10, Some(Token::Plus));
        assert_eq!(token11, Some(Token::Number(15.0)));
        assert_eq!(token12, Some(Token::Semicolon));

        let comment2 = lexer.next_token();
        assert_eq!(comment2, None);

        let token13 = lexer.next_token();
        let token14 = lexer.next_token();
        let token15 = lexer.next_token();

        assert_eq!(token13, Some(Token::Return));
        assert_eq!(token14, Some(Token::Identifier("z".to_string())));
        assert_eq!(token15, Some(Token::Semicolon));

    }
}
