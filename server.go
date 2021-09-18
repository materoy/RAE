package main

import (
	"fmt"
	"log"
	"net"
	"net/http"
	"net/rpc"
)

func runRPCserver() {

	prog := new(Program)
	rpc.Register(prog)
	rpc.HandleHTTP()

	l, e := net.Listen("tcp", "localhost:1234")

	if e != nil {
		log.Fatal("Listen error:", e)
	}

	fmt.Printf("RPC Server listening on: %v\n", l.Addr().String())

	go http.Serve(l, nil)

	select {}
}
