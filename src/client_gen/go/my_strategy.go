package main

import (
	. "project_name/model"
)

type MyStrategy struct{}

func NewMyStrategy() MyStrategy {
	return MyStrategy{}
}

func (strategy MyStrategy) getAction(playerView PlayerView, debugInterface *DebugInterface) Action {
	panic("Write your strategy here")
}

func (strategy MyStrategy) debugUpdate(playerView PlayerView, debugInterface DebugInterface) {
	debugInterface.Send(DebugCommandClear{})
	debugInterface.GetState()
}
