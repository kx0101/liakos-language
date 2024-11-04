package main

import (
	"fmt"
	"os"
	"os/user"

	"github.com/kx0101/monkey-language/repl"
)

func main() {
    user, err := user.Current()
    if err != nil {
        panic(err)
    }

    fmt.Printf("Hello %s! This is the monkey programming language!\n", user.Username)
    fmt.Printf("feel free to type in commands\n")

    repl.Start(os.Stdin, os.Stdout)
}
