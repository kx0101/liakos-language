use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub struct Identifier {
    pub value: String,
}

impl fmt::Display for Identifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    Let(LetStatement),
    Return(ReturnStatement),
    Expression(Expression),
    Block(BlockStatement),
}

#[derive(Debug, Clone, PartialEq)]
pub struct LetStatement {
    pub name: Identifier,
    pub value: Expression,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ReturnStatement {
    pub value: Option<Expression>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BlockStatement {
    pub statements: Vec<Statement>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Identifier(Identifier),
    IntegerLiteral(i64),
    StringLiteral(String),
    Boolean(bool),
    Prefix { operator: String, right: Box<Expression> },
    Infix { operator: String, left: Box<Expression>, right: Box<Expression> },
    If { condition: Box<Expression>, consequence: BlockStatement, alternative: Option<BlockStatement> },
    Function { parameters: Vec<Identifier>, body: BlockStatement },
    Call { function: Box<Expression>, arguments: Vec<Expression> },
    Array(Vec<Expression>),
    Index { left: Box<Expression>, index: Box<Expression> },
    Hash(Vec<(Expression, Expression)>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Program {
    pub statements: Vec<Statement>,
}

impl fmt::Display for Program {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for s in &self.statements {
            write!(f, "{}", s)?;
        }
        Ok(())
    }
}

impl fmt::Display for Statement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Statement::Let(ls) => write!(f, "let {} = {};", ls.name, ls.value),
            Statement::Return(rs) => match &rs.value {
                Some(v) => write!(f, "return {};", v),
                None => f.write_str("return ;"),
            },
            Statement::Expression(e) => write!(f, "{}", e),
            Statement::Block(b) => {
                for s in &b.statements {
                    write!(f, "{}", s)?;
                }
                Ok(())
            }
        }
    }
}

impl fmt::Display for BlockStatement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for s in &self.statements {
            write!(f, "{}", s)?;
        }
        Ok(())
    }
}

fn join<T: fmt::Display>(items: &[T], sep: &str) -> String {
    items.iter().map(|i| i.to_string()).collect::<Vec<_>>().join(sep)
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expression::Identifier(i) => write!(f, "{}", i),
            Expression::IntegerLiteral(n) => write!(f, "{}", n),
            Expression::StringLiteral(s) => write!(f, "{}", s),
            Expression::Boolean(b) => write!(f, "{}", b),
            Expression::Prefix { operator, right } => write!(f, "({}{})", operator, right),
            Expression::Infix { operator, left, right } => write!(f, "({} {} {})", left, operator, right),
            Expression::If { condition, consequence, alternative } => {
                write!(f, "if{} {}", condition, consequence)?;
                if let Some(alt) = alternative {
                    write!(f, "else {}", alt)?;
                }
                Ok(())
            }
            Expression::Function { parameters, body } => {
                write!(f, "fn({}) {}", join(parameters, ","), body)
            }
            Expression::Call { function, arguments } => {
                write!(f, "{}({})", function, join(arguments, ", "))
            }
            Expression::Array(elements) => write!(f, "[{}]", join(elements, ", ")),
            Expression::Index { left, index } => write!(f, "({}[{}])", left, index),
            Expression::Hash(pairs) => {
                let parts: Vec<String> = pairs.iter().map(|(k, v)| format!("{}:{}", k, v)).collect();
                write!(f, "{{{}}}", parts.join(", "))
            }
        }
    }
}
