import model.*

class MyStrategy {
    fun getAction(playerView: PlayerView, debugInterface: DebugInterface?): Action {
        TODO("Write your strategy here")
    }
    fun debugUpdate(playerView: PlayerView, debugInterface: DebugInterface) {
        debugInterface.send(model.DebugCommand.Clear())
        debugInterface.getState()
    }
}
