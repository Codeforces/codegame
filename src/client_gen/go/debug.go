package main

import . "project_name/model"
import "bufio"

type Debug struct {
	Writer *bufio.Writer
}

func (debug Debug) Send(command DebugCommand) {
	ClientMessageDebugMessage {
		Command: command,
	}.Write(debug.Writer)
	err := debug.Writer.Flush()
	if err != nil {
		panic(err)
	}
}