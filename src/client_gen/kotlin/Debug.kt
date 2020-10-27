import java.io.IOException
import java.io.OutputStream

class Debug(private val stream: OutputStream) {

    fun send(debugData: model.DebugData) {
        try {
            model.ClientMessage.DebugDataMessage(debugData).writeTo(stream)
            stream.flush()
        } catch (e: IOException) {
            throw RuntimeException(e)
        }
    }
}
