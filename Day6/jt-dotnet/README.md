# C\# .NET Core Template

A quick template for a C\# .NET Core (version 6) CLI applicaiton with separated class library and XUnit testing projects.

# Setup
1. Download the files or clone the repo and copy them to the desired directory
2. Install the [CLI tool](https://dotnet.microsoft.com/en-us/download)
3. Run the application from the root of this directory with `dotnet run --project csNetCoreCLI/csNetCoreCLI.csproj` or navigate inside the CLI directory and `dotnet run`
4. Run tests from the root of this directory with `dotnet test csNetCoreTest/csNetCoreTest.csproj` or navigate inside the test directory and `dotnet test`
5. [Add new a new project](https://learn.microsoft.com/en-us/dotnet/core/tools/dotnet-new) with `dotnet new <TEMPLATE>`
6. Build the CLI application with `dotnet build --project csNetCoreCLI/csNetCoreCLI.csproj` or inside the CLI directory with `dotnet build` then execute the binary in `/csNetCoreCLI/bin/Debug/net6.0/csNetCoreCLI` - the contents of the `net6.0` directory are the deployable files for "production"