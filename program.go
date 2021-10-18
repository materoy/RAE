package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"os/exec"

	pb "github.com/rmgen/rae/rpc/proto"
)

/** Program definition struct where
 * [Executable] is the program binary data as a list of bytes
 * [Data] is the input of the program
 * [Path] is the program location in filesystem
 * [Argv] - commandline arguments supplied to the program in the RAC
 * [Envv] - enviroment variables required by the program
 **/

type Program struct {
	Name           string
	Executable     []byte
	ExecuteCommand string
	Data           string
	Path           string
	Argv           []string
	Envv           []string
}

func (p *Program) Execute(stream pb.StreamService_StartApplicationServer) error {

	/* Write the program data to a new file identified by
	* the program's name
	* in execute read and write mode
	 */
	// It's probably stupid to do so since io is slow
	writeErr := os.WriteFile(p.Name, p.Executable, 0755)

	if writeErr != nil {
		log.Fatal("Write error: ", writeErr)
	}

	fmt.Println("Executing program ...")
	// Calls execve to execute program with given params

	cmd := exec.Command(p.ExecuteCommand + p.Name)
	/* Catch all the stdout since current assumption
	* is that all program will output through stdout
	 */

	/* Future development will allow for
	* interfacing different program output channels
	* for multimedia including and especially graphics for X11, Direct12 and Metal
	* I dont know what I'm talking about I should probably shut up
	 */
	stdout, err := cmd.StdoutPipe()

	if err != nil {
		fmt.Printf("Pipe error: %s", err)
		return err
	}

	cmd.Start()
	buf := bufio.NewReader(stdout)

	/* Checking process exit state is blocking
	thus cannot be used wherever stdout is recorded
	Another solution to check when process exits, needs to be deviced
	*/

	exitChan := make(chan *os.ProcessState)

	go func() {
		processState, _ := cmd.Process.Wait()

		exitChan <- processState
	}()

	if err != nil {
		log.Fatal("Process error: ", err)
	}

	go func() {
		exitCode := <-exitChan
		if exitCode != nil {
			fmt.Println(<-exitChan)
		}
	}()

	go func() {
		for {
			line, _, eof := buf.ReadLine()

			if eof != nil {
				break
			}
			output := string(line)
			log.Println(output)

			stream.Send(&pb.Response{Result: output})
		}
	}()

	var code *os.ProcessState
	var ok bool = false
	for !ok {
		code, ok = <-exitChan
		if ok {
			fmt.Println(code)
		}

	}

	defer os.Remove(p.Name)
	return nil
}
