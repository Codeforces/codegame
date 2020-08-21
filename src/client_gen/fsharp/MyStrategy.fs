namespace ProjectName

open ProjectName.Model

type MyStrategy() =
    member this.getAction(playerView: PlayerView): Action =
        raise (System.NotImplementedException "Write your strategy here")