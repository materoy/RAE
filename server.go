package main

import (
	"fmt"
	"log"
	"net"
	"net/http"
	"net/rpc"
	"strconv"
)

func getOutBoundIP() net.IP {
	conn, err := net.Dial("tcp", "8.8.8.8:8")

	if err != nil {
		log.Fatal("Cannot get IP", err)
	}

	defer conn.Close()

	localAddr := conn.LocalAddr().(*net.TCPAddr)

	return localAddr.IP
}

func runRPCserver(port *int) error {
	prog := new(Program)
	rpc.Register(prog)
	rpc.HandleHTTP()

	l, e := net.Listen("tcp", string(getOutBoundIP())+":"+strconv.Itoa(*port))

	if e != nil {
		log.Fatal("Listen error:", e)
		return e
	}

	fmt.Printf("RPC Server listening on: %v\n", l.Addr().String())

	go http.Serve(l, nil)

	select {}
}
