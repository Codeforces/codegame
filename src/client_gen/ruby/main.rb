require 'socket'
require_relative 'stream_wrapper'
require_relative 'model'
require_relative 'my_strategy'

class SocketWrapper
    def initialize(socket)
        @socket = socket
    end
    def read_bytes(byte_count)
        result = ''
        while result.length < byte_count
            chunk = @socket.recv(byte_count - result.length)
            if chunk.length <= 0
                raise "Can't read from socket"
            end
            result << chunk
        end
        result
    end
    def write_bytes(data)
        @socket.write(data)
    end
    def flush()
        # TODO
    end
end

class Runner
    def initialize(host, port, token)
        socket = TCPSocket.open(host, port)
        socket.setsockopt(Socket::IPPROTO_TCP, Socket::TCP_NODELAY, 1)
        stream = SocketWrapper.new(socket)
        @reader = StreamWrapper.new(stream)
        @writer = StreamWrapper.new(stream)
        @token = token
        @writer.write_string(@token)
        @writer.flush()
    end

    def run()
        strategy = MyStrategy.new()

        while true
            message = ServerMessage.read_from(@reader)
            player_view = message.player_view
            if player_view == nil
                break
            end
            ClientMessage::ActionMessage.new(strategy.get_action(player_view)).write_to(@writer)
            @writer.flush()
        end
    end
end

host = ARGV.length < 1 ? "127.0.0.1" : ARGV[0]
port = ARGV.length < 2 ? 31001 : ARGV[1].to_i
token = ARGV.length < 3 ? "0000000000000000" : ARGV[2]
Runner.new(host, port, token).run()
