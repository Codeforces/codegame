using System.IO;

namespace ProjectName
{
    public class Debug
    {
        private BinaryWriter writer;
        public Debug(BinaryWriter writer)
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