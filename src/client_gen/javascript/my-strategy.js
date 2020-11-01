const model = require('./model/index');
const Debug = require('./debug').Debug;

class MyStrategy {
    async getAction(playerView, debug) {
        throw "Write your strategy here";
    }
    async debugUpdate(debug) { }
}

module.exports.MyStrategy = MyStrategy;
