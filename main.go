package main

import (
	"fmt"
	"os"

	"github.com/kx0101/liakos-language/repl"
	"github.com/kx0101/liakos-language/runner"
)

func main() {
	if len(os.Args) != 2 {
		fmt.Println("Usage: liakos <filename>")
		fmt.Println("or run without arguments to start the REPL.")
		os.Exit(1)
	}

	if len(os.Args) == 2 {
		filePath := os.Args[1]
		file, err := os.Open(filePath)
		if err != nil {
			fmt.Printf("Could not open file %s: %s\n", filePath, err)
			os.Exit(1)
		}
		defer file.Close()

		runner.RunFile(filePath)
		os.Exit(0)
	}

	fmt.Println("Welcome to the Liakos REPL!")
	repl.Start(os.Stdin, os.Stdout)
}
