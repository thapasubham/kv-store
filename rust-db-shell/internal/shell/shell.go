package shell

import (
	"bufio"
	"fmt"
	"os"
	"rust-db-shell/internal/client"
	"strings"
)

type Status struct {
	isConnected bool
}

const addr = "127.0.0.1:5000"

func Run() {
	scanner := bufio.NewScanner(os.Stdin)
	status := Status{isConnected: false}

	db := client.NewClient(addr)

	fmt.Printf("Connecting to database at %s...\n", addr)
	if err := db.Connect(); err != nil {
		fmt.Printf("Warning: Could not connect to database (%v). Operating in offline mode.\n", err)
	} else {
		status.isConnected = true
		fmt.Println("Connected successfully!")
	}

	for {
		fmt.Print("shell> ")

		if !scanner.Scan() {
			break
		}

		input := scanner.Text()

		if isExit(input) {
			_, _ = db.Command("EXIT")
			break
		}

		if strings.TrimSpace(input) == "" {
			continue
		}

		if status.isConnected {
			resp, err := db.Command(input)
			if err != nil {
				fmt.Println("Network error:", err)
				continue
			}
			fmt.Print(resp)
		} else {
			fmt.Println("Error: Not connected to the server. Try restarting the shell.")
		}
	}
}

func isExit(input string) bool {
	clean := strings.ToLower(strings.TrimSpace(input))
	return clean == "exit" || clean == "quit" || clean == "q"
}
