using csNetCoreClassLib;
namespace csNetCoreTest;
public class UnitTest1
{
    [Fact]
    public void Test1_ShouldBeAbleToImport_One()
    {
        Assert.Equal(1, Class1.One);
    }
}