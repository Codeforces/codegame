#ifndef _DEBUG_HPP_
#define _DEBUG_HPP_

#include "Stream.hpp"
#include "model/DebugData.hpp"
#include <memory>

class Debug {
public:
    Debug(const std::shared_ptr<OutputStream>& outputStream);
    void send(const DebugData& debugData);

private:
    std::shared_ptr<OutputStream> outputStream;
};

#endif