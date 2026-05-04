use std::collections::HashMap;
use std::rc::Rc;

use crate::ast::{BlockStatement, Expression, Program, Statement};
use crate::builtins::lookup_builtin;
use crate::environment::{EnvRef, Environment};
use crate::object::{HashPair, Object};

pub fn eval_program(program: &Program, env: EnvRef) -> Object {
    let mut result = Object::Null;
    for stmt in &program.statements {
        result = eval_statement(stmt, env.clone());
        match result {
            Object::ReturnValue(v) => return *v,
            Object::Error(_) => return result,
            _ => {}
        }
    }
    result
}

pub fn eval_statement(stmt: &Statement, env: EnvRef) -> Object {
    match stmt {
        Statement::Expression(e) => eval_expression(e, env),
        Statement::Return(rs) => {
            let val = match &rs.value {
                Some(e) => eval_expression(e, env),
                None => Object::Null,
            };
            if val.is_error() {
                return val;
            }
            Object::ReturnValue(Box::new(val))
        }
        Statement::Let(ls) => {
            let val = eval_expression(&ls.value, env.clone());
            if val.is_error() {
                return val;
            }
            env.borrow_mut().set(ls.name.value.clone(), val);
            Object::Null
        }
        Statement::Block(b) => eval_block(b, env),
    }
}

fn eval_block(block: &BlockStatement, env: EnvRef) -> Object {
    let mut result = Object::Null;
    for stmt in &block.statements {
        result = eval_statement(stmt, env.clone());
        match result {
            Object::ReturnValue(_) | Object::Error(_) => return result,
            _ => {}
        }
    }
    result
}

pub fn eval_expression(expr: &Expression, env: EnvRef) -> Object {
    match expr {
        Expression::IntegerLiteral(n) => Object::Integer(*n),
        Expression::StringLiteral(s) => Object::String(s.clone()),
        Expression::Boolean(b) => Object::Boolean(*b),
        Expression::Identifier(id) => {
            if let Some(v) = env.borrow().get(&id.value) {
                return v;
            }
            if let Some(b) = lookup_builtin(&id.value) {
                return Object::Builtin(b);
            }
            new_error(format!("identifier not found: {}", id.value))
        }
        Expression::Prefix { operator, right } => {
            let r = eval_expression(right, env);
            if r.is_error() {
                return r;
            }
            eval_prefix(operator, r)
        }
        Expression::Infix { operator, left, right } => {
            let l = eval_expression(left, env.clone());
            if l.is_error() {
                return l;
            }
            let r = eval_expression(right, env);
            if r.is_error() {
                return r;
            }
            eval_infix(operator, l, r)
        }
        Expression::If { condition, consequence, alternative } => {
            let cond = eval_expression(condition, env.clone());
            if cond.is_error() {
                return cond;
            }
            if is_truthy(&cond) {
                eval_block(consequence, env)
            } else if let Some(alt) = alternative {
                eval_block(alt, env)
            } else {
                Object::Null
            }
        }
        Expression::Function { parameters, body } => Object::Function {
            parameters: parameters.clone(),
            body: body.clone(),
            env: env.clone(),
        },
        Expression::Call { function, arguments } => {
            let func = eval_expression(function, env.clone());
            if func.is_error() {
                return func;
            }
            let args = eval_expressions(arguments, env);
            if args.len() == 1 && args[0].is_error() {
                return args.into_iter().next().unwrap();
            }
            apply_function(func, args)
        }
        Expression::Array(elements) => {
            let els = eval_expressions(elements, env);
            if els.len() == 1 && els[0].is_error() {
                return els.into_iter().next().unwrap();
            }
            Object::Array(els)
        }
        Expression::Index { left, index } => {
            let l = eval_expression(left, env.clone());
            if l.is_error() {
                return l;
            }
            let i = eval_expression(index, env);
            if i.is_error() {
                return i;
            }
            eval_index(l, i)
        }
        Expression::Hash(pairs) => eval_hash(pairs, env),
    }
}

fn eval_expressions(exps: &[Expression], env: EnvRef) -> Vec<Object> {
    let mut result = Vec::with_capacity(exps.len());
    for e in exps {
        let v = eval_expression(e, env.clone());
        if v.is_error() {
            return vec![v];
        }
        result.push(v);
    }
    result
}

