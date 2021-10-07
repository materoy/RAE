package main

import (
	"fmt"
	"log"
	"net/rpc"
)

func runClient(path string) {

	client, err := rpc.DialHTTP("tcp", "localhost:1234")
	if err != nil {
		log.Fatal("dialing:", err)
	}

	// path := "/home/redview/go/src/github.com/rmgen/distributed_computation/data/distribute.exec"

	program := Program{
		Name:           "Helloworld",
		Executable:     readFile(&path),
		ExecuteCommand: "",
		Data:           "",
		Path:           path,
		Argv:           nil,
	}

	var reply string
	doneChan := make(chan *rpc.Call, 10)

	execCall := client.Go("Program.Execute", program, &reply, doneChan)

	if execCall.Error != nil {
		log.Fatal("RPC error: ", execCall.Error)
	}

	data, done := <-doneChan
	if done {
		fmt.Println("Execution done ...")
		fmt.Println(*(data.Reply.(*string)))
	} else {
		fmt.Println("No value was read from channel")
	}

}
