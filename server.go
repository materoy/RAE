package main

import (
	"fmt"
	"net"
	"net/http"
	"net/rpc"
)

func runRPCserver() error {

	prog := new(Program)
	rpc.Register(prog)
	rpc.HandleHTTP()

	l, e := net.Listen("tcp", ":1234")

	if e != nil {
		// log.Fatal("Listen error:", e)
		if e.Error() != "listen tcp :1234: bind: address already in use" {
			fmt.Println("Listen errror: ", e)
		}
		return e
	}

	fmt.Printf("RPC Server listening on: %v\n", l.Addr().String())

	go http.Serve(l, nil)

	select {}
}
