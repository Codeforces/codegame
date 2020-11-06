#ifndef _DEBUG_INTERFACE_HPP_
#define _DEBUG_INTERFACE_HPP_

#include "Stream.hpp"
#include "model/DebugCommand.hpp"
#include <memory>

class DebugInterface {
public:
    DebugInterface(const std::shared_ptr<OutputStream>& outputStream);
    void send(const DebugCommand& command);

private:
    std::shared_ptr<OutputStream> outputStream;
};

#endif