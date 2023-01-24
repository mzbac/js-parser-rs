#[derive(Debug, PartialEq)]
pub enum Token {
    // Single-character tokens
    LeftParen, RightParen, LeftBrace, RightBrace,
    LeftBracket, RightBracket,
    Comma, Dot, Minus, Plus, Semicolon, Slash, Star,

    // One or two character tokens
    Bang, BangEqual,
    Equal, EqualEqual,
    Greater, GreaterEqual,
    Less, LessEqual,
    GreaterGreater, GreaterGreaterEqual,
    LessLess, LessLessEqual,
    PlusPlus, MinusMinus,
    EqualEqualEqual, BangEqualEqual,
    Ampersand, AmpersandAmpersand,
    Pipe, PipePipe,
    Caret, Tilde,
    Question, Colon,


    // Literals
    Identifier(String), String(String), Number(f64),

    // Keywords
    Break, Case, Catch, Class, Const, Continue,
    Debugger, Default, Delete, Do, Else, Export, Extends,
    Finally, For, Function, If, Import, In, Instanceof,
    New, Return, Super, Switch, This, Throw, Try, Typeof,
    Var, Void, While, With,

    // Future reserved words
    Enum,

    // Null literal
    Null,

    // Boolean literals
    True, False,

    // Special word
    Async, Await,
    Get, Set,
    Of,

    EOF
}
