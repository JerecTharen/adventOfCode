using System.Linq;
using System.Collections;
namespace RPSLib;

public class ScoreCalculator
{
    public static int GetRoundScore(PlayOption playOption, WinResult winResult)
    {
        return (int)playOption + (int)winResult;
    }

    public static WinResult GetWinResult(PlayOption chosenOption, PlayOption opponentOption)
    {
        if(chosenOption == opponentOption)
        {
            return WinResult.DRAW;
        }
        
        switch(chosenOption)
        {
            case PlayOption.Rock:
                return opponentOption == PlayOption.Paper ? WinResult.LOSS : WinResult.WIN;
            case PlayOption.Paper:
                return opponentOption == PlayOption.Scissors ? WinResult.LOSS : WinResult.WIN;
            case PlayOption.Scissors:
                return opponentOption == PlayOption.Rock ? WinResult.LOSS : WinResult.WIN;
            default:
                throw new NotSupportedException("Play options other than Rock, Paper, or Scissors are not supported!");
        }
    }

    public static PlayOption GetPlayOptionFromResultAndOpponent(PlayOption opponentOption, WinResult winResult)
    {
        if(winResult == WinResult.DRAW)
        {
            return opponentOption;
        }
        Boolean isWin = winResult == WinResult.WIN;
        switch(opponentOption)
        {
            case PlayOption.Rock:
                return isWin ? PlayOption.Paper : PlayOption.Scissors;
            case PlayOption.Paper:
                return isWin ? PlayOption.Scissors : PlayOption.Rock;
            case PlayOption.Scissors:
                return isWin ? PlayOption.Rock : PlayOption.Paper;
            default:
                throw new ArgumentOutOfRangeException("Can only play with Rock, Paper, or Scissors");
        }
    }

    public static int GetGameScore(List<Round> gameRounds)
    {
        return gameRounds
            .Select(r => 
            {
                r.WinResult = ScoreCalculator.GetWinResult(r.ChosenOption, r.OpponentOption);
                return ScoreCalculator.GetRoundScore(r.ChosenOption, r.WinResult);
            })
            .Sum();
    }

    public static Dictionary<PlayOption, PlayOption> WinOptionByOponentPlayOption = new Dictionary<PlayOption, PlayOption>()
    {
        {PlayOption.Rock, PlayOption.Paper},
        {PlayOption.Paper, PlayOption.Scissors},
        {PlayOption.Scissors, PlayOption.Rock}
    };
}