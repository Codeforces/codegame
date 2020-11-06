import model.*;

public class MyStrategy {
    public Action getAction(PlayerView playerView, DebugInterface debugInterface) {
        throw new UnsupportedOperationException("Not implemented");
    }
    public void debugUpdate(PlayerView playerView, DebugInterface debugInterface) {
        debugInterface.send(new DebugCommand.Clear());
        debugInterface.getState();
    }
}