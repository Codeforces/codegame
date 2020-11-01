import java.io.OutputStream

class Debug(private val stream: OutputStream) {
  def send(command: model.DebugCommand) {
    model.ClientMessage.DebugMessage(command).writeTo(stream)
    stream.flush()
  }
}