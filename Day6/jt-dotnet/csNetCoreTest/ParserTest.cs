using ReceiverLib;

public class ParserTest{

    [Fact]
    public void Parser_Returns7_ForFirstExample(){
        const int EXPECTED_BUFFER_START = 7;
        String stubExampleInput = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";

        int bufferStart = new Parser(stubExampleInput).GetBufferStartCount();

        Assert.Equal(EXPECTED_BUFFER_START, bufferStart);
    }
    [Fact]
    public void Parser_Returns5_ForSecondExample(){
        const int EXPECTED_BUFFER_START = 5;
        String stubExampleInput = "bvwbjplbgvbhsrlpgdmjqwftvncz";

        int bufferStart = new Parser(stubExampleInput).GetBufferStartCount();

        Assert.Equal(EXPECTED_BUFFER_START, bufferStart);
    }
    [Fact]
    public void Parser_Returns6_ForThirdExample(){
        const int EXPECTED_BUFFER_START = 6;
        String stubExampleInput = "nppdvjthqldpwncqszvftbrmjlhg";

        int bufferStart = new Parser(stubExampleInput).GetBufferStartCount();

        Assert.Equal(EXPECTED_BUFFER_START, bufferStart);
    }
    [Fact]
    public void Parser_Returns10_ForFourthExample(){
        const int EXPECTED_BUFFER_START = 10;
        String stubExampleInput = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";

        int bufferStart = new Parser(stubExampleInput).GetBufferStartCount();

        Assert.Equal(EXPECTED_BUFFER_START, bufferStart);
    }
    [Fact]
    public void Parser_Returns11_ForFifthExample(){
        const int EXPECTED_BUFFER_START = 11;
        String stubExampleInput = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";

        int bufferStart = new Parser(stubExampleInput).GetBufferStartCount();

        Assert.Equal(EXPECTED_BUFFER_START, bufferStart);
    }

    //Message parser tests
    [Fact]
    public void MessageParser_Returns19_ForFirstExample(){
        const int EXPECTED_BUFFER_START = 19;
        String stubExampleInput = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";

        int bufferStart = new Parser(stubExampleInput).GetMessageStartIndex();

        Assert.Equal(EXPECTED_BUFFER_START, bufferStart);
    }
    [Fact]
    public void MessageParser_Returns23_ForSecondExample(){
        const int EXPECTED_BUFFER_START = 23;
        String stubExampleInput = "bvwbjplbgvbhsrlpgdmjqwftvncz";

        int bufferStart = new Parser(stubExampleInput).GetMessageStartIndex();

        Assert.Equal(EXPECTED_BUFFER_START, bufferStart);
    }
    [Fact]
    public void MessageParser_Returns23_ForThirdExample(){
        const int EXPECTED_BUFFER_START = 23;
        String stubExampleInput = "nppdvjthqldpwncqszvftbrmjlhg";

        int bufferStart = new Parser(stubExampleInput).GetMessageStartIndex();

        Assert.Equal(EXPECTED_BUFFER_START, bufferStart);
    }
    [Fact]
    public void MessageParser_Returns29_ForFourthExample(){
        const int EXPECTED_BUFFER_START = 29;
        String stubExampleInput = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";

        int bufferStart = new Parser(stubExampleInput).GetMessageStartIndex();

        Assert.Equal(EXPECTED_BUFFER_START, bufferStart);
    }
    [Fact]
    public void MessageParser_Returns26_ForFifthExample(){
        const int EXPECTED_BUFFER_START = 26;
        String stubExampleInput = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";

        int bufferStart = new Parser(stubExampleInput).GetMessageStartIndex();

        Assert.Equal(EXPECTED_BUFFER_START, bufferStart);
    }
}