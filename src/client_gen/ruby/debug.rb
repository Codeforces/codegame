require_relative 'model'

class Debug
    def initialize(writer)
        @writer = writer
    end

    def send(debugData)
        ClientMessage::DebugDataMessage.new(debugData).write_to(@writer)
        @writer.flush()
    end
end