#include "Debug.hpp"
#include "model/ClientMessage.hpp"

Debug::Debug(const std::shared_ptr<OutputStream>& outputStream)
    : outputStream(outputStream)
{
}

void Debug::send(const DebugData& debugData)
{
    outputStream->write(ClientMessage::DebugDataMessage::TAG);
    debugData.writeTo(*outputStream);
    outputStream->flush();
}