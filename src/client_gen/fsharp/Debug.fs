namespace ProjectName

type Debug(writer) =
    member this.send(debugData) =
        let message : Model.ClientMessageDebugDataMessage = {Data = debugData}
        message.writeTo writer
        writer.Flush()