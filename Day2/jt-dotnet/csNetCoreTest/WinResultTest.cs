using RPSLib;

public class WinResultTest
{
    [Fact]
    public void WinResult_WIN_ShouldBeWorth6Points()
    {
        const int expectedWINPoints = 6;
        int actualWINPoints = (int)WinResult.WIN;
        Assert.Equal(expectedWINPoints, actualWINPoints);
    }
    [Fact]
    public void WinResult_LOSS_ShouldBeWorth0Points()
    {
        const int expectedLOSSPoints = 0;
        int actualLOSSPoints = (int)WinResult.LOSS;
        Assert.Equal(expectedLOSSPoints, actualLOSSPoints);
    }
    [Fact]
    public void WinResult_DRAW_ShouldBeWorth3Points()
    {
        const int expectedDRAWPoints = 3;
        int actualDRAWPoints = (int)WinResult.DRAW;
        Assert.Equal(expectedDRAWPoints, actualDRAWPoints);
    }
}