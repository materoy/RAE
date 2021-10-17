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

func (s *server) StartApplication(in *pb.Request, stream pb.StreamService_StartApplicationServer) error {
	for i := 0; i < 4; i++ {
		response := strconv.Itoa(i)
		if err := stream.Send(&pb.Response{Result: response}); err != nil {
			return err
		}
	}

	return nil
}

func runRPCserver(port *int) {

	prog := new(Program)
	rpc.Register(prog)
	rpc.HandleHTTP()

	l, e := net.Listen("tcp", getOutBoundIP().String()+":"+strconv.Itoa(*port))

	if e != nil {
		log.Fatal("Listen error:", e)
	}

	grpcServer := grpc.NewServer()
	pb.RegisterStreamServiceServer(grpcServer, &server{})

	fmt.Printf("RPC Server listening on: %v\n", l.Addr().String())

	if err := grpcServer.Serve(l); err != nil {
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
