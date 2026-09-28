#[derive(Debug)]
pub enum Statement{
    VariableDeclaration{
        name: String,
        type_: Type,
        value: Expression,
    },

    Assignment {
        name: String,
        value: Expression,
    },

    Print {
        expression: Expression
    },
}

#[derive(Debug)]
pub enum Type {
    Int,
    String,
}
#[derive(Debug)]
pub enum Expression {
    IntegerLiteral(i32),
    StringLiteral(String),
    Identifier(String),

    Binary {
        left: Box<Expression>,
        operator: BinaryOperator,
        right: Box<Expression>
    }
}
#[derive(Debug)]
pub enum BinaryOperator {
    Add,
    LessThan,
    GreaterThan
}