#[derive(Debug, PartialEq)]
pub enum Token {
    // Keywords
    Print,
    If,
    Else,
    Int,

    // Literals
    IntegerLiteral(i32),
    StringLiteral(String),

    // Identifiers
    Identifier(String),

    // Operators
    Plus,
    Assign,
    GreaterThan,
    LessThan,

    // Type Declaration
    DoubleColon,

    // Punctuation
    Semicolon,
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
}

impl Token {
pub fn get_token(token_type: &str, value: Option<&str>) -> Token {
        match token_type {
            // Keywords
            "Print" => Token::Print,
            "If" => Token::If,
            "Else" => Token::Else,
            "Int" => Token::Int,

            // Literals
            "IntegerLiteral" => {
                Token::IntegerLiteral(
                    value
                        .expect("IntegerLiteral requires a value")
                        .parse::<i32>()
                        .expect("Invalid integer literal")
                )
            }

            "StringLiteral" => {
                Token::StringLiteral(
                    value
                        .expect("StringLiteral requires a value")
                        .to_string()
                )
            }

            // Identifiers
            "Identifier" => {
                Token::Identifier(
                    value
                        .expect("Identifier requires a value")
                        .to_string()
                )
            }

            // Operators
            "Plus" => Token::Plus,
            "Assign" => Token::Assign,
            "GreaterThan" => Token::GreaterThan,
            "LessThan" => Token::LessThan,

            // Type declaration
            "DoubleColon" => Token::DoubleColon,

            // Punctuation
            "Semicolon" => Token::Semicolon,
            "LeftParen" => Token::LeftParen,
            "RightParen" => Token::RightParen,
            "LeftBrace" => Token::LeftBrace,
            "RightBrace" => Token::RightBrace,

            _ => panic!("Unknown token type: {}", token_type),
        }
    }

        pub fn get_token_regex(token_type: &str) -> String {
        match token_type {
            "Print" => r"print",
            "If" => r"if",
            "Else" => r"else",
            "Int" => r"int",
            "IntegerLiteral" => r"\d+",
            "StringLiteral" => r#"\".*\""#,
            "Identifier" => r"[a-zA-Z_][a-zA-Z0-9_]*",
            "Plus" => r"\+",
            "Assign" => r"=",
            "DoubleColon" => r"::",
            "Semicolon" => r";",
            "LeftParen" => r"\(",
            "RightParen" => r"\)",
            "LeftBrace" => r"\{",
            "RightBrace" => r"\}",
            "GreaterThan" => r">",
            "LessThan" => r"<",
            _ => panic!("Invalid token type: {}", token_type),
        }.to_string()
    }
}