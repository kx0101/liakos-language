use liakos::environment::Environment;
use liakos::evaluator::eval_program;
use liakos::lexer::Lexer;
use liakos::object::Object;
use liakos::parser::Parser;

fn run(input: &str) -> Object {
    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program();
    assert!(parser.errors.is_empty(), "parser errors: {:?}", parser.errors);
    let env = Environment::new();
    eval_program(&program, env)
}

fn assert_int(input: &str, expected: i64) {
    match run(input) {
        Object::Integer(n) => assert_eq!(n, expected, "input: {}", input),
        other => panic!("expected Integer({}), got {:?} for {}", expected, other, input),
    }
}

fn assert_bool(input: &str, expected: bool) {
    match run(input) {
        Object::Boolean(b) => assert_eq!(b, expected, "input: {}", input),
        other => panic!("expected Boolean({}), got {:?} for {}", expected, other, input),
    }
}

#[test]
fn integer_arithmetic() {
    let cases = [
        ("5", 5),
        ("10", 10),
        ("-5", -5),
        ("5 + 5 + 5 + 5 - 10", 10),
        ("2 * 2 * 2 * 2 * 2", 32),
        ("-50 + 100 + -50", 0),
        ("5 * 2 + 10", 20),
        ("5 + 2 * 10", 25),
        ("(5 + 10 * 2 + 15 / 3) * 2 + -10", 50),
    ];
    for (i, e) in cases {
        assert_int(i, e);
    }
}

#[test]
fn boolean_logic() {
    let cases = [
        ("true", true),
        ("false", false),
        ("1 < 2", true),
        ("1 > 2", false),
        ("1 == 1", true),
        ("1 != 1", false),
        ("true == true", true),
        ("true != false", true),
        ("(1 < 2) == true", true),
        ("!true", false),
        ("!!true", true),
        ("!5", false),
    ];
    for (i, e) in cases {
        assert_bool(i, e);
    }
}

#[test]
fn if_else() {
    assert_int("if (true) { 10 }", 10);
    assert_int("if (1 < 2) { 10 } else { 20 }", 10);
    assert_int("if (1 > 2) { 10 } else { 20 }", 20);
    assert!(matches!(run("if (false) { 10 }"), Object::Null));
}

#[test]
fn return_statements() {
    assert_int("return 10;", 10);
    assert_int("return 10; 9;", 10);
    assert_int("return 2 * 5; 9;", 10);
    assert_int("9; return 2 * 5; 9;", 10);
    assert_int(
        "if (10 > 1) { if (10 > 1) { return 10; } return 1; }",
        10,
    );
}

#[test]
fn let_and_identifier() {
    assert_int("let a = 5; a;", 5);
    assert_int("let a = 5 * 5; a;", 25);
    assert_int("let a = 5; let b = a; b;", 5);
    assert_int("let a = 5; let b = a; let c = a + b + 5; c;", 15);
}

#[test]
fn functions_and_closures() {
    assert_int("let identity = fn(x) { x; }; identity(5);", 5);
    assert_int("let identity = fn(x) { return x; }; identity(5);", 5);
    assert_int("let double = fn(x) { x * 2; }; double(5);", 10);
    assert_int("let add = fn(x, y) { x + y; }; add(5, 5);", 10);
    assert_int("let add = fn(x, y) { x + y; }; add(5 + 5, add(5, 5));", 20);
    assert_int("fn(x) { x; }(5)", 5);
    assert_int(
        "let newAdder = fn(x) { fn(y) { x + y; }; };
         let addTwo = newAdder(2);
         addTwo(3);",
        5,
    );
}

#[test]
fn strings() {
    match run(r#""hello world""#) {
        Object::String(s) => assert_eq!(s, "hello world"),
        o => panic!("got {:?}", o),
    }
    match run(r#""hello" + " " + "world""#) {
        Object::String(s) => assert_eq!(s, "hello world"),
        o => panic!("got {:?}", o),
    }
}

#[test]
fn arrays() {
    assert_int("[1, 2, 3][0]", 1);
    assert_int("[1, 2, 3][1]", 2);
    assert_int("[1, 2, 3][2]", 3);
    assert_int("let i = 0; [1][i];", 1);
    assert_int("let myArr = [1,2,3]; myArr[2];", 3);
    assert!(matches!(run("[1,2,3][3]"), Object::Null));
    assert!(matches!(run("[1,2,3][-1]"), Object::Null));
}

#[test]
fn builtins() {
    assert_int(r#"len("")"#, 0);
    assert_int(r#"len("four")"#, 4);
    assert_int(r#"len("hello world")"#, 11);
    assert_int("len([1,2,3])", 3);
    assert_int("first([1,2,3])", 1);
    assert_int("last([1,2,3])", 3);
    assert_int("len(rest([1,2,3]))", 2);
    assert_int("len(push([1,2], 3))", 3);
    assert_int("first(push([1,2], 3))", 1);
}

#[test]
fn hash_literals_and_indexing() {
    assert_int(r#"{"one": 1}["one"]"#, 1);
    assert_int(r#"let two = "two"; {"one": 1, two: 2}[two]"#, 2);
    assert_int("{1: 1, 2: 2}[1]", 1);
    assert_int("{true: 5}[true]", 5);
    assert!(matches!(run(r#"{"foo": 5}["bar"]"#), Object::Null));
}

#[test]
fn errors() {
    let cases = [
        ("5 + true;", "type mismatch: INTEGER + BOOLEAN"),
        ("-true", "unknown operator: -BOOLEAN"),
        ("true + false", "unknown operator: BOOLEAN + BOOLEAN"),
        ("foobar", "identifier not found: foobar"),
        (r#"{"name": "Liakos"}[fn(x) { x }];"#, "unusable as hash key: FUNCTION"),
    ];
    for (input, want) in cases {
        match run(input) {
            Object::Error(msg) => assert_eq!(msg, want, "input: {}", input),
            o => panic!("expected Error for {}, got {:?}", input, o),
        }
    }
}
