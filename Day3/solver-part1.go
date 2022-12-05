package main

import (
	"bufio"
	"fmt"
	"os"
	"strings"
	"unicode"
)

func main() {
	alphabetArray := generateAlphabetArray()

	file, err := os.Open("input.txt")
	input := [][]string{}
	if err != nil {
		fmt.Println(err)
	}
	defer file.Close()

	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		line := scanner.Text()
		stringLen := len(line)
		ruckOne := line[:(stringLen / 2)]
		ruckTwo := line[(stringLen / 2):]
		splitStr := []string{}
		splitStr = append(splitStr, ruckOne)
		splitStr = append(splitStr, ruckTwo)
		input = append(input, splitStr)
	}

	runningTotal := 0
	for i := 0; i < len(input); i++ {
		commonChar := getCommonChar(input[i][0], input[i][1])
		// fmt.Print(commonChar)
		score := getScore(commonChar, alphabetArray)
		// fmt.Println(score)
		// fmt.Println()
		runningTotal = runningTotal + score
	}
	// fmt.Print(alphabetArray)
	fmt.Print(runningTotal)

}

func getCommonChar(ruckOne string, ruckTwo string) string {
	commonChar := ""
	for i := 0; i < len(ruckOne); i++ {
		if strings.Contains(ruckTwo, string(ruckOne[i])) {
			commonChar = string(ruckOne[i])
		}
	}
	return commonChar
}

func getScore(letter string, alphabetArray [52]string) int {
	idxScore := indexOf(letter, alphabetArray[:])
	return idxScore + 1
}

func generateAlphabetArray() [52]string {
	alphaRay := [52]string{}
	counter := 0
	for r := 'a'; r <= 'z'; r++ {
		R := unicode.ToUpper(r)

		alphaRay[counter] = string(r)
		alphaRay[counter+26] = string(R)
		counter++
	}

	return alphaRay
}

func indexOf(word string, data []string) int {
	for k, v := range data {
		if word == v {
			return k
		}
	}
	return -1
}
