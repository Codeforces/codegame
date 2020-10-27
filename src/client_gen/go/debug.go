package main

import . "project_name/model"
import "bufio"

type Debug struct {
	Writer *bufio.Writer
}

func (debug Debug) Send(debugData DebugData) {
	ClientMessageDebugDataMessage {
		Data: debugData,
	}.Write(debug.Writer)
	err := debug.Writer.Flush()
	if err != nil {
		panic(err)
	}
}