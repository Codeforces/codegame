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
        public void Send(Model.DebugData debugData)
        {
            new Model.ClientMessage.DebugDataMessage(debugData).WriteTo(writer);
            writer.Flush();
        }
    }
}