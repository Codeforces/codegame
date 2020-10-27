namespace ProjectName

open ProjectName.Model

type MyStrategy() =
    member this.getAction(playerView: PlayerView, debug: Debug): Action =
        raise (System.NotImplementedException "Write your strategy here")