use crate::lexing::Token;
pub struct Lexer {
    // Código-fonte que será analisado
    source: Vec<char>,

    // Posição atual dentro do código-fonte
    position: usize,
}

impl Lexer {
    // Cria um novo Lexer a partir do código-fonte
    pub fn new(source: &str) -> Self {
        Self {
            source: source.chars().collect(),
            position: 0,
        }
    }

    // Analisa todo o código-fonte e retorna os tokens encontrados
    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();

        while self.position < self.source.len() {
            if let Some(token) = self.next_token() {
                tokens.push(token);
            }
        }

        tokens
    }

    // Analisa o próximo token
    fn next_token(&mut self) -> Option<Token> {
        // Ignora espaços, tabs e quebras de linha
        self.skip_whitespace();

        // Verifica se chegamos ao final do código
        if self.position >= self.source.len() {
            return None;
        }

        // Pega o caractere atual
        let current = self.source[self.position];

        match current {
            // =========================================
            // LITERALS
            // =========================================

            // Números
            '0'..='9' => Some(self.read_number()),

            // Strings
            '"' => Some(self.read_string()),


            // =========================================
            // IDENTIFIERS / KEYWORDS
            // =========================================

            // Identificadores começam com letra ou _
            'a'..='z' | 'A'..='Z' | '_' => {
                Some(self.read_identifier())
            }


            // =========================================
            // OPERATORS
            // =========================================

            // +
            '+' => {
                self.position += 1;

                Some(Token::Plus)
            }

            // =
            '=' => {
                self.position += 1;

                Some(Token::Assign)
            }

            // >
            '>' => {
                self.position += 1;

                Some(Token::GreaterThan)
            }

            // <
            '<' => {
                self.position += 1;

                Some(Token::LessThan)
            }


            // =========================================
            // TYPE DECLARATION
            // =========================================

            // ::
            ':'
                => {
                self.position += 1;

                // Verifica se o próximo caractere também é ':'
                if self.position < self.source.len()
                    && self.source[self.position] == ':'
                {
                    self.position += 1;

                    Some(Token::DoubleColon)
                } else {
                    panic!("Expected ':' after ':'");
                }
            }


            // =========================================
            // PUNCTUATION
            // =========================================

            // ;
            ';' => {
                self.position += 1;

                Some(Token::Semicolon)
            }

            // (
            '(' => {
                self.position += 1;

                Some(Token::LeftParen)
            }

            // )
            ')' => {
                self.position += 1;

                Some(Token::RightParen)
            }

            // {
            '{' => {
                self.position += 1;

                Some(Token::LeftBrace)
            }

            // }
            '}' => {
                self.position += 1;

                Some(Token::RightBrace)
            }


            // =========================================
            // UNKNOWN CHARACTER
            // =========================================

            _ => {
                panic!(
                    "Unknown character: {}",
                    current
                );
            }
        }
    }


    // =============================================
    // WHITESPACE
    // =============================================

    // Ignora:
    //
    // " "
    // "\t"
    // "\n"
    // "\r"
    //
    // etc.
    fn skip_whitespace(&mut self) {
        while self.position < self.source.len()
            && self.source[self.position].is_whitespace()
        {
            self.position += 1;
        }
    }


    // =============================================
    // NUMBER
    // =============================================

    // Lê um número inteiro.
    //
    // Exemplo:
    //
    // 12345
    //
    // Resultado:
    //
    // Token::IntegerLiteral(12345)
    fn read_number(&mut self) -> Token {
        let start = self.position;

        // Continua enquanto encontrar números
        while self.position < self.source.len()
            && self.source[self.position].is_ascii_digit()
        {
            self.position += 1;
        }

        // Pega os caracteres que representam o número
        let value: String = self.source[start..self.position]
            .iter()
            .collect();

        // Converte String para i32
        Token::IntegerLiteral(
            value.parse::<i32>().unwrap()
        )
    }


    // =============================================
    // IDENTIFIER / KEYWORD
    // =============================================

    // Lê uma palavra.
    //
    // Depois verifica se essa palavra é uma keyword
    // ou simplesmente um identificador.
    //
    // Exemplo:
    //
    // int
    // ↓
    // Token::Int
    //
    // x
    // ↓
    // Token::Identifier("x")
    fn read_identifier(&mut self) -> Token {
        let start = self.position;

        // Identificadores podem conter:
        //
        // letras
        // números
        // _
        //
        // Mas o primeiro caractere precisa ser tratado
        // separadamente no next_token().
        while self.position < self.source.len()
            && (
                self.source[self.position].is_ascii_alphanumeric()
                || self.source[self.position] == '_'
            )
        {
            self.position += 1;
        }

        // Converte os caracteres encontrados para String
        let value: String = self.source[start..self.position]
            .iter()
            .collect();

        // Verifica se é uma keyword
        match value.as_str() {
            "print" => Token::Print,

            "if" => Token::If,

            "else" => Token::Else,

            "int" => Token::Int,

            // Se não for keyword, é um identificador
            _ => Token::Identifier(value),
        }
    }


    // =============================================
    // STRING
    // =============================================

    // Lê uma String.
    //
    // Exemplo:
    //
    // "Hello World"
    //
    // Resultado:
    //
    // Token::StringLiteral("Hello World")
    fn read_string(&mut self) -> Token {
        // Ignora a primeira aspas:
        //
        // "Hello
        // ^
        //
        self.position += 1;

        let start = self.position;

        // Continua até encontrar a segunda aspas
        while self.position < self.source.len()
            && self.source[self.position] != '"'
        {
            self.position += 1;
        }

        // Pega o conteúdo da String
        let value: String = self.source[start..self.position]
            .iter()
            .collect();

        // Ignora a segunda aspas
        self.position += 1;

        Token::StringLiteral(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_variable_declaration() {
        let source = "int::x = 10;";

        let mut lexer = Lexer::new(source);

        let tokens = lexer.tokenize();

        println!("{:?}", tokens);
    }
}