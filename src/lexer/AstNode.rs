pub enum AstNode {
    NumberLiteral(f64),
    StringLiteral(String),
    Identifier(String),
    CallExpression {
        callee: Box<AstNode>,
        arguments: Vec<AstNode>,
    },
    BinaryExpression {
        operator: String,
        left: Box<AstNode>,
        right: Box<AstNode>,
    },
    AssignmentExpression {
        operator: String,
        left: Box<AstNode>,
        right: Box<AstNode>,
    },
    VariableDeclaration {
        id: Box<AstNode>,
        init: Box<AstNode>
    },
    BlockStatement {
        body: Vec<AstNode>
    },
    FunctionDeclaration {
        id: Box<AstNode>,
        params: Vec<AstNode>,
        body: Box<AstNode>
    },
    Program {
        body: Vec<AstNode>
    }
}
