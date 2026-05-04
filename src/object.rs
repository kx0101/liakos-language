use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;

use crate::ast::{BlockStatement, Identifier};
use crate::environment::EnvRef;

pub type BuiltinFn = fn(&[Object]) -> Object;

#[derive(Debug, Clone)]
pub enum Object {
    Integer(i64),
    Boolean(bool),
    String(String),
    Null,
    ReturnValue(Box<Object>),
    Error(String),
    Function {
        parameters: Vec<Identifier>,
        body: BlockStatement,
        env: EnvRef,
    },
    Builtin(BuiltinFn),
    Array(Vec<Object>),
    Hash(Rc<HashMap<HashKey, HashPair>>),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum HashKey {
    Integer(i64),
    Boolean(bool),
    String(String),
}

#[derive(Debug, Clone)]
pub struct HashPair {
    pub key: Object,
    pub value: Object,
}

impl Object {
    pub fn type_name(&self) -> &'static str {
        match self {
            Object::Integer(_) => "INTEGER",
            Object::Boolean(_) => "BOOLEAN",
            Object::String(_) => "STRING",
            Object::Null => "NULL",
            Object::ReturnValue(_) => "RETURN_VALUE",
            Object::Error(_) => "ERROR",
            Object::Function { .. } => "FUNCTION",
            Object::Builtin(_) => "BUILTIN",
            Object::Array(_) => "ARRAY",
            Object::Hash(_) => "HASH",
        }
    }

    pub fn inspect(&self) -> String {
        match self {
            Object::Integer(n) => n.to_string(),
            Object::Boolean(b) => b.to_string(),
            Object::String(s) => s.clone(),
            Object::Null => "null".to_string(),
            Object::ReturnValue(v) => v.inspect(),
            Object::Error(m) => format!("ERROR: {}", m),
            Object::Function { parameters, body, .. } => {
                let params: Vec<String> = parameters.iter().map(|p| p.to_string()).collect();
                format!("fn({}) {{\n{}\n}}", params.join(", "), body)
            }
            Object::Builtin(_) => "builtin function".to_string(),
            Object::Array(els) => {
                let parts: Vec<String> = els.iter().map(|e| e.inspect()).collect();
                format!("[{}]", parts.join(", "))
            }
            Object::Hash(map) => {
                let parts: Vec<String> = map
                    .values()
                    .map(|p| format!("{}: {}", p.key.inspect(), p.value.inspect()))
                    .collect();
                format!("{{{}}}", parts.join(", "))
            }
        }
    }

    pub fn hash_key(&self) -> Option<HashKey> {
        match self {
            Object::Integer(n) => Some(HashKey::Integer(*n)),
            Object::Boolean(b) => Some(HashKey::Boolean(*b)),
            Object::String(s) => Some(HashKey::String(s.clone())),
            _ => None,
        }
    }

    pub fn is_error(&self) -> bool {
        matches!(self, Object::Error(_))
    }
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.inspect())
    }
}
