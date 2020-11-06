#include "DebugInterface.hpp"
#include "model/ClientMessage.hpp"

DebugInterface::DebugInterface(const std::shared_ptr<OutputStream>& outputStream)
    : outputStream(outputStream)
{
}

void DebugInterface::send(const DebugCommand& command)
{
    // TODO: Construct actual message, this is a hack :)
    outputStream->write(ClientMessage::DebugMessage::TAG);
    command.writeTo(*outputStream);
    outputStream->flush();
}