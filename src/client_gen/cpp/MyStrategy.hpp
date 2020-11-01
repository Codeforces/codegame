#ifndef _MY_STRATEGY_HPP_
#define _MY_STRATEGY_HPP_

#include "Debug.hpp"
#include "model/Model.hpp"

class MyStrategy {
public:
    MyStrategy();
    Action getAction(const PlayerView& playerView, Debug& debug);
    void debugUpdate(const PlayerView& playerView, Debug& debug);
};

#endif