using RPSLib;
using System.Collections;

public class ScoreCalculatorTest
{
    #region GetRoundScore
    [Fact]
    public void GetRoundScore_PaperWin_ShouldBeWorth8Points()
    {
        const int expectedPoints = 8;
        const PlayOption playedOption = PlayOption.Paper;
        const WinResult winResult = WinResult.WIN;

        int actualPoints = ScoreCalculator.GetRoundScore(playedOption, winResult);
        
        Assert.Equal(expectedPoints, actualPoints);
    }
    [Fact]
    public void GetRoundScore_RockLoss_ShouldBeWorth1Point()
    {
        const int expectedPoint = 1;
        const PlayOption playedOption = PlayOption.Rock;
        const WinResult winResult = WinResult.LOSS;

        int actualPoint = ScoreCalculator.GetRoundScore(playedOption, winResult);
        
        Assert.Equal(expectedPoint, actualPoint);
    }
    [Fact]
    public void GetRoundScore_ScissorsDraw_ShouldBeWorth6Points()
    {
        const int expectedPoints = 6;
        const PlayOption playedOption = PlayOption.Scissors;
        const WinResult winResult = WinResult.DRAW;

        int actualPoints = ScoreCalculator.GetRoundScore(playedOption, winResult);
        
        Assert.Equal(expectedPoints, actualPoints);
    }
    #endregion

    #region GetWinResult
    [Fact]
    public void GetWinResult_PaperAgainstRock_ShouldWin()
    {
        const WinResult expectedWinResult = WinResult.WIN;
        const PlayOption chosenOption = PlayOption.Paper;
        const PlayOption opponentOption = PlayOption.Rock;
        
        WinResult actualWinResult = ScoreCalculator.GetWinResult(chosenOption, opponentOption);
        
        Assert.Equal(expectedWinResult, actualWinResult);
    }
    [Fact]
    public void GetWinResult_RockAgainstPaper_ShouldLose()
    {
        const WinResult expectedWinResult = WinResult.LOSS;
        const PlayOption chosenOption = PlayOption.Rock;
        const PlayOption opponentOption = PlayOption.Paper;
        
        WinResult actualWinResult = ScoreCalculator.GetWinResult(chosenOption, opponentOption);
        
        Assert.Equal(expectedWinResult, actualWinResult);
    }
    [Fact]
    public void GetWinResult_ScissorsBoth_ShouldDraw()
    {
        const WinResult expectedWinResult = WinResult.DRAW;
        const PlayOption chosenOption = PlayOption.Scissors;
        const PlayOption opponentOption = PlayOption.Scissors;
        
        WinResult actualWinResult = ScoreCalculator.GetWinResult(chosenOption, opponentOption);
        
        Assert.Equal(expectedWinResult, actualWinResult);
    }
    #endregion

    [Fact]
    public void GetGameScore_ShouldCalculateTheExample_ToBe15Points()
    {
        const int expectedGameScore = 15;
        List<Round> stubGameRounds = new List<Round>()
        {
            new Round(PlayOption.Paper, PlayOption.Rock),
            new Round(PlayOption.Rock, PlayOption.Paper),
            new Round(PlayOption.Scissors, PlayOption.Scissors)
        };

        int actualGameScore = ScoreCalculator.GetGameScore(stubGameRounds);

        Assert.Equal(expectedGameScore, actualGameScore);
    }
}