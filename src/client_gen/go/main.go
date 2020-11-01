package main

import (
	"bufio"
	"net"
	"os"
	. "project_name/model"
	. "project_name/stream"
	"strconv"
)

type Runner struct {
	conn   net.Conn
	reader *bufio.Reader
	writer *bufio.Writer
}

func NewRunner(host string, port uint16, token string) Runner {
	conn, err := net.Dial("tcp", host+":"+strconv.Itoa(int(port)))
	if err != nil {
		panic(err)
	}
	writer := bufio.NewWriter(conn)
	WriteString(writer, token)
	err = writer.Flush()
	if err != nil {
		panic(err)
	}
	return Runner{
		conn:   conn,
		reader: bufio.NewReader(conn),
		writer: writer,
	}
}

func (runner Runner) Run() {
	myStrategy := NewMyStrategy()
	debug := Debug{
		Writer: runner.writer,
	}
loop:
	for {
		switch message := ReadServerMessage(runner.reader).(type) {
		case ServerMessageGetAction:
			ClientMessageActionMessage{
				Action: myStrategy.getAction(message.PlayerView, debug),
			}.Write(runner.writer)
			err := runner.writer.Flush()
			if err != nil {
				panic(err)
			}
		case ServerMessageFinish:
			break loop
		case ServerMessageDebugUpdate:
			myStrategy.debugUpdate(debug)
		default:
			panic("Unexpected server message")
		}
	}
}

func main() {
	var host string
	if len(os.Args) < 2 {
		host = "localhost"
	} else {
		host = os.Args[1]
	}
	var port uint16
	if len(os.Args) < 3 {
		port = 31001
	} else {
		portInt, err := strconv.Atoi(os.Args[2])
		port = uint16(portInt)
		if err != nil {
			panic(err)
		}
	}
	var token string
	if len(os.Args) < 4 {
		token = "0000000000000000"
	} else {
		token = os.Args[3]
	}

	runner := NewRunner(host, port, token)
	runner.Run()
}
