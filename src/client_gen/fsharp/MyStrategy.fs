namespace ProjectName

open ProjectName.Model

type MyStrategy() =
    member this.getAction(playerView: PlayerView, debugInterface: DebugInterface): Action =
        raise (System.NotImplementedException "Write your strategy here")

    member this.debugUpdate(playerView: PlayerView, debugInterface: DebugInterface) = ()
