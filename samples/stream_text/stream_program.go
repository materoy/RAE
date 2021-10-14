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
	for i := 0; i < 5; i++ {
		fmt.Println("Hello ", i)
		time.Sleep(time.Second * 1)
	}
}
