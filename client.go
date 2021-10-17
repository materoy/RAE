package main

import (
	"context"
	"io"
	"log"
	"strconv"
	"strings"
	"time"

	pb "github.com/rmgen/rae/rpc/proto"
	"google.golang.org/grpc"
)

func runClient(host *string, port *int, path *string) {
	// client, err := rpc.DialHTTP("tcp", *host+":"+strconv.Itoa(*port))
	conn, err := grpc.Dial(*host+":"+strconv.Itoa(*port), grpc.WithInsecure(), grpc.WithBlock())
	if err != nil {
		log.Fatal("dialing error: ", err)
	}

	defer conn.Close()
	client := pb.NewStreamServiceClient(conn)

	ctx, cancel := context.WithTimeout(context.Background(), time.Second*2)
	defer cancel()

	programPath := strings.Split(*path, "/")

	program := Program{
		Name:           programPath[len(programPath)-1],
		Executable:     readFile(path),
		ExecuteCommand: "./",
		Data:           "",
		Path:           *path,
		Argv:           nil,
	}

	stream, err := client.StartApplication(ctx, &pb.Request{
		Name:           program.Name,
		Executable:     program.Executable,
		ExecuteCommand: program.ExecuteCommand,
		Data:           program.Data,
		Path:           program.Path,
		Argv:           program.Argv,
		Envv:           program.Envv,
	})

	if err != nil {
		log.Fatal("gRPC error: ", err)
	}

	for {
		response, err := stream.Recv()
		if err == io.EOF {
			break
		}
		if err != nil {
			log.Fatal("stream error , ", err)
		}

		log.Println(response.GetResult())
	}

	// reply := make(chan string)
	// doneChan := make(chan *rpc.Call, 10)

	// execCall := client.Go("Program.Execute", program, &reply, doneChan)

	// if execCall.Error != nil {
	// 	log.Fatal("RPC error: ", execCall.Error)
	// }

	// data, done := <-doneChan
	// if done {
	// 	fmt.Println("Execution done ...")
	// 	fmt.Println(<-(*data.Reply.(*chan string)))
	// } else {
	// 	fmt.Println("No value was read from channel")
	// }

}
