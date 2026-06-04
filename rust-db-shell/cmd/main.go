package main

import (
	"bufio"
	"fmt"
	"os"
	"strings"
)

func main() {
	scanner := bufio.NewScanner(os.Stdin)

	for {
		fmt.Print("shell> ")

		if !scanner.Scan() {
			break
		}

		input := scanner.Text()
		input = strings.ToLower(input)

		if input == "exit" {
			break
		}

		fmt.Println("You typed:", input)
	}
}
