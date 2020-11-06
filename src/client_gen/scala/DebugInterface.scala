import java.io.OutputStream

class DebugInterface(private val stream: OutputStream) {
  def send(command: model.DebugCommand) {
    model.ClientMessage.DebugMessage(command).writeTo(stream)
    stream.flush()
  }
}