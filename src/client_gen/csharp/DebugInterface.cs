using System.IO;

namespace ProjectName
{
    public class DebugInterface
    {
        private BinaryWriter writer;
        public DebugInterface(BinaryWriter writer)
        {
            this.writer = writer;
        }
        public void Send(Model.DebugCommand command)
        {
            new Model.ClientMessage.DebugMessage(command).WriteTo(writer);
            writer.Flush();
        }
    }
}