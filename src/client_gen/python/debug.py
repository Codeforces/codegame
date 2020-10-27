import model


class Debug:
    def __init__(self, writer):
        self.writer = writer

    def send(self, debugData):
        model.ClientMessage.DebugDataMessage(debugData).write_to(self.writer)
        self.writer.flush()
