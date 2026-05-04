use std::io::{BufRead, Write};

use crate::environment::Environment;
use crate::evaluator::eval_program;
use crate::lexer::Lexer;
use crate::parser::Parser;

const PROMPT: &str = ">> ";

pub fn start<R: BufRead, W: Write>(mut input: R, mut output: W) {
    let env = Environment::new();
    let mut line = String::new();
    loop {
        write!(output, "{}", PROMPT).ok();
        output.flush().ok();
        line.clear();
        match input.read_line(&mut line) {
            Ok(0) => return,
            Ok(_) => {}
            Err(_) => return,
        }
        let lexer = Lexer::new(line.trim_end_matches(['\n', '\r']).to_string());
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();
        if !parser.errors.is_empty() {
            print_parser_errors(&mut output, &parser.errors);
            continue;
        }
        let evaluated = eval_program(&program, env.clone());
        writeln!(output, "{}", evaluated.inspect()).ok();
    }
}

fn print_parser_errors<W: Write>(out: &mut W, errors: &[String]) {
    writeln!(out, "Error").ok();
    writeln!(out, " parser errors:").ok();
    for msg in errors {
        writeln!(out, "\t{}", msg).ok();
    }
}
