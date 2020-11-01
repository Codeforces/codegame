package main

import (
	. "project_name/model"
)

type MyStrategy struct{}

func NewMyStrategy() MyStrategy {
	return MyStrategy{}
}

func (strategy MyStrategy) getAction(playerView PlayerView, debug Debug) Action {
	panic("Write your strategy here")
}

func (strategy MyStrategy) debugUpdate(playerView PlayerView, debug Debug) {}
