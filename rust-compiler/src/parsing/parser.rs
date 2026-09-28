use crate::ast::{
    Expression,
    Statement,
    Type,
};

use crate::lexing::Token;

pub struct Parser {
    tokens: Vec<Token>,
    position: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            position: 0,
        }
    }

    pub fn parse(&mut self) -> Vec<Statement> {
        let mut statements = Vec::new();

        while self.position < self.tokens.len() {
            statements.push(self.parse_statement());
        }

        statements
    }

    fn parse_statement(&mut self) -> Statement {
        match self.current_token() {
            Some(Token::Int) => {
                self.parse_variable_declaration()
            }

            Some(Token::Identifier(_)) => {
                self.parse_assignment()
            }

            Some(Token::Print) => {
                self.parse_print()
            }

            _ => panic!("Unexpected token"),
        }
    }

    fn parse_variable_declaration(&mut self) -> Statement {
        self.expect(Token::Int);

        self.expect(Token::DoubleColon);

        let name = match self.advance() {
            Some(Token::Identifier(name)) => name.clone(),

            _ => panic!("Expected identifier"),
        };

        self.expect(Token::Assign);

        let value = self.parse_expression();

        self.expect(Token::Semicolon);

        Statement::VariableDeclaration {
            name,
            type_: Type::Int,
            value,
        }
    }

    fn parse_assignment(&mut self) -> Statement {
        let name = match self.advance() {
            Some(Token::Identifier(name)) => name.clone(),

            _ => panic!("Expected identifier"),
        };

        self.expect(Token::Assign);

        let value = self.parse_expression();

        self.expect(Token::Semicolon);

        Statement::Assignment {
            name,
            value,
        }
    }

    fn parse_print(&mut self) -> Statement {
        self.expect(Token::Print);

        self.expect(Token::LeftParen);

        let expression = self.parse_expression();

        self.expect(Token::RightParen);

        self.expect(Token::Semicolon);

        Statement::Print {
            expression,
        }
    }

    fn parse_expression(&mut self) -> Expression {
        match self.advance() {
            Some(Token::IntegerLiteral(value)) => {
                Expression::IntegerLiteral(*value)
            }

            Some(Token::StringLiteral(value)) => {
                Expression::StringLiteral(value.clone())
            }

            Some(Token::Identifier(name)) => {
                Expression::Identifier(name.clone())
            }

            _ => panic!("Expected expression"),
        }
    }

    fn current_token(&self) -> Option<&Token> {
        self.tokens.get(self.position)
    }

    fn advance(&mut self) -> Option<&Token> {
        let token = self.tokens.get(self.position);

        self.position += 1;

        token
    }

    fn expect(&mut self, expected: Token) {
        let current = self.advance();

        if current != Some(&expected) {
            panic!("Unexpected token");
        }
    }
}