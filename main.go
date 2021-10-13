package main

import (
	"flag"
	"log"
	"os"
)

func readFile(path *string) []byte {
	data, err := os.ReadFile(*path)
	if err != nil {
		log.Fatal("Read file error: ", err)
		return nil
	}
	return data
}

func main() {
	isServer := flag.Bool("server", false, "")
	program := flag.String("program", "data/distribute.exec", "Path to program to execute")
	flag.Parse()

	if *isServer {
		// Starts the rpc handler server
		// Only runs if a flag --server true has been passed
		// I was thinking of placing it on a different package
		// but aah thats what you get
		runRPCserver()

	} else {
		runClient(*program)
	}
}
