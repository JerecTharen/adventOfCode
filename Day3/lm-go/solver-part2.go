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
	input := []string{}
	if err != nil {
		fmt.Println(err)
	}
	defer file.Close()

	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		line := scanner.Text()
		input = append(input, line)
	}

	runningTotal := 0
	for i := 0; i < len(input)/3; i++ {
		commonChar := getCommonChar(input[(i*3)], input[(i*3)+1], input[(i*3)+2])
		score := getScore(commonChar, alphabetArray)
		runningTotal = runningTotal + score
	}
	fmt.Print(runningTotal)

}

func getCommonChar(ruckOne string, ruckTwo string, ruckThree string) string {
	commonChar := ""
	error := false
	for i := 0; i < len(ruckOne); i++ {
		if strings.Contains(ruckTwo, string(ruckOne[i])) && strings.Contains(ruckThree, string(ruckOne[i])) {
			commonChar = string(ruckOne[i])
		}
	}
	if commonChar == "" {
		error = true
	}

	if error {
		fmt.Print("ERROR!")
		fmt.Print(ruckOne + " ")
		fmt.Print(ruckTwo + " ")
		fmt.Print(ruckThree)
		fmt.Println()
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
