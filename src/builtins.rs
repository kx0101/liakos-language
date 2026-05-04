use crate::evaluator::new_error;
use crate::object::{BuiltinFn, Object};

pub fn lookup_builtin(name: &str) -> Option<BuiltinFn> {
    match name {
        "len" => Some(b_len),
        "first" => Some(b_first),
        "last" => Some(b_last),
        "rest" => Some(b_rest),
        "push" => Some(b_push),
        "print" => Some(b_print),
        _ => None,
    }
}

fn check_arity(args: &[Object], want: usize) -> Option<Object> {
    if args.len() != want {
        Some(new_error(format!(
            "wrong number of arguments. got={}, want={}",
            args.len(),
            want
        )))
    } else {
        None
    }
}

fn b_len(args: &[Object]) -> Object {
    if let Some(e) = check_arity(args, 1) {
        return e;
    }
    match &args[0] {
        Object::String(s) => Object::Integer(s.len() as i64),
        Object::Array(els) => Object::Integer(els.len() as i64),
        other => new_error(format!("argument to `len` not supported, got {}", other.type_name())),
    }
}

fn b_first(args: &[Object]) -> Object {
    if let Some(e) = check_arity(args, 1) {
        return e;
    }
    match &args[0] {
        Object::Array(els) => els.first().cloned().unwrap_or(Object::Null),
        other => new_error(format!("argument to `first` must be ARRAY, got {}", other.type_name())),
    }
}

fn b_last(args: &[Object]) -> Object {
    if let Some(e) = check_arity(args, 1) {
        return e;
    }
    match &args[0] {
        Object::Array(els) => els.last().cloned().unwrap_or(Object::Null),
        other => new_error(format!("argument to `last` must be ARRAY, got {}", other.type_name())),
    }
}

fn b_rest(args: &[Object]) -> Object {
    if let Some(e) = check_arity(args, 1) {
        return e;
    }
    match &args[0] {
        Object::Array(els) => {
            if els.is_empty() {
                Object::Null
            } else {
                Object::Array(els[1..].to_vec())
            }
        }
        other => new_error(format!("argument to `rest` must be ARRAY, got {}", other.type_name())),
    }
}

fn b_push(args: &[Object]) -> Object {
    if let Some(e) = check_arity(args, 2) {
        return e;
    }
    match &args[0] {
        Object::Array(els) => {
            let mut new = els.clone();
            new.push(args[1].clone());
            Object::Array(new)
        }
        other => new_error(format!("argument to `push` must be ARRAY, got {}", other.type_name())),
    }
}

fn b_print(args: &[Object]) -> Object {
    for a in args {
        println!("{}", a.inspect());
    }
    Object::Null
}
