// See https://aka.ms/new-console-template for more information
using csNetCoreClassLib;
using System.IO;
namespace RSP;

public class RockPaperScissors
{
    public static void Main(String[] args)
    {
        Console.WriteLine("Hello, World!");
        Console.WriteLine($"Importing integer One from class library: {Class1.One}");

        //Read file - assumes it is starting in root of jt-dotnet
        String gameInput = System.IO.File.ReadAllText("./ignore_data/input.txt");
        //Parse input and get score
        int gameScore = new RSPScorer(gameInput, false).GetScore();

        System.Console.WriteLine($"Game score is: {gameScore}");
    }
}
