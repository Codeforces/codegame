require_relative 'model'

class DebugInterface
    def initialize(writer)
        @writer = writer
    end

    def send(command)
        ClientMessage::DebugMessage.new(command).write_to(@writer)
        @writer.flush()
    end
end