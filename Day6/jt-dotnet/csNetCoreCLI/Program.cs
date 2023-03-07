// See https://aka.ms/new-console-template for more information
using System.IO;
using csNetCoreClassLib;
using ReceiverLib;


public class Program{
    public static void Main(String[] args)
    {
        Console.WriteLine("Hello, World!");
        Console.WriteLine($"Importing integer One from class library: {Class1.One}");

        String file = File.ReadAllText("./ignore_data/input.txt");
        System.Console.WriteLine($"Read file of length: {file.Count()}");
        Parser parser = new Parser(file);
        System.Console.WriteLine($"Buffer starts at character number: {parser.GetBufferStartCount()}");
        System.Console.WriteLine($"Message starts at character number: {parser.GetMessageStartIndex()}");

    }
}