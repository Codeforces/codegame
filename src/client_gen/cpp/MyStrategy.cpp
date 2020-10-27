#include "MyStrategy.hpp"
#include <exception>

MyStrategy::MyStrategy() {}

Action MyStrategy::getAction(const PlayerView& playerView, Debug& debug)
{
    throw std::exception("Write your strategy here");
}