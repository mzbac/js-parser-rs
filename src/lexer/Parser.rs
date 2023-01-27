use crate::lexer::AstNode::AstNode;
use crate::lexer::Token::Token;

struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Parser {
        Parser { tokens, current: 0 }
    }

    fn parse(&mut self) -> AstNode {
        let mut nodes = Vec::new();
        while !self.is_at_end() {
            nodes.push(self.declaration());
        }
        AstNode::Program { body: nodes }
    }

    fn declaration(&mut self) -> AstNode {
        if self.match_token(Token::Var) {
            self.var_declaration()
        } else if self.match_token(Token::Function) {
            self.function("function")
        } else {
            self.statement()
        }
    }

    fn var_declaration(&mut self) -> AstNode {
        let id = self.consume(Token::Identifier, "Expect variable name.");
        let init = if self.match_token(Token::Equal) {
            Some(self.expression())
        } else {
            None
        };
        self.consume(Token::Semicolon, "Expect ';' after variable declaration.");
        AstNode::VariableDeclaration {
            id: Box::new(AstNode::Identifier(id.lexeme)),
            init: Box::new(init.unwrap_or(AstNode::NumberLiteral(0.0))),
        }
    }

    fn function(&mut self, kind: &str) -> AstNode {
        let name = self.consume(Token::Identifier, &format!("Expect {} name.", kind));
        self.consume(
            Token::LeftParen,
            &format!("Expect '(' after {} name.", kind),
        );
        let mut parameters = Vec::new();
        if !self.check(Token::RightParen) {
            loop {
                parameters.push(self.consume(Token::Identifier, "Expect parameter name."));
                if !self.match_token(Token::Comma) {
                    break;
                }
            }
        }
        self.consume(Token::RightParen, "Expect ')' after parameters.");
        self.consume(
            Token::LeftBrace,
            &format!("Expect '{{' before {} body.", kind),
        );
        let body = self.block();
        AstNode::FunctionDeclaration {
            id: Box::new(AstNode::Identifier(name.lexeme)),
            params: parameters
                .iter()
                .map(|param| AstNode::Identifier(param.lexeme))
                .collect(),
            body: Box::new(body),
        }
    }

    fn statement(&mut self) -> AstNode {
        if self.match_token(Token::LeftBrace) {
            self.block()
        } else if self.match_token(Token::If) {
            self.if_statement()
        } else if self.match_token(Token::Return) {
            self.return_statement()
        } else {
            self.expression_statement()
        }
    }

    fn parse_number(&mut self) -> AstNode {
        let start = self.current;
        while is_digit(self.peek()) {
            self.advance();
        }
        if self.peek() == '.' && is_digit(self.peek_next()) {
            self.advance();
            while is_digit(self.peek()) {
                self.advance();
            }
        }
        let value = self.source[start..self.current].parse().unwrap();
        AstNode::NumberLiteral(value)
    }

    fn parse_string(&mut self) -> AstNode {
        let start = self.current + 1;
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\\' {
                self.advance();
            }
            self.advance();
        }
        if self.is_at_end() {
            // Handle unterminated string error
        }
        let value = self.source[start..self.current].to_string();
        self.advance();
        AstNode::StringLiteral(value)
    }

    fn parse_identifier(&mut self) -> AstNode {
        let start = self.current;
        while is_alphanumeric(self.peek()) {
            self.advance();
        }
        let value = self.source[start..self.current].to_string();
        AstNode::Identifier(value)
    }

    fn parse_call_expression(&mut self) -> AstNode {
        let callee = self.parse_primary();
        let mut arguments = vec![];
        while self.peek() == '(' {
            self.advance();
            arguments.push(self.parse_expression());
            if self.peek() != ')' {
                // Handle missing ')' error
            }
            self.advance();
        }
        AstNode::CallExpression {
            callee: Box::new(callee),
            arguments,
        }
    }

    fn parse_expression(&mut self) -> AstNode {
        self.parse_assignment()
    }

    fn parse_assignment(&mut self) -> AstNode {
        let left = self.parse_ternary();

        if self.match_token(Token::Equal)
            || self.match_token(Token::PlusEqual)
            || self.match_token(Token::MinusEqual)
            || self.match_token(Token::StarEqual)
            || self.match_token(Token::SlashEqual)
        {
            let operator = self.previous().to_string();
            let right = self.parse_assignment();
            return AstNode::AssignmentExpression {
                operator,
                left: Box::new(left),
                right: Box::new(right),
            };
        }

        left
    }

    fn parse_ternary(&mut self) -> AstNode {
        let left = self.parse_or();

        if self.match_token(Token::Question) {
            self.advance();
            let middle = self.parse_expression();
            self.consume(Token::Colon, "Expect ':' after '?' in ternary operator.");
            let right = self.parse_expression();
            return AstNode::TernaryExpression {
                left: Box::new(left),
                middle: Box::new(middle),
                right: Box::new(right),
            };
        }

        left
    }

    fn parse_or(&mut self) -> AstNode {
        let mut left = self.parse_and();

        while self.match_token(Token::PipePipe) {
            let operator = self.previous().to_string();
            let right = self.parse_and();
            left = AstNode::LogicalExpression {
                operator,
                left: Box::new(left),
                right: Box::new(right),
            };
        }

        left
    }

    fn parse_and(&mut self) -> AstNode {
        let mut left = self.parse_equality();

        while self.match_token(Token::AmpersandAmpersand) {
            let operator = self.previous().to_string();
            let right = self.parse_equality();
            left = AstNode::LogicalExpression {
                operator,
                left: Box::new(left),
                right: Box::new(right),
            };
        }

        left
    }

    fn parse_equality(&mut self) -> AstNode {
        let mut left = self.parse_comparison();

        while self.match_token(Token::EqualEqual) || self.match_token(Token::BangEqual) {
            let operator = self.previous().to_string();
            let right = self.parse_comparison();
            left = AstNode::BinaryExpression {
                operator,
                left: Box::new(left),
                right: Box::new(right),
            };
        }

        left
    }

    fn parse_primary(&mut self) -> AstNode {
        if self.match_token(Token::False) {
            AstNode::NumberLiteral(0.0)
        } else if self.match_token(Token::True) {
            AstNode::NumberLiteral(1.0)
        } else if self.match_token(Token::Nil) {
            AstNode::NumberLiteral(0.0)
        } else if self.match_token(Token::Number) {
            self.previous().literal
        } else if self.match_token(Token::String) {
            self.previous().literal
        } else if self.match_token(Token::Identifier) {
            AstNode::Identifier(self.previous().lexeme)
        } else if self.match_token(Token::LeftParen) {
            let expression = self.parse_expression();
            if self.peek() != Token::RightParen {
                // Handle missing ')' error
            }
            self.advance();
            expression
        } else {
            // Handle unexpected token
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::lexer;

    use super::*;

    #[test]
    fn test_parsing_simple_math_expression() {
        let input = "2 + 2";
        let expected_output = AstNode::BinaryExpression {
            operator: "+".to_string(),
            left: Box::new(AstNode::NumberLiteral(2.0)),
            right: Box::new(AstNode::NumberLiteral(2.0)),
        };
        let tokens = lexer::tokenize(input);
        let ast = Parser::parse(tokens);
        assert_eq!(ast, expected_output);
    }

    #[test]
    fn test_parsing_function_call() {
        let input = "add(2, 3)";
        let expected_output = AstNode::CallExpression {
            callee: Box::new(AstNode::Identifier("add".to_string())),
            arguments: vec![AstNode::NumberLiteral(2.0), AstNode::NumberLiteral(3.0)],
        };
        let tokens = lexer::tokenize(input);
        let ast = Parser::parse(tokens);
        assert_eq!(ast, expected_output);
    }

    #[test]
    fn test_parsing_variable_assignment() {
        let input = "let x = 5;";
        let expected_output = AstNode::VariableDeclaration {
            id: Box::new(AstNode::Identifier("x".to_string())),
            init: Box::new(AstNode::NumberLiteral(5.0)),
        };
        let tokens = lexer::tokenize(input);
        let ast = Parser::parse(tokens);
        assert_eq!(ast, expected_output);
    }
}
