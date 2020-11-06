import java.io.IOException
import java.io.OutputStream

class DebugInterface(private val stream: OutputStream) {
    fun send(command: model.DebugCommand) {
        try {
            model.ClientMessage.DebugMessage(command).writeTo(stream)
            stream.flush()
        } catch (e: IOException) {
            throw RuntimeException(e)
        }
    }
}
