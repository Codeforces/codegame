import java.io.{BufferedInputStream, BufferedOutputStream}
import java.net.Socket

import model.ClientMessage.ActionMessage
import util.StreamUtil

object Runner extends App {

  val host = if (args.length < 1) "127.0.0.1" else args(0)
  val port = if (args.length < 2) 31001 else args(1).toInt
  val token = if (args.length < 3) "0000000000000000" else args(2)

  run(host, port, token)

  def run(host: String, port: Int, token: String) {
    val socket = new Socket(host, port)
    socket.setTcpNoDelay(true)
    val inputStream = new BufferedInputStream(socket.getInputStream)
    val outputStream = new BufferedOutputStream(socket.getOutputStream)

    StreamUtil.writeString(outputStream, token)
    outputStream.flush()

    val myStrategy = new MyStrategy()
    while (true) {
      val message = model.ServerMessage.readFrom(inputStream)

      message.playerView match {
        case None => return
        case Some(playerView) =>
          ActionMessage(myStrategy.getAction(playerView)).writeTo(outputStream)
          outputStream.flush()
      }
    }
  }
}