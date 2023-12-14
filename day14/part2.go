package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
)

func getFile() []string {
	file, err := os.Open("./input.txt")
	if err != nil {
		log.Fatal(err)
	}
	defer file.Close()

	scanner := bufio.NewScanner(file)
	var data []string
	for scanner.Scan() {
		data = append(data, scanner.Text())
	}
	return data
}

func depositeRocks(input []string, i, j, rockCount int, dir int) []string {
	updatedInput := make([]string, len(input))
	copy(updatedInput, input)
	for k := 1; k < rockCount+1; k++ {
		var firstIndex int
		var secondIndex int

		if dir%2 == 0 {
			if dir < 2 {
				firstIndex = j + k
			} else {
				firstIndex = j - k
			}
			secondIndex = i
		} else {
			firstIndex = j
			if dir < 2 {
				secondIndex = i + k
			} else {
				secondIndex = i - k
			}
		}

		row := []rune(updatedInput[firstIndex])
		row[secondIndex] = 'O'
		updatedInput[firstIndex] = string(row)
	}

	return updatedInput
}

func checkNorthLoad(input []string) int {
	currScore := 0
	for i := 0; i < len(input[0]); i++ {
		for j := len(input) - 1; j >= 0; j-- {
			if input[j][i] == 'O' {
				currScore += len(input) - j
			}
		}
	}
	return currScore
}

func tilt(input []string, dir int) ([]string, int) {
	updatedInput := make([]string, len(input))
	copy(updatedInput, input)
	limit := len(input)
	if dir%2 == 0 {
		limit = len(input[0])
	}

	var firstIndex int
	var secondIndex int
	for i := 0; i < limit; i++ {
		start := 0
		if dir == 0 {
			start = len(input) - 1
		}
		if dir == 1 {
			start = len(input[0]) - 1
		}
		j := start
		rockCount := 0
		for {
			if dir == 2 && j >= len(input) {
				updatedInput = depositeRocks(updatedInput, secondIndex, len(input), rockCount, dir)
				break
			}

			if dir == 3 && j >= len(input[0]) {
				updatedInput = depositeRocks(updatedInput, len(input[0]), firstIndex, rockCount, dir)
				break
			}

			if dir < 2 && j < 0 {
				if dir == 0 {
					updatedInput = depositeRocks(updatedInput, secondIndex, -1, rockCount, dir)
				} else {
					updatedInput = depositeRocks(updatedInput, -1, firstIndex, rockCount, dir)
				}
				break
			}

			if dir%2 == 0 {
				firstIndex = j
				secondIndex = i
			} else {
				firstIndex = i
				secondIndex = j
			}

			if input[firstIndex][secondIndex] == 'O' {
				rockCount += 1
				row := []rune(updatedInput[firstIndex])
				row[secondIndex] = '.'
				updatedInput[firstIndex] = string(row)
			}

			if input[firstIndex][secondIndex] == '#' {
				updatedInput = depositeRocks(updatedInput, secondIndex, firstIndex, rockCount, dir)
				rockCount = 0
			}

			if dir > 1 {
				j++
			} else {
				j--
			}
		}
	}
	currentScore := checkNorthLoad(updatedInput)
	return updatedInput, currentScore
}

func cycle(input []string, scores []int) ([]string, []int) {
	updatedInput := input
	currentScore := make([]int, 0)
	for i := 0; i < 4; i++ {
		tmp := 0
		updatedInput, tmp = tilt(updatedInput, i)
		currentScore = append(currentScore, tmp)
	}

	return updatedInput, currentScore
}

func checkPrevScores(prevScores [][]int, currScore []int) int {
	if len(prevScores) == 0 {
		return -1
	}
	for i := len(prevScores) - 1; i >= 0; i-- {
		isEqual := true
		for j := 0; j < 4; j++ {
			if prevScores[i][j] != currScore[j] {
				isEqual = false
				break
			}
		}
		if isEqual == true {
			return len(prevScores) - i
		}
	}
	return -1
}

func main() {
	data := getFile()

	updatedInput := data
	prevScores := make([][]int, 0)
	prevScores = append(prevScores, make([]int, 4))
	for i := 0; i < 1000000000; i++ {
		tmp := i - 1
		if i == 0 {
			tmp = i
		}
		score := make([]int, 4)
		updatedInput, score = cycle(updatedInput, prevScores[tmp])
		cycle := checkPrevScores(prevScores, score)
		if cycle > 0 {
			fmt.Println(prevScores[i-cycle+(1000000000-i)%cycle][len(prevScores[0])-1])
			break
		}
		prevScores = append(prevScores, score)
	}
}
