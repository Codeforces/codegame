const model = require('./model/index');

class Debug {
    constructor(streamWrapper) {
        this.streamWrapper = streamWrapper;
    }

    async send(command) {
        await (new model.ClientMessage.DebugMessage(command)).writeTo(this.streamWrapper);
        // TODO: only flush stream once here?
    }
}

module.exports.Debug = Debug;