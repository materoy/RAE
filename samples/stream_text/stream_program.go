package main

import (
	"fmt"
	"time"
)

/* This program acts as an output stream
* prints a string of characters to the standard output
every [T] seconds
*/
func main() {
	for {
		fmt.Println("Hello world")
		time.Sleep(time.Second * 2)
	}
}
