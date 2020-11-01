import model;
import stream;

class Debugger
{
    this(Stream stream)
    {
        this.stream = stream;
    }

    void send(const DebugCommand command)
    {
        // TODO: Construct actual message, this is a hack :)
        stream.write(ClientMessage.DebugMessage.TAG);
        command.writeTo(stream);
    }

private:
    Stream stream;
}