fn eval_prefix(operator: &str, right: Object) -> Object {
    match operator {
        "!" => match right {
            Object::Boolean(true) => Object::Boolean(false),
            Object::Boolean(false) => Object::Boolean(true),
            Object::Null => Object::Boolean(true),
            _ => Object::Boolean(false),
        },
        "-" => match right {
            Object::Integer(n) => Object::Integer(-n),
            other => new_error(format!("unknown operator: -{}", other.type_name())),
        },
        op => new_error(format!("unknown operator: {}{}", op, right.type_name())),
    }
}

fn eval_infix(operator: &str, left: Object, right: Object) -> Object {
    if left.type_name() != right.type_name() {
        return new_error(format!(
            "type mismatch: {} {} {}",
            left.type_name(),
            operator,
            right.type_name()
        ));
    }
    match (&left, &right) {
        (Object::Integer(a), Object::Integer(b)) => eval_integer_infix(operator, *a, *b),
        (Object::String(a), Object::String(b)) => {
            if operator == "+" {
                Object::String(format!("{}{}", a, b))
            } else {
                new_error(format!("unknown operator: STRING {} STRING", operator))
            }
        }
        (Object::Boolean(a), Object::Boolean(b)) => match operator {
            "==" => Object::Boolean(a == b),
            "!=" => Object::Boolean(a != b),
            _ => new_error(format!("unknown operator: BOOLEAN {} BOOLEAN", operator)),
        },
        _ => new_error(format!(
            "unknown operator: {} {} {}",
            left.type_name(),
            operator,
            right.type_name()
        )),
    }
}

fn eval_integer_infix(operator: &str, a: i64, b: i64) -> Object {
    match operator {
        "+" => Object::Integer(a + b),
        "-" => Object::Integer(a - b),
        "*" => Object::Integer(a * b),
        "/" => Object::Integer(a / b),
        "<" => Object::Boolean(a < b),
        ">" => Object::Boolean(a > b),
        "==" => Object::Boolean(a == b),
        "!=" => Object::Boolean(a != b),
        op => new_error(format!("unknown operator: INTEGER {} INTEGER", op)),
    }
}

fn is_truthy(obj: &Object) -> bool {
    match obj {
        Object::Null => false,
        Object::Boolean(b) => *b,
        _ => true,
    }
}

fn eval_index(left: Object, index: Object) -> Object {
    match (&left, &index) {
        (Object::Array(els), Object::Integer(i)) => {
            let idx = *i;
            if idx < 0 || idx as usize >= els.len() {
                Object::Null
            } else {
                els[idx as usize].clone()
            }
        }
        (Object::Hash(map), _) => match index.hash_key() {
            Some(k) => match map.get(&k) {
                Some(p) => p.value.clone(),
                None => Object::Null,
            },
            None => new_error(format!("unusable as hash key: {}", index.type_name())),
        },
        _ => new_error(format!("index operator not supported: {}", left.type_name())),
    }
}

fn eval_hash(pairs: &[(Expression, Expression)], env: EnvRef) -> Object {
    let mut map: HashMap<_, _> = HashMap::new();
    for (k_expr, v_expr) in pairs {
        let key = eval_expression(k_expr, env.clone());
        if key.is_error() {
            return key;
        }
        let hk = match key.hash_key() {
            Some(k) => k,
            None => return new_error(format!("unusable as hash key: {}", key.type_name())),
        };
        let value = eval_expression(v_expr, env.clone());
        if value.is_error() {
            return value;
        }
        map.insert(hk, HashPair { key, value });
    }
    Object::Hash(Rc::new(map))
}

fn apply_function(func: Object, args: Vec<Object>) -> Object {
    match func {
        Object::Function { parameters, body, env } => {
            let extended = Environment::new_enclosed(env);
            for (param, arg) in parameters.iter().zip(args.into_iter()) {
                extended.borrow_mut().set(param.value.clone(), arg);
            }
            let evaluated = eval_block(&body, extended);
            unwrap_return(evaluated)
        }
        Object::Builtin(f) => f(&args),
        other => new_error(format!("not a function {}", other.type_name())),
    }
}

fn unwrap_return(obj: Object) -> Object {
    match obj {
        Object::ReturnValue(v) => *v,
        other => other,
    }
}

pub fn new_error(msg: impl Into<String>) -> Object {
    Object::Error(msg.into())
}
