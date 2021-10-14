package main

import (
	"fmt"
	"log"
	"net/rpc"
	"strconv"
	"strings"
)

func runClient(host *string, port *int, path *string) {
	client, err := rpc.DialHTTP("tcp", *host+":"+strconv.Itoa(*port))
	if err != nil {
		log.Fatal("dialing:", err)
	}

	// path := "/home/redview/go/src/github.com/rmgen/distributed_computation/data/distribute.exec"

	programPath := strings.Split(*path, "/")

	program := Program{
		Name:           programPath[len(programPath)-1],
		Executable:     readFile(path),
		ExecuteCommand: "",
		Data:           "",
		Path:           *path,
		Argv:           nil,
	}

	reply := make(chan string)
	doneChan := make(chan *rpc.Call, 10)

	execCall := client.Go("Program.Execute", program, &reply, doneChan)

	if execCall.Error != nil {
		log.Fatal("RPC error: ", execCall.Error)
	}

	data, done := <-doneChan
	if done {
		fmt.Println("Execution done ...")
		fmt.Println(<-(*data.Reply.(*chan string)))
	} else {
		fmt.Println("No value was read from channel")
	}

}
