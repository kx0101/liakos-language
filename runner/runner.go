package runner

import (
	"fmt"
	"os"

	"github.com/kx0101/liakos-language/evaluator"
	"github.com/kx0101/liakos-language/lexer"
	"github.com/kx0101/liakos-language/object"
	"github.com/kx0101/liakos-language/parser"
)

func RunFile(filePath string) {
	content, err := os.ReadFile(filePath)
	if err != nil {
		fmt.Printf("Could not read file: %s\n", err)
		os.Exit(1)
	}

	l := lexer.New(string(content))
	p := parser.New(l)

	program := p.ParseProgram()
	if len(p.Errors()) != 0 {
		printParserErrors(p.Errors())
		os.Exit(1)
	}

	env := object.NewEnvironment()

	for _, stmt := range program.Statements {
		evaluated := evaluator.Eval(stmt, env)
		if evaluated != nil && evaluated.Type() != object.NULL_OBJ {
			fmt.Println(evaluated.Inspect())
		}
	}
}

func printParserErrors(errors []string) {
	fmt.Println("Parser errors:")

	for _, msg := range errors {
		fmt.Printf("\t%s\n", msg)
	}
}
