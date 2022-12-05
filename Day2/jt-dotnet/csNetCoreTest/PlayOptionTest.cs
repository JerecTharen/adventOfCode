using RPSLib;

public class PlayOptionTest
{
    [Fact]
    public void PlayOption_Rock_ShouldBeWorthOnePoint()
    {
        const int expectedRockValue = 1;
        int actualRockValue = (int)PlayOption.Rock;
        Assert.Equal(expectedRockValue, actualRockValue);
    }
    [Fact]
    public void PlayOption_Paper_ShouldBeWorthTwoPoints()
    {
        const int expectedPaperValue = 2;
        int actualPaperValue = (int)PlayOption.Paper;
        Assert.Equal(expectedPaperValue, actualPaperValue);
    }
    [Fact]
    public void PlayOption_Scissors_ShouldBeWorthThreePoints()
    {
        const int expectedScissorsValue = 3;
        int actualScissorsValue = (int)PlayOption.Scissors;
        Assert.Equal(expectedScissorsValue, actualScissorsValue);
    }
}