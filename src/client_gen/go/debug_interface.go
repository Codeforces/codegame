package main

import (
	"bufio"
	. "project_name/model"
)

type DebugInterface struct {
	Writer *bufio.Writer
}

func (debug DebugInterface) Send(command DebugCommand) {
	ClientMessageDebugMessage{
		Command: command,
	}.Write(debug.Writer)
	err := debug.Writer.Flush()
	if err != nil {
		panic(err)
	}
}
