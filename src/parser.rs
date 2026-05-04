use crate::ast::{
    BlockStatement, Expression, Identifier, LetStatement, Program, ReturnStatement, Statement,
};
use crate::lexer::Lexer;
use crate::token::{Token, TokenType};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Precedence {
    Lowest = 1,
    Equals = 2,
    LessGreater = 3,
    Sum = 4,
    Product = 5,
    Prefix = 6,
    Call = 7,
    Index = 8,
}

fn precedence_for(t: &TokenType) -> Precedence {
    match t {
        TokenType::Eq | TokenType::NotEq => Precedence::Equals,
        TokenType::Lt | TokenType::Gt => Precedence::LessGreater,
        TokenType::Plus | TokenType::Minus => Precedence::Sum,
        TokenType::Slash | TokenType::Asterisk => Precedence::Product,
        TokenType::LParen => Precedence::Call,
        TokenType::LBracket => Precedence::Index,
        _ => Precedence::Lowest,
    }
}

pub struct Parser {
    lexer: Lexer,
    curr: Token,
    peek: Token,
    pub errors: Vec<String>,
}

impl Parser {
    pub fn new(mut lexer: Lexer) -> Self {
        let curr = lexer.next_token();
        let peek = lexer.next_token();
        Self { lexer, curr, peek, errors: Vec::new() }
    }

    fn next_token(&mut self) {
        std::mem::swap(&mut self.curr, &mut self.peek);
        self.peek = self.lexer.next_token();
    }

    fn curr_is(&self, t: &TokenType) -> bool {
        &self.curr.ttype == t
    }

    fn peek_is(&self, t: &TokenType) -> bool {
        &self.peek.ttype == t
    }

    fn expect_peek(&mut self, t: TokenType) -> bool {
        if self.peek_is(&t) {
            self.next_token();
            true
        } else {
            self.peek_error(&t);
            false
        }
    }

    fn peek_error(&mut self, t: &TokenType) {
        self.errors.push(format!(
            "expected next token to be {}, got {} instead",
            t, self.peek.ttype
        ));
    }

    fn no_prefix_parse_fn_error(&mut self, t: &TokenType) {
        self.errors.push(format!("no prefix parse functions for {} found", t));
    }

    fn peek_precedence(&self) -> Precedence {
        precedence_for(&self.peek.ttype)
    }

    fn curr_precedence(&self) -> Precedence {
        precedence_for(&self.curr.ttype)
    }

    pub fn parse_program(&mut self) -> Program {
        let mut statements = Vec::new();
        while !self.curr_is(&TokenType::Eof) {
            if let Some(stmt) = self.parse_statement() {
                statements.push(stmt);
            }
            self.next_token();
        }
        Program { statements }
    }

    fn parse_statement(&mut self) -> Option<Statement> {
        match self.curr.ttype {
            TokenType::Let => self.parse_let_statement().map(Statement::Let),
            TokenType::Return => self.parse_return_statement().map(Statement::Return),
            _ => self.parse_expression_statement().map(Statement::Expression),
        }
    }

    fn parse_let_statement(&mut self) -> Option<LetStatement> {
        if !self.expect_peek(TokenType::Ident) {
            return None;
        }
        let name = Identifier { value: self.curr.literal.clone() };
        if !self.expect_peek(TokenType::Assign) {
            return None;
        }
        self.next_token();
        let value = self.parse_expression(Precedence::Lowest)?;
        while !self.curr_is(&TokenType::Semicolon) && !self.curr_is(&TokenType::Eof) {
            self.next_token();
        }
        Some(LetStatement { name, value })
    }

    fn parse_return_statement(&mut self) -> Option<ReturnStatement> {
        self.next_token();
        let value = self.parse_expression(Precedence::Lowest);
        while !self.curr_is(&TokenType::Semicolon) && !self.curr_is(&TokenType::Eof) {
            self.next_token();
        }
        Some(ReturnStatement { value })
    }

    fn parse_expression_statement(&mut self) -> Option<Expression> {
        let expr = self.parse_expression(Precedence::Lowest)?;
        if self.peek_is(&TokenType::Semicolon) {
            self.next_token();
        }
        Some(expr)
    }

    fn parse_expression(&mut self, precedence: Precedence) -> Option<Expression> {
        let mut left = match self.curr.ttype {
            TokenType::Ident => Some(Expression::Identifier(Identifier {
                value: self.curr.literal.clone(),
            })),
            TokenType::Int => self.parse_integer_literal(),
            TokenType::String => Some(Expression::StringLiteral(self.curr.literal.clone())),
            TokenType::Bang | TokenType::Minus => self.parse_prefix_expression(),
            TokenType::True => Some(Expression::Boolean(true)),
            TokenType::False => Some(Expression::Boolean(false)),
            TokenType::LParen => self.parse_grouped_expression(),
            TokenType::If => self.parse_if_expression(),
            TokenType::Function => self.parse_function_literal(),
            TokenType::LBracket => self.parse_array_literal(),
            TokenType::LBrace => self.parse_hash_literal(),
            _ => {
                let t = self.curr.ttype.clone();
                self.no_prefix_parse_fn_error(&t);
                None
            }
        }?;

        while !self.peek_is(&TokenType::Semicolon) && precedence < self.peek_precedence() {
            let has_infix = matches!(
                self.peek.ttype,
                TokenType::Plus
                    | TokenType::Minus
                    | TokenType::Slash
                    | TokenType::Asterisk
                    | TokenType::Eq
                    | TokenType::NotEq
                    | TokenType::Lt
                    | TokenType::Gt
                    | TokenType::LParen
                    | TokenType::LBracket
            );
            if !has_infix {
                return Some(left);
            }
            self.next_token();
            left = match self.curr.ttype {
                TokenType::LParen => self.parse_call_expression(left)?,
                TokenType::LBracket => self.parse_index_expression(left)?,
                _ => self.parse_infix_expression(left)?,
            };
        }
        Some(left)
    }

