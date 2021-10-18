package main

import (
	"fmt"
	"io"
	"log"
	"net"
	"strconv"

	pb "github.com/rmgen/rae/rpc/proto"

	"google.golang.org/grpc"
)

type server struct {
	pb.UnimplementedStreamServiceServer
}

func (s *server) StartApplication(in *pb.Request, stream pb.StreamService_StartApplicationServer) error {
	program := Program{
		Name:           in.Name,
		Executable:     in.Executable,
		ExecuteCommand: in.ExecuteCommand,
		Data:           in.Data,
		Path:           in.Path,
		Argv:           in.Argv,
		Envv:           in.Envv,
	}

	program.Execute(stream)

	return nil
}

func (s *server) StreamInput(stream pb.StreamService_StreamInputServer) error {
	for {
		input, err := stream.Recv()
		if err == io.EOF {
			// Invoke terminate program
			fmt.Println("EOF")
		}

		if err != nil {
			return err
		}

		log.Println(input.Input)
	}
}

func runRPCserver(port *int) {

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

	// 	fmt.Printf("Connected to: %s, on Port %d \n", conn.RemoteAddr(), *port)

}
