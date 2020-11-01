#include "Debug.hpp"
#include "model/ClientMessage.hpp"

Debug::Debug(const std::shared_ptr<OutputStream>& outputStream)
    : outputStream(outputStream)
{
}

void Debug::send(const DebugCommand& command)
{
    // TODO: Construct actual message, this is a hack :)
    outputStream->write(ClientMessage::DebugMessage::TAG);
    command.writeTo(*outputStream);
    outputStream->flush();
}