    fn parse_integer_literal(&mut self) -> Option<Expression> {
        match self.curr.literal.parse::<i64>() {
            Ok(v) => Some(Expression::IntegerLiteral(v)),
            Err(_) => {
                self.errors.push(format!("could not parse {:?} as integer", self.curr.literal));
                None
            }
        }
    }

    fn parse_prefix_expression(&mut self) -> Option<Expression> {
        let operator = self.curr.literal.clone();
        self.next_token();
        let right = self.parse_expression(Precedence::Prefix)?;
        Some(Expression::Prefix { operator, right: Box::new(right) })
    }

    fn parse_infix_expression(&mut self, left: Expression) -> Option<Expression> {
        let operator = self.curr.literal.clone();
        let precedence = self.curr_precedence();
        self.next_token();
        let right = self.parse_expression(precedence)?;
        Some(Expression::Infix { operator, left: Box::new(left), right: Box::new(right) })
    }

    fn parse_grouped_expression(&mut self) -> Option<Expression> {
        self.next_token();
        let exp = self.parse_expression(Precedence::Lowest)?;
        if !self.expect_peek(TokenType::RParen) {
            return None;
        }
        Some(exp)
    }

    fn parse_if_expression(&mut self) -> Option<Expression> {
        if !self.expect_peek(TokenType::LParen) {
            return None;
        }
        self.next_token();
        let condition = self.parse_expression(Precedence::Lowest)?;
        if !self.expect_peek(TokenType::RParen) {
            return None;
        }
        if !self.expect_peek(TokenType::LBrace) {
            return None;
        }
        let consequence = self.parse_block_statement();
        let alternative = if self.peek_is(&TokenType::Else) {
            self.next_token();
            if !self.expect_peek(TokenType::LBrace) {
                return None;
            }
            Some(self.parse_block_statement())
        } else {
            None
        };
        Some(Expression::If {
            condition: Box::new(condition),
            consequence,
            alternative,
        })
    }

    fn parse_block_statement(&mut self) -> BlockStatement {
        let mut statements = Vec::new();
        self.next_token();
        while !self.curr_is(&TokenType::RBrace) && !self.curr_is(&TokenType::Eof) {
            if let Some(s) = self.parse_statement() {
                statements.push(s);
            }
            self.next_token();
        }
        BlockStatement { statements }
    }

    fn parse_function_literal(&mut self) -> Option<Expression> {
        if !self.expect_peek(TokenType::LParen) {
            return None;
        }
        let parameters = self.parse_function_parameters()?;
        if !self.expect_peek(TokenType::LBrace) {
            return None;
        }
        let body = self.parse_block_statement();
        Some(Expression::Function { parameters, body })
    }

    fn parse_function_parameters(&mut self) -> Option<Vec<Identifier>> {
        let mut idents = Vec::new();
        if self.peek_is(&TokenType::RParen) {
            self.next_token();
            return Some(idents);
        }
        self.next_token();
        idents.push(Identifier { value: self.curr.literal.clone() });
        while self.peek_is(&TokenType::Comma) {
            self.next_token();
            self.next_token();
            idents.push(Identifier { value: self.curr.literal.clone() });
        }
        if !self.expect_peek(TokenType::RParen) {
            return None;
        }
        Some(idents)
    }

    fn parse_call_expression(&mut self, function: Expression) -> Option<Expression> {
        let arguments = self.parse_expression_list(TokenType::RParen)?;
        Some(Expression::Call { function: Box::new(function), arguments })
    }

    fn parse_array_literal(&mut self) -> Option<Expression> {
        let elements = self.parse_expression_list(TokenType::RBracket)?;
        Some(Expression::Array(elements))
    }

    fn parse_expression_list(&mut self, end: TokenType) -> Option<Vec<Expression>> {
        let mut list = Vec::new();
        if self.peek_is(&end) {
            self.next_token();
            return Some(list);
        }
        self.next_token();
        list.push(self.parse_expression(Precedence::Lowest)?);
        while self.peek_is(&TokenType::Comma) {
            self.next_token();
            self.next_token();
            list.push(self.parse_expression(Precedence::Lowest)?);
        }
        if !self.expect_peek(end) {
            return None;
        }
        Some(list)
    }

    fn parse_index_expression(&mut self, left: Expression) -> Option<Expression> {
        self.next_token();
        let index = self.parse_expression(Precedence::Lowest)?;
        if !self.expect_peek(TokenType::RBracket) {
            return None;
        }
        Some(Expression::Index { left: Box::new(left), index: Box::new(index) })
    }

    fn parse_hash_literal(&mut self) -> Option<Expression> {
        let mut pairs = Vec::new();
        while !self.peek_is(&TokenType::RBrace) {
            self.next_token();
            let key = self.parse_expression(Precedence::Lowest)?;
            if !self.expect_peek(TokenType::Colon) {
                return None;
            }
            self.next_token();
            let value = self.parse_expression(Precedence::Lowest)?;
            pairs.push((key, value));
            if !self.peek_is(&TokenType::RBrace) && !self.expect_peek(TokenType::Comma) {
                return None;
            }
        }
        if !self.expect_peek(TokenType::RBrace) {
            return None;
        }
        Some(Expression::Hash(pairs))
    }
}
