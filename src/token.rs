use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TokenType {
    Illegal,
    Eof,

    Ident,
    Int,
    String,

    Assign,
    Plus,
    Minus,
    Bang,
    Asterisk,
    Slash,

    Lt,
    Gt,
    Eq,
    NotEq,

    Comma,
    Semicolon,
    Colon,

    LParen,
    RParen,
    LBrace,
    RBrace,
    LBracket,
    RBracket,

    Function,
    Let,
    True,
    False,
    If,
    Else,
    Return,
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use TokenType::*;
        let s = match self {
            Illegal => "ILLEGAL",
            Eof => "EOF",
            Ident => "IDENT",
            Int => "INT",
            String => "STRING",
            Assign => "=",
            Plus => "+",
            Minus => "-",
            Bang => "!",
            Asterisk => "*",
            Slash => "/",
            Lt => "<",
            Gt => ">",
            Eq => "==",
            NotEq => "!=",
            Comma => ",",
            Semicolon => ";",
            Colon => ":",
            LParen => "(",
            RParen => ")",
            LBrace => "{",
            RBrace => "}",
            LBracket => "[",
            RBracket => "]",
            Function => "FUNCTION",
            Let => "LET",
            True => "TRUE",
            False => "FALSE",
            If => "IF",
            Else => "ELSE",
            Return => "RETURN",
        };
        f.write_str(s)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
    pub ttype: TokenType,
    pub literal: String,
}

impl Token {
    pub fn new(ttype: TokenType, literal: impl Into<String>) -> Self {
        Self { ttype, literal: literal.into() }
    }
}

pub fn lookup_ident(ident: &str) -> TokenType {
    match ident {
        "fn" => TokenType::Function,
        "let" => TokenType::Let,
        "true" => TokenType::True,
        "false" => TokenType::False,
        "if" => TokenType::If,
        "else" => TokenType::Else,
        "return" => TokenType::Return,
        _ => TokenType::Ident,
    }
}
