const model = require('./model/index');

class MyStrategy {
    async getAction(playerView, debugInterface) {
        throw "Write your strategy here";
    }
    async debugUpdate(playerView, debugInterface) {
        await debugInterface.send(new model.DebugCommand.Clear());
        await debugInterface.getState();
    }
}

module.exports.MyStrategy = MyStrategy;
