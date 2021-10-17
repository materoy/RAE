package main

import (
	"fmt"
	"log"
	"net"
	"net/rpc"
	"strconv"

	pb "github.com/rmgen/rae/rpc/proto"

	"google.golang.org/grpc"
)

type server struct {
	pb.UnimplementedStreamServiceServer
}

func runRPCserver(port *int) {

	prog := new(Program)
	rpc.Register(prog)
	rpc.HandleHTTP()

	l, e := net.Listen("tcp", getOutBoundIP().String()+":"+strconv.Itoa(*port))

	if e != nil {
		log.Fatal("Listen error:", e)
	}

	s := grpc.NewServer()
	pb.RegisterStreamServiceServer(s, &server{})

	fmt.Printf("RPC Server listening on: %v\n", l.Addr().String())

	if err := s.Serve(l); err != nil {
		log.Fatal("Failed to serve: ", err)
	}

	// go http.Serve(l, nil)

	// select {}
	// for {
	// 	conn, err := l.Accept()

	// 	if err != nil {
	// 		fmt.Println("Server, Accept: ", err)
	// 		continue
	// 	}

	// 	fmt.Printf("Connected to: %s, on Port %d \n", conn.RemoteAddr(), *port)

	// }

}
