using RPSLib;
using FluentAssertions;

public class InputParserTest
{
    [Fact]
    public void GetGameRounds_ShouldParseExampleInput_Successfully()
    {
        List<Round> expectedGameRounds = new List<Round>()
        {
            new Round(PlayOption.Paper, PlayOption.Rock),
            new Round(PlayOption.Rock, PlayOption.Paper),
            new Round(PlayOption.Scissors, PlayOption.Scissors)
        };
        const String gameInput = @"A Y
B X
C Z";

        List<Round> actualGameRounds = InputParser.GetGameRounds(gameInput);

        //Need to use fluent assertion library for comparing Lists
        actualGameRounds.Should().BeEquivalentTo(expectedGameRounds);
    }

    #region GetOpponentOptionFromChar
    [Fact]
    public void GetOpponentOptionFromChar_ShouldReturnRock_ForA()
    {
        const PlayOption expectedOpponentOption = PlayOption.Rock;
        const Char a = 'A';

        PlayOption actualOpponentOption = InputParser.GetOpponentOptionFromChar(a);

        Assert.Equal(expectedOpponentOption, actualOpponentOption);
    }
    [Fact]
    public void GetOpponentOptionFromChar_ShouldReturnPaper_ForB()
    {
        const PlayOption expectedOpponentOption = PlayOption.Paper;
        const Char b = 'B';

        PlayOption actualOpponentOption = InputParser.GetOpponentOptionFromChar(b);

        Assert.Equal(expectedOpponentOption, actualOpponentOption);
    }
    [Fact]
    public void GetOpponentOptionFromChar_ShouldReturnScissors_ForC()
    {
        const PlayOption expectedOpponentOption = PlayOption.Scissors;
        const Char c = 'C';

        PlayOption actualOpponentOption = InputParser.GetOpponentOptionFromChar(c);

        Assert.Equal(expectedOpponentOption, actualOpponentOption);
    }
    #endregion

    #region GetChosenOptionFromChar
    [Fact]
    public void GetChosenOptionFromChar_ShouldReturnPaper_ForY()
    {
        const PlayOption expectedChosenOption = PlayOption.Paper;
        const Char y = 'Y';

        PlayOption actualChosenOption = InputParser.GetChosenOptionFromChar(y);

        Assert.Equal(expectedChosenOption, actualChosenOption);
    }
    [Fact]
    public void GetChosenOptionFromChar_ShouldReturnRock_ForX()
    {
        const PlayOption expectedChosenOption = PlayOption.Rock;
        const Char x = 'X';

        PlayOption actualChosenOption = InputParser.GetChosenOptionFromChar(x);

        Assert.Equal(expectedChosenOption, actualChosenOption);
    }
    [Fact]
    public void GetChosenOptionFromChar_ShouldReturnScissors_ForZ()
    {
        const PlayOption expectedOpponentOption = PlayOption.Scissors;
        const Char z = 'Z';

        PlayOption actualChosenOption = InputParser.GetChosenOptionFromChar(z);

        Assert.Equal(expectedOpponentOption, actualChosenOption);
    }
    #endregion

    #region GetWinOptionFromChar
    [Fact]
    public void GetWinOptionFromChar_ShouldReturnWin_ForZ()
    {
        const WinResult expectedResult = WinResult.WIN;
        const Char stubInput = 'Z';

        WinResult actualResult = InputParser.GetWinResultFromChar(stubInput);

        Assert.Equal(expectedResult, actualResult);
    }
    [Fact]
    public void GetWinOptionFromChar_ShouldReturnLoss_ForX()
    {
        const WinResult expectedResult = WinResult.LOSS;
        const Char stubInput = 'X';

        WinResult actualResult = InputParser.GetWinResultFromChar(stubInput);

        Assert.Equal(expectedResult, actualResult);
    }
    [Fact]
    public void GetWinOptionFromChar_ShouldReturnDraw_ForY()
    {
        const WinResult expectedResult = WinResult.DRAW;
        const Char stubInput = 'Y';

        WinResult actualResult = InputParser.GetWinResultFromChar(stubInput);

        Assert.Equal(expectedResult, actualResult);
    }
    #endregion
}