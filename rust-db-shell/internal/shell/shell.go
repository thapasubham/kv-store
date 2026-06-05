package shell

import (
	"bufio"
	"fmt"
	"os"
	"strings"
)

func Run() {
	scanner := bufio.NewScanner(os.Stdin)

	for {
		fmt.Print("shell> ")

		if !scanner.Scan() {
			break
		}

		input := scanner.Text()
		input = strings.ToLower(input)

		if isExit(input) {
			break
		}

		fmt.Println("You typed:", input)
	}
}

func isExit(input string) bool {
	input = strings.ToLower(strings.TrimSpace(input))

	return input == "exit" || input == "quit" || input == "q"
}
