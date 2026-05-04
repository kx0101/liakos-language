use crate::token::{lookup_ident, Token, TokenType};

pub struct Lexer {
    input: Vec<u8>,
    position: usize,
    read_position: usize,
    ch: u8,
}

impl Lexer {
    pub fn new(input: impl Into<String>) -> Self {
        let s: String = input.into();
        let mut l = Self {
            input: s.into_bytes(),
            position: 0,
            read_position: 0,
            ch: 0,
        };
        l.read_char();
        l
    }

    fn read_char(&mut self) {
        self.ch = if self.read_position >= self.input.len() {
            0
        } else {
            self.input[self.read_position]
        };
        self.position = self.read_position;
        self.read_position += 1;
    }

    fn peek_char(&self) -> u8 {
        if self.read_position >= self.input.len() {
            0
        } else {
            self.input[self.read_position]
        }
    }

    fn skip_whitespace(&mut self) {
        while matches!(self.ch, b' ' | b'\t' | b'\n' | b'\r') {
            self.read_char();
        }
    }

    fn read_identifier(&mut self) -> String {
        let start = self.position;
        while is_letter(self.ch) {
            self.read_char();
        }
        String::from_utf8_lossy(&self.input[start..self.position]).into_owned()
    }

    fn read_number(&mut self) -> String {
        let start = self.position;
        while is_digit(self.ch) {
            self.read_char();
        }
        String::from_utf8_lossy(&self.input[start..self.position]).into_owned()
    }

    fn read_string(&mut self) -> String {
        let start = self.position + 1;
        loop {
            self.read_char();
            if self.ch == b'"' || self.ch == 0 {
                break;
            }
        }
        String::from_utf8_lossy(&self.input[start..self.position]).into_owned()
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        let tok = match self.ch {
            b'=' => {
                if self.peek_char() == b'=' {
                    self.read_char();
                    Token::new(TokenType::Eq, "==")
                } else {
                    Token::new(TokenType::Assign, "=")
                }
            }
            b'+' => Token::new(TokenType::Plus, "+"),
            b'-' => Token::new(TokenType::Minus, "-"),
            b'!' => {
                if self.peek_char() == b'=' {
                    self.read_char();
                    Token::new(TokenType::NotEq, "!=")
                } else {
                    Token::new(TokenType::Bang, "!")
                }
            }
            b'/' => Token::new(TokenType::Slash, "/"),
            b'*' => Token::new(TokenType::Asterisk, "*"),
            b'<' => Token::new(TokenType::Lt, "<"),
            b'>' => Token::new(TokenType::Gt, ">"),
            b';' => Token::new(TokenType::Semicolon, ";"),
            b',' => Token::new(TokenType::Comma, ","),
            b'(' => Token::new(TokenType::LParen, "("),
            b')' => Token::new(TokenType::RParen, ")"),
            b'{' => Token::new(TokenType::LBrace, "{"),
            b'}' => Token::new(TokenType::RBrace, "}"),
            b'[' => Token::new(TokenType::LBracket, "["),
            b']' => Token::new(TokenType::RBracket, "]"),
            b':' => Token::new(TokenType::Colon, ":"),
            0 => Token::new(TokenType::Eof, ""),
            b'"' => {
                let s = self.read_string();
                Token::new(TokenType::String, s)
            }
            c if is_letter(c) => {
                let lit = self.read_identifier();
                let tt = lookup_ident(&lit);
                return Token::new(tt, lit);
            }
            c if is_digit(c) => {
                let lit = self.read_number();
                return Token::new(TokenType::Int, lit);
            }
            c => Token::new(TokenType::Illegal, (c as char).to_string()),
        };

        self.read_char();
        tok
    }
}

fn is_letter(c: u8) -> bool {
    c.is_ascii_alphabetic() || c == b'_'
}

fn is_digit(c: u8) -> bool {
    c.is_ascii_digit()
}
