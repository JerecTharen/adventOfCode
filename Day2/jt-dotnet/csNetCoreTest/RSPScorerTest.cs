using RSP;

public class RSPScorerTest
{
    [Fact]
    public void GetScore_ShouldCalculateExample_To15Points()
    {
        const int expectedScore = 15;
        const String stubInput = @"A Y
B X
C Z";

        int actualGameScore = new RSPScorer(stubInput).GetScore();

        Assert.Equal(expectedScore, actualGameScore);
    }

    [Fact]
    public void GetScore_ShouldCalculatePart2Example_To12Points()
    {
        const int expectedGameScore = 12;
        const Boolean isSecondColumnChosenOption = false;
        const String stubGameInput = @"A Y
B X
C Z";

        int actualGameScore = new RSPScorer(stubGameInput, isSecondColumnChosenOption).GetScore();

        Assert.Equal(expectedGameScore, actualGameScore);
    }
}