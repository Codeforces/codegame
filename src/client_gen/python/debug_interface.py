import model


class DebugInterface:
    def __init__(self, writer):
        self.writer = writer

    def send(self, command):
        model.ClientMessage.DebugMessage(command).write_to(self.writer)
        self.writer.flush()
