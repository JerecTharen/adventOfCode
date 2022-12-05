namespace RPSLib;

public class InputParser
{
    public static List<Round> GetGameRounds(String input, Boolean isSecondColumnChosenOption = true)
    {
        //Order matters - parse opponent then chosen
        PlayOption? tempOppenentOption = null;
        List<Round> gameRounds = new List<Round>();
        foreach(Char c in input)
        {
            if(_allowedCharacters.Contains(c))
            {
                //Parse character
                try
                {
                    if(!tempOppenentOption.HasValue)
                    {
                        tempOppenentOption = GetOpponentOptionFromChar(c);
                    }
                    else if(isSecondColumnChosenOption)
                    {
                        gameRounds.Add(new Round(GetChosenOptionFromChar(c), tempOppenentOption.Value));
                        tempOppenentOption = null;
                    }
                    else if(!isSecondColumnChosenOption)
                    {
                        gameRounds.Add
                        (
                            new Round(ScoreCalculator.GetPlayOptionFromResultAndOpponent(tempOppenentOption.Value, GetWinResultFromChar(c)), tempOppenentOption.Value)
                        );
                        tempOppenentOption = null;
                    }
                }
                catch (System.Exception ex)
                {
                    Console.WriteLine($"Could not parse {c}");
                    throw ex;
                }
            }
        }
        if(tempOppenentOption.HasValue)
        {
            throw new ArgumentException("Input had an odd number of Opponent and Chosen game options, cannot parse it.");
        }
        return gameRounds;
    }

    public static PlayOption GetOpponentOptionFromChar(Char opponentChar)
    {
        switch(opponentChar)
        {
            case 'A':
                return PlayOption.Rock;
            case 'B':
                return PlayOption.Paper;
            case 'C':
                return PlayOption.Scissors;
            default:
                throw new ArgumentException($"Can only parse opponent input A, B, or C. Received: {opponentChar}");
        }
    }

    public static PlayOption GetChosenOptionFromChar(Char chosenChar)
    {
        switch(chosenChar)
        {
            case 'X':
                return PlayOption.Rock;
            case 'Y':
                return PlayOption.Paper;
            case 'Z':
                return PlayOption.Scissors;
            default:
                throw new ArgumentException($"Can only parse chosen input X, Y, or Z. Received: {chosenChar}");
        }
    }

    public static WinResult GetWinResultFromChar(Char winChar)
    {
        switch(winChar)
        {
            case 'X':
                return WinResult.LOSS;
            case 'Y':
                return WinResult.DRAW;
            case 'Z':
                return WinResult.WIN;
            default:
                throw new ArgumentOutOfRangeException($"Can only parse win result from X, Y, or Z. Recieved: {winChar}");
        }
    }

    private static readonly Char[] _allowedCharacters = new Char[]{'A', 'B', 'C', 'X', 'Y', 'Z'};
}