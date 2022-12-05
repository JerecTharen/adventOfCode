using System.Collections;
using RPSLib;
namespace RSP;

public class RSPScorer
{
    private readonly List<Round> _gameRounds;
    public RSPScorer(String gameInput, Boolean isSecondColumnChosenOption = true)
    {
        _gameRounds = InputParser.GetGameRounds(gameInput, isSecondColumnChosenOption);
    }

    public int GetScore()
    {
        return ScoreCalculator.GetGameScore(_gameRounds);
    }
}