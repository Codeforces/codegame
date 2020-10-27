const model = require('./model/index');

class Debug {
    constructor(streamWrapper) {
        this.streamWrapper = streamWrapper;
    }

    async send(debugData) {
        await (new model.ClientMessage.DebugDataMessage(debugData)).writeTo(this.streamWrapper);
    }
}

module.exports.Debug = Debug;