const model = require('./model/index');
const Debug = require('./debug').Debug;

class MyStrategy {
    async getAction(playerView, debug) {
        throw "Write your strategy here";
    }
}

module.exports.MyStrategy = MyStrategy;
