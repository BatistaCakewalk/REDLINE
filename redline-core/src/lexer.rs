#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Let,            // 'let'
    Def,            // 'def'
    Identifier(String), // 'x', 'my_func'
    Integer(i64),   // '100'
    StringLit(String), // "Hello"
    Colon,          // ':'
    Equals,         // '='
    Plus,           // '+'
    LParen,         // '('
    RParen,         // ')'
    Indent,         // For Python-like scope
    Dedent,
    EOF,
}
