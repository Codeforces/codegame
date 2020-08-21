#ifndef _MY_STRATEGY_HPP_
#define _MY_STRATEGY_HPP_

#include "model/Model.hpp"

class MyStrategy {
public:
    MyStrategy();
    Action getAction(const PlayerView& playerView);
};

#endif