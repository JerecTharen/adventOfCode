namespace ReceiverLib;

public class Parser{
    private readonly String _input;
    public Parser(String input)
    {
        _input = input;
    }

    public int GetBufferStartCount(int bufferCount = 4){
        bool isBufferStartFound = false;
        int bufferStart = 0;
        while(!isBufferStartFound){
            String nextCharacters = _input.Substring(bufferStart, bufferCount);
            IEnumerable<char> duplicateCharacters = nextCharacters.GroupBy(c => c).Where(cg => cg.Count() > 1).Select(cg => cg.First());
            isBufferStartFound = duplicateCharacters.Count() == 0 || bufferStart == _input.Count();
            if(!isBufferStartFound)
            {;
                bufferStart++;
            }
            else{
                bufferStart += bufferCount;
            }
        }
        return bufferStart;
    }

    public int GetMessageStartIndex(){
        return GetBufferStartCount(14);
    }
}