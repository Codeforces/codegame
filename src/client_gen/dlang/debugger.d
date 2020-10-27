import model;
import stream;

class Debugger {
    this(Stream stream) {
        this.stream = stream;
    }
    void send(const DebugData debugData) {
        stream.write(ClientMessage.DebugDataMessage.TAG);
        debugData.writeTo(stream);
    }
private:
    Stream stream;
}