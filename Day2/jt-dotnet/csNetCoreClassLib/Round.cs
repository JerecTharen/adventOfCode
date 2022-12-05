namespace RPSLib;

public class Round
{
    public readonly PlayOption ChosenOption;
    public readonly PlayOption OpponentOption;
    public WinResult WinResult { get; set; }
    public Round(PlayOption chosenOption, PlayOption opponentOption)
    {
        ChosenOption = chosenOption;
        OpponentOption = opponentOption;
    }
}