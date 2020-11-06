import model;
import debugger;
import std.typecons;
import std.conv;

class MyStrategy
{
    Action getAction(PlayerView playerView, DebugInterface debugInterface)
    {
        throw new Error("Write your strategy here");
    }

    void debugUpdate(PlayerView playerView, DebugInterface debugInterface)
    {
    }
}
