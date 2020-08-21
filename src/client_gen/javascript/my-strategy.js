const model = require('./model/model');

class MyStrategy {
    async getAction(playerView) {
        throw "Write your strategy here";
    }
}

module.exports.MyStrategy = MyStrategy;
