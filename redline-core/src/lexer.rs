use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Var, Val, Def, Pub, Print, Return, If, Else,
    Ident(String), Int(i64), Float(f64), Str(String), Type(String),
    Op(String), Arrow, Colon, Assign, LParen, RParen, Comma, Newline,
    Indent, Dedent,
}

#[derive(Debug)]
pub struct LexerError {
    pub message: String,
    pub line: usize,
    pub column: usize,
}

impl fmt::Display for LexerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} at line {}, column {}", self.message, self.line, self.column)
    }
}

pub struct Lexer {
    input: Vec<char>,
    pos: usize,
    line: usize,
    column: usize,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        Self {
            input: input.chars().collect(),
            pos: 0,
            line: 1,
            column: 1,
        }
    }

    fn advance(&mut self) {
        if self.pos < self.input.len() {
            if self.input[self.pos] == '\n' {
                self.line += 1;
                self.column = 1;
            } else {
                self.column += 1;
            }
            self.pos += 1;
        }
    }

    pub fn tokenize(&mut self) -> Result<Vec<Token>, LexerError> {
        let mut tokens = Vec::new();
        let mut indent_stack = vec![0];

        while self.pos < self.input.len() {
            // Handle indentation at the start of a line
            if self.column == 1 {
                let mut spaces = 0;
                let mut lookahead = self.pos;
                let mut is_empty_line = false;

                while lookahead < self.input.len() {
                    match self.input[lookahead] {
                        ' ' => spaces += 1,
                        '\t' => spaces += 4,
                        '\n' => {
                            is_empty_line = true;
                            break;
                        }
                        '\r' => {},
                        _ => break,
                    }
                    lookahead += 1;
                }

                if is_empty_line {
                    // Skip whitespace on empty lines
                    while self.pos < lookahead {
                        self.advance();
                    }
                    // Let the main loop handle the newline
                } else {
                    // Handle indentation changes
                    let last_indent = *indent_stack.last().unwrap();
                    if spaces > last_indent {
                        indent_stack.push(spaces);
                        tokens.push(Token::Indent);
                    } else if spaces < last_indent {
                        while spaces < *indent_stack.last().unwrap() {
                            indent_stack.pop();
                            tokens.push(Token::Dedent);
                        }
                        if spaces != *indent_stack.last().unwrap() {
                            return Err(LexerError {
                                message: "Unindent does not match any outer indentation level".to_string(),
                                line: self.line,
                                column: self.column,
                            });
                        }
                    }

                    // Consume indentation
                    while self.pos < lookahead {
                        self.advance();
                    }
                }
            }

            if self.pos >= self.input.len() {
                break;
            }

            let c = self.input[self.pos];
            match c {
                ' ' | '\r' | '\t' => self.advance(),
                '\n' => {
                    tokens.push(Token::Newline);
                    self.advance();
                }
                ':' => {
                    tokens.push(Token::Colon);
                    self.advance();
                }
                '=' => {
                    if self.pos + 1 < self.input.len() && self.input[self.pos + 1] == '=' {
                        tokens.push(Token::Op("==".to_string()));
                        self.advance();
                        self.advance();
                    } else {
                        tokens.push(Token::Assign);
                        self.advance();
                    }
                }
                '(' => {
                    tokens.push(Token::LParen);
                    self.advance();
                }
                ')' => {
                    tokens.push(Token::RParen);
                    self.advance();
                }
                ',' => {
                    tokens.push(Token::Comma);
                    self.advance();
                }
                '>' | '<' | '!' => {
                    let next = self.input.get(self.pos + 1);
                    if next == Some(&'=') {
                        tokens.push(Token::Op(format!("{}=", c)));
                        self.advance();
                        self.advance();
                    } else {
                        tokens.push(Token::Op(c.to_string()));
                        self.advance();
                    }
                }
                '+' | '*' | '/' => {
                    tokens.push(Token::Op(c.to_string()));
                    self.advance();
                }
                '-' => {
                    if self.pos + 1 < self.input.len() && self.input[self.pos + 1] == '>' {
                        tokens.push(Token::Arrow);
                        self.advance();
                        self.advance();
                    } else {
                        tokens.push(Token::Op("-".to_string()));
                        self.advance();
                    }
                }
                '#' => {
                    while self.pos < self.input.len() && self.input[self.pos] != '\n' {
                        self.advance();
                    }
                }
                '"' => {
                    self.advance();
                    let mut s = String::new();
                    while self.pos < self.input.len() {
                        if self.input[self.pos] == '"' {
                            break;
                        }
                        if self.input[self.pos] == '\\' {
                            self.advance();
                            if self.pos < self.input.len() {
                                match self.input[self.pos] {
                                    'n' => s.push('\n'),
                                    't' => s.push('\t'),
                                    'r' => s.push('\r'),
                                    '\\' => s.push('\\'),
                                    '"' => s.push('"'),
                                    _ => s.push(self.input[self.pos]),
                                }
                            }
                        } else {
                            s.push(self.input[self.pos]);
                        }
                        self.advance();
                    }
                    if self.pos < self.input.len() && self.input[self.pos] == '"' {
                        tokens.push(Token::Str(s));
                        self.advance();
                    } else {
                        return Err(LexerError {
                            message: "Unterminated string literal".to_string(),
                            line: self.line,
                            column: self.column,
                        });
                    }
                }
                _ if c.is_alphabetic() => {
                    let mut ident = String::new();
                    while self.pos < self.input.len() 
                        && (self.input[self.pos].is_alphanumeric() || self.input[self.pos] == '_')
                    {
                        ident.push(self.input[self.pos]);
                        self.advance();
                    }
                    match ident.as_str() {
                        "var" => tokens.push(Token::Var),
                        "val" => tokens.push(Token::Val),
                        "def" => tokens.push(Token::Def),
                        "if" => tokens.push(Token::If),
                        "else" => tokens.push(Token::Else),
                        "pub" => tokens.push(Token::Pub),
                        "return" => tokens.push(Token::Return),
                        "print" => tokens.push(Token::Print),
                        "int" | "float" | "string" => tokens.push(Token::Type(ident)),
                        _ => tokens.push(Token::Ident(ident)),
                    }
                }
                _ if c.is_numeric() || c == '.' => {
                    let mut num = String::new();
                    let mut is_float = false;

                    if c == '.' {
                        is_float = true;
                        num.push('0');
                        num.push('.');
                        self.advance();
                    }

                    while self.pos < self.input.len() && (self.input[self.pos].is_numeric() || self.input[self.pos] == '.') {
                        if self.input[self.pos] == '.' {
                            if is_float {
                                return Err(LexerError {
                                    message: format!("Invalid number: multiple decimal points"),
                                    line: self.line,
                                    column: self.column,
                                });
                            }
                            is_float = true;
                        }
                        num.push(self.input[self.pos]);
                        self.advance();
                    }

                    if is_float {
                        match num.parse() {
                            Ok(n) => tokens.push(Token::Float(n)),
                            Err(_) => {
                                return Err(LexerError {
                                    message: format!("Invalid float: {}", num),
                                    line: self.line,
                                    column: self.column,
                                })
                            }
                        }
                    } else {
                        match num.parse() {
                            Ok(n) => tokens.push(Token::Int(n)),
                            Err(_) => {
                                return Err(LexerError {
                                    message: format!("Invalid integer: {}", num),
                                    line: self.line,
                                    column: self.column,
                                })
                            }
                        }
                    }
                }
                _ => {
                    return Err(LexerError {
                        message: format!("Unknown character: {}", c),
                        line: self.line,
                        column: self.column,
                    });
                }
            }
        }

        // Emit remaining Dedents
        while indent_stack.len() > 1 {
            indent_stack.pop();
            tokens.push(Token::Dedent);
        }

        Ok(tokens)
    }
}