package main

import (
	"fmt"
	"log"
	"os"
	"os/exec"
	"syscall"
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

func (p *Program) Execute(data *Program, reply *string) error {

	// r, w := io.Pipe()

	writeErr := os.WriteFile(data.Name, data.Executable, 0755)
	if writeErr != nil {
		log.Fatal("Write error: ", writeErr)
	}

	/** Creates a new child process for the program execution
	* Execution replaces current process with the system call process
	* thus quitting current
	* hence a fork is necessary !
	 */
	id, _, _ := syscall.Syscall(syscall.SYS_FORK, 0, 0, 0)

	if id == 0 {
		fmt.Println("Executing program ...")
		// Calls execve to execute program with given params
		// err := syscall.Exec(data.Name, data.Argv, data.Envv)

		out, err := exec.Command("./" + data.Name).Output()

		if err != nil {
			fmt.Printf("Program execution error: %s", err)
			return err
		}

		output := string(out)
		*reply = output
		fmt.Println(*reply)

		os.Remove(data.Name)
		return nil
	} else {
		// Any computation for the parent process should be handled here
		return nil
	}

}
