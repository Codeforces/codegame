#include "Debug.hpp"
#include "MyStrategy.hpp"
#include "TcpStream.hpp"
#include "model/Model.hpp"
#include <memory>
#include <string>

class Runner {
public:
    Runner(const std::string& host, int port, const std::string& token)
    {
        std::shared_ptr<TcpStream> tcpStream(new TcpStream(host, port));
        inputStream = getInputStream(tcpStream);
        outputStream = getOutputStream(tcpStream);
        outputStream->write(token);
        outputStream->flush();
    }
    void run()
    {
        Debug debug(outputStream);
        MyStrategy myStrategy;
        while (true) {
            auto message = ServerMessage::readFrom(*inputStream);
            const auto& playerView = message.playerView;
            if (!playerView) {
                break;
            }
            ClientMessage::ActionMessage(myStrategy.getAction(*playerView, debug)).writeTo(*outputStream);
            outputStream->flush();
        }
    }

private:
    std::shared_ptr<InputStream> inputStream;
    std::shared_ptr<OutputStream> outputStream;
};

int main(int argc, char* argv[])
{
    std::string host = argc < 2 ? "127.0.0.1" : argv[1];
    int port = argc < 3 ? 31001 : atoi(argv[2]);
    std::string token = argc < 4 ? "0000000000000000" : argv[3];
    Runner(host, port, token).run();
    return 0;
}