namespace ProjectName

type Debug(writer) =
    member this.send(command) =
        let message: Model.ClientMessageDebugMessage = { Command = command }
        message.writeTo writer
        writer.Flush()
