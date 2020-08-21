#include "MyStrategy.hpp"
#include <exception>

MyStrategy::MyStrategy() {}

Action MyStrategy::getAction(const PlayerView& playerView)
{
    throw std::exception("Write your strategy here");
}