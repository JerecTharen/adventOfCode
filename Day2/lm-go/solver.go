package main
 import (
     "fmt"
     "os"
     "bufio"
     "strings"
 )

 func main() {

     file, err := os.Open("input.txt")
     input:= [][]string{}
     if err != nil {
         fmt.Println(err)
     }
     defer file.Close()

     scanner := bufio.NewScanner(file)
     for scanner.Scan() {
         splitStr:= strings.Fields(scanner.Text())
         if (len(splitStr) == 2){
             input= append(input, splitStr)
         }
     }

     if err := scanner.Err(); err != nil {
         fmt.Println(err)
     }

     runningTotalP1 := 0
     runningTotalP2 := 0
     for i := 0; i < len(input); i++ {
         round := input[i]

         yourPlay := round[1]
         theirPlay := round[0]
         result := getRoundResultP1(yourPlay, theirPlay)
         roundScoreP1 := scoreRound(yourPlay, result)
         runningTotalP1 = runningTotalP1 + roundScoreP1

         desiredResult := round[1]
         neededShape := getNeededShape(theirPlay, desiredResult)
         roundScoreP2 := scoreRound2(neededShape, desiredResult)
         runningTotalP2 = runningTotalP2 + roundScoreP2
     }
     fmt.Println(runningTotalP1)
     fmt.Println(runningTotalP2)
 }

 var attackOptionMap = map[string]string{
     "A": "rock",
     "B": "paper",
     "C": "scissors",
     "X": "rock",
     "Y": "paper",
     "Z": "scissors",
 }

 var attackOptionEncodeMap = map[string]string{
     "rock": "X",
     "paper": "Y",
     "scissors": "Z",
 }

 var resultEncoder = map[string]string{
     "X": "lose",
     "Y": "draw",
     "Z": "win",
 }

 var translateTheirPlayToYours = map[string]string{
     "A": "X",
     "B": "Y",
     "C": "Z",
 }

 var playScoreMap = map[string]int{
     "A": 1,
     "B": 2,
     "C": 3,
     "X": 1,
     "Y": 2,
     "Z": 3,
 }

 var resultScoreMap = map[string]int{
     "win": 6,
     "lose": 0,
     "draw": 3,
 }

 func getRoundResultP1(yourPlayInput string, theirPlayInput string) string{
     yourPlay:= attackOptionMap[yourPlayInput]
     theirPlay:= attackOptionMap[theirPlayInput]

     if (yourPlay == theirPlay){
         return "draw"
     } else if (yourPlay == "rock" && theirPlay == "scissors"){
         return "win"
     } else if (yourPlay == "paper" && theirPlay == "rock"){
         return "win"
     } else if (yourPlay == "scissors" && theirPlay == "paper"){
         return "win"
     } else{
         return "lose"
     }
 }

 func getNeededShape(theirPlayInput string, roundResultInput string) string{
     theirPlay := attackOptionMap[theirPlayInput]
     roundResult := resultEncoder[roundResultInput]
     result := "a"
     if (roundResult == "win"){
         if(theirPlay == "rock"){
             result = attackOptionEncodeMap["paper"]
         } else if(theirPlay == "paper"){
             result = attackOptionEncodeMap["scissors"]
         } else if(theirPlay == "scissors"){
             result = attackOptionEncodeMap["rock"]
         }
     } else if roundResult == "lose" {
         if(theirPlay == "rock"){
             result = attackOptionEncodeMap["scissors"]
         } else if(theirPlay == "paper"){
             result = attackOptionEncodeMap["rock"]
         } else if(theirPlay == "scissors"){
             result = attackOptionEncodeMap["paper"]
         }
     } else{
         result = translateTheirPlayToYours[theirPlayInput]
     }
     return result
 }

 func scoreRound(yourPlay string, result string) int{
     return playScoreMap[yourPlay] + resultScoreMap[result]
 }

 func scoreRound2(yourPlay string, result string) int{
     return playScoreMap[yourPlay] + resultScoreMap[resultEncoder[result]]
 }
