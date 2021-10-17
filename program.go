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

	writeErr := os.WriteFile(p.Name, p.Executable, 0755)
	if writeErr != nil {
		log.Fatal("Write error: ", writeErr)
	}

	/** Creates a new child process for the program execution
	* Execution replaces current process with the system call process
	* thus quitting current
	* hence a fork is necessary !
	 */
	// id, _, _ := syscall.Syscall(syscall.SYS_FORK, 0, 0, 0)

	// if id == 0 {
	fmt.Println("Executing program ...")
	// r, w := io.Pipe()
	// Calls execve to execute program with given params

	cmd := exec.Command("./" + p.Name)
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
		/// Assigned whitespace for entry to loop
		var output string = " "

		// Exists whenever there is no output from the stdout
		// We'll probably need another exit criteria
		for output != "" {
			line, _, _ := buf.ReadLine()
			output = string(line)
			fmt.Println(output)
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
	// } else {
	// 	// Any computation for the parent process should be handled here
	// 	return nil
	// }

}
