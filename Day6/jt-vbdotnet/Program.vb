Imports System

Module Program
    Sub Main(args As String())
        Console.WriteLine("Hello World!")
        ' Pass in file input string here
        dim parse = New Parser("mjqjpqmgbljsphdztnvjfqwrcgsmlb")
        dim parseCount = parse.GetBufferStartCount()
        Console.WriteLine(parseCount)
    End Sub
End Module

Public Class Parser
    Private Property _input As String
    Public Sub New(ByVal input As String)
        _input = input
    End Sub

    Public Function GetBufferStartCount()
        dim bufferCount = 4
        dim isBufferStartFound = false
        dim bufferStart = 0
        While isBufferStartFound = false
            dim nextCharacters = _input.Substring(bufferStart, bufferCount)
            dim duplicateCharacters = nextCharacters.Distinct()
            isBufferStartFound = duplicateCharacters.Count() >= nextCharacters.Count()
            If isBufferStartFound = false Then
                bufferStart = bufferStart + 1
            Else
                bufferStart = bufferStart + bufferCount
            End If
        End While
        Return bufferStart
    End Function

end Class
