package main

import (
	. "project_name/model"
)

type MyStrategy struct{}

func NewMyStrategy() MyStrategy {
	return MyStrategy{}
}

func (strategy MyStrategy) getAction(playerView PlayerView) Action {
	panic("Write your strategy here")
}
