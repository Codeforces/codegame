namespace ProjectName

open System
open System.IO;
open System.Net.Sockets;

module Runner =
    type T(host, port, token: string) =
        let client = new TcpClient(host, port)
        let stream = new BufferedStream(client.GetStream())
        let reader = new BinaryReader(stream)
        let writer = new BinaryWriter(stream)
        let tokenData = System.Text.Encoding.UTF8.GetBytes token
        do 
            client.NoDelay <- true
            writer.Write tokenData.Length
            writer.Write tokenData
            writer.Flush()

        member this.run =
            let myStrategy = new MyStrategy()

            let rec loop() = 
                let message = Model.ServerMessage.readFrom reader
                
                match message.PlayerView with
                    | Some playerView ->                                                     
                        (Model.ClientMessage.ActionMessage {Action = myStrategy.getAction(playerView)}).writeTo writer
                        writer.Flush()
                        loop()
                    | None -> ()

            loop()

    [<EntryPoint>]
    let main argv =
        let host = match argv with
                    | x when x.Length >=1 -> x.[0]
                    | _-> "127.0.0.1"
        let port = match argv with
                    | x when x.Length >=2 -> x.[1] |> Int32.Parse
                    | _ -> 31001
        let token = match argv with
                    | x when x.Length >=3 -> x.[2]
                    | _ -> "0000000000000000"

        (new T(host, port, token)).run

        0 // return an integer exit code
