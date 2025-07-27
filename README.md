
- **REPL (Read-Eval-Print Loop)**
- **Primitive Types**: `int`, `bool`, `string`
- **Data Structures**: Arrays (`[]`) & Hash maps (`{}`)
- **Arithmetic Expressions**: `+`, `-`, `*`, `/`, `<`, `>`, `==`, `!=`
- **Variables & Bindings**: `let` statements
- **Control Flow**: `if-else`, `return`
- **Functions**:
  - First-class & higher-order
  - Closures & lexical scoping
  - Recursion supported
- **Built-in Functions**: e.g. `len()`, `first()`, `last()`, `push()`, `print()`
- **Parser with Pratt Parsing** for correct operator precedence
- **Error Handling** with informative messages
- **Lexical Analyzer** (Tokenizer) and recursive descent parser

## 🚀 Getting Started

### 1. Clone the repo

```bash
git clone https://github.com/kx0101/liakos-language.git
cd liakos-language
```

## REPL
```bash
go run main.go

>> let add = fn(a, b) { return a + b; };
>> add(2, 3);
5
```

## Run a File
You can also execute `.liakos` source files:

```bash
./liakos examples/main.liakos
```
