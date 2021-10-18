package main

import "fmt"

func main() {
	fmt.Println("Hello and welcome to scanline game")
	for {
		var input string
		fmt.Scanln(&input)
		fmt.Println("Your input ", input)
	}
}
