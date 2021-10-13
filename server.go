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
	conn, err := net.Dial("udp", "8.8.8.8:8")

	if err != nil {
		log.Fatal("Cannot get IP", err)
	}

	defer conn.Close()

	localAddr := conn.LocalAddr().(*net.UDPAddr)

	return localAddr.IP
}

func runRPCserver(port *int) error {

	prog := new(Program)
	rpc.Register(prog)
	rpc.HandleHTTP()

	l, e := net.Listen("tcp", getOutBoundIP().String()+":"+strconv.Itoa(*port))

	if e != nil {
		log.Fatal("Listen error:", e)
		return e
	}

	fmt.Printf("RPC Server listening on: %v\n", l.Addr().String())

	go http.Serve(l, nil)

	for {
		conn, err := l.Accept()

		if err != nil {
			fmt.Println("Server, Accept: ", err)
			continue
		}

		fmt.Printf("Connected to: %s, on Port %d \n", conn.RemoteAddr(), *port)

	}

}
