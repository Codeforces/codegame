import model;
import debugger;
import std.typecons;
import std.conv;

class MyStrategy {
    Action getAction(PlayerView playerView, Debugger debugger) {
        throw new Error("Write your strategy here");
    }
}