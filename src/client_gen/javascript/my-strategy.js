const model = require('./model/index');
const DebugInterface = require('./debug-interface').DebugInterface;

class MyStrategy {
    async getAction(playerView, debugInterface) {
        throw "Write your strategy here";
    }
    async debugUpdate(playerView, debugInterface) { }
}

module.exports.MyStrategy = MyStrategy;
