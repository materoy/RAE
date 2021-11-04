package main

import (
	"fmt"
	"os"
)

func main() {
	fmt.Println("Hello and welcome to scanline game")
	for {
		// buf := bufio.NewReader(os.Stdin)
		fmt.Println("> ")

		// line, err := buf.ReadBytes('\n')
		// if err != nil {
		// 	fmt.Println(err)
		// } else {
		// 	fmt.Println(string(line))
		// }
		var input string
		fmt.Scan(&input)

		fmt.Fprint(os.Stdout, input)
	}
}
