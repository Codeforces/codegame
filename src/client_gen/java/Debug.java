import java.io.IOException;
import java.io.OutputStream;

public class Debug {
    private OutputStream stream;

    public Debug(OutputStream stream) {
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