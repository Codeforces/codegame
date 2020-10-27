import java.io.OutputStream

class Debug(private val stream: OutputStream) {
  def send(debugData: model.DebugData) {
    model.ClientMessage.DebugDataMessage(debugData).writeTo(stream)
    stream.flush()
  }
}