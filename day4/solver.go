package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

func main() {
	file, err := os.Open("input.txt")
	if err != nil {
		fmt.Println(err)
	}
	defer file.Close()
	fullOverlapResultCounter := 0
	partialOverlapResultCounter := 0
	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		line := scanner.Text()
		separateCodes := strings.Split(line, ",")

		p1Input := strings.Split(separateCodes[0], "-")
		p1Min, err1Min := strconv.Atoi(p1Input[0])
		p1Max, err1Max := strconv.Atoi(p1Input[1])

		p2Input := strings.Split(separateCodes[1], "-")
		p2Min, err2Min := strconv.Atoi(p2Input[0])
		p2Max, err2Max := strconv.Atoi(p2Input[1])

		if err1Min == nil && err1Max == nil && err2Min == nil && err2Max == nil {
			tempMap := map[string]int{
				"p1Min": p1Min,
				"p1Max": p1Max,
				"p2Min": p2Min,
				"p2Max": p2Max,
			}

			smallerRangePersonIdx := getSmallerRangePerson(tempMap)
			if smallerRangePersonIdx == 99 {
				fmt.Print("ERROR!")
			}

			isRangeFullyContainedResult := false
			isRangePartiallyContainedResult := false
			if smallerRangePersonIdx == 0 {
				isRangeFullyContainedResult = isRangeFullyContained(p2Min, p2Max, p1Min, p1Max)
				isRangePartiallyContainedResult = isRangePartiallyContained(p2Min, p2Max, p1Min, p1Max)
			} else if smallerRangePersonIdx == 1 {
				isRangeFullyContainedResult = isRangeFullyContained(p1Min, p1Max, p2Min, p2Max)
				isRangePartiallyContainedResult = isRangePartiallyContained(p1Min, p1Max, p2Min, p2Max)
			} else if smallerRangePersonIdx == 2 {
				if (p1Min == p2Min) && (p1Max == p2Max) {
					fullOverlapResultCounter++
					partialOverlapResultCounter++
				} else {
					isRangePartiallyContainedResult = isRangePartiallyContained(p1Min, p1Max, p2Min, p2Max)
				}
			}

			if isRangeFullyContainedResult == true {
				fullOverlapResultCounter++
			}
			if isRangePartiallyContainedResult == true {
				partialOverlapResultCounter++
			}
		}
	}

	fmt.Print(fullOverlapResultCounter)
	fmt.Print(partialOverlapResultCounter)
}

//returns 0 for p1, 1 for p2, 2 for same, 99 for error
func getSmallerRangePerson(personMap map[string]int) int {
	range1 := personMap["p1Max"] - personMap["p1Min"]
	range2 := personMap["p2Max"] - personMap["p2Min"]

	if (range1 != range2) && range1 < range2 {
		return 0
	}
	if (range1 != range2) && range1 > range2 {
		return 1
	}
	if range1 == range2 {
		return 2
	} else {
		return 99
	}
}

func isRangeFullyContained(biggerRangeMin int, biggerRangeMax int, smallerRangeMin int, smallerRangeMax int) bool {
	if (int(smallerRangeMin) >= int(biggerRangeMin)) && (int(smallerRangeMax) <= int(biggerRangeMax)) {
		return true
	} else {
		return false
	}
}

func isRangePartiallyContained(biggerRangeMin int, biggerRangeMax int, smallerRangeMin int, smallerRangeMax int) bool {
	if (int(smallerRangeMin) >= int(biggerRangeMin)) && (int(smallerRangeMin) <= biggerRangeMax) {
		return true
	} else if (int(smallerRangeMax) <= int(biggerRangeMax)) && (int(smallerRangeMax) >= biggerRangeMin) {
		return true
	} else {
		return false
	}
}
