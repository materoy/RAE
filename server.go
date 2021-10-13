package main

import (
	"fmt"
	"log"
	"net"
	"net/http"
	"net/rpc"
	"strconv"
)

func runRPCserver(port *int) error {

	prog := new(Program)
	rpc.Register(prog)
	rpc.HandleHTTP()

	l, e := net.Listen("tcp", ":"+strconv.Itoa(*port))

	if e != nil {
		log.Fatal("Listen error:", e)
		return e
	}

	fmt.Printf("RPC Server listening on: %v\n", l.Addr().String())

	go http.Serve(l, nil)

	select {}
}
