import java.io.IOException;
import java.io.OutputStream;

public class DebugInterface {
    private OutputStream stream;

    public DebugInterface(OutputStream stream) {
        this.stream = stream;
    }

    public void send(model.DebugCommand command) {
        try {
            new model.ClientMessage.DebugMessage(command).writeTo(stream);
            stream.flush();
        } catch (IOException e) {
            throw new RuntimeException(e);
        }
    }
}