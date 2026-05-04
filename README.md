# Liakos (Rust)

A Rust port of the [liakos-language](https://github.com/kx0101/liakos-language) tree-walking interpreter — a Monkey-style language with `let`, `fn`, closures, `if`/`else`, `return`, integers, booleans, strings, arrays, and hash maps.

## Features

- REPL (Read-Eval-Print Loop)
- Primitive types: `int`, `bool`, `string`
- Data structures: arrays (`[]`) and hash maps (`{}`)
- Arithmetic / comparison: `+ - * / < > == !=`
- `let` bindings, `if`/`else`, `return`
- First-class & higher-order functions, closures, lexical scoping, recursion
- Built-ins: `len`, `first`, `last`, `rest`, `push`, `print`
- Pratt parser with proper operator precedence

## Build & Run

```bash
cargo build --release

# REPL
cargo run

# Run a .liakos file
cargo run -- examples/main.liakos
```

## REPL example

```
>> let add = fn(a, b) { return a + b; };
>> add(2, 3);
5
>> let fib = fn(n) { if (n < 2) { n } else { fib(n-1) + fib(n-2) } };
>> fib(10);
55
```

## Tests

```bash
cargo test
```

## Layout

| File | Purpose |
|---|---|
| `src/token.rs` | Token type enum and keyword lookup |
| `src/lexer.rs` | Source → token stream |
| `src/ast.rs` | AST as `Statement` / `Expression` enums |
| `src/parser.rs` | Pratt parser |
| `src/object.rs` | Runtime object enum + hash keys |
| `src/environment.rs` | Lexical environments (`Rc<RefCell<...>>`) |
| `src/evaluator.rs` | Tree-walking interpreter |
| `src/builtins.rs` | Built-in functions |
| `src/repl.rs` | Interactive REPL |
| `src/runner.rs` | File runner |
| `src/main.rs` | CLI entry point |
