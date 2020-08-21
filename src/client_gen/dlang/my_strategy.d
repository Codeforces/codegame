import model;
import std.typecons;
import std.conv;

class MyStrategy {
    Action getAction(PlayerView playerView) {
        throw new Error("Write your strategy here");
    }
}