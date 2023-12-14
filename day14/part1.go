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

func calcRockDeposit(inputLen int, j int, rockCount int) int {
	score := 0
	for k := 0; k < rockCount; k++ {
		score += inputLen - j - k
	}
	return score
}

func main() {
	data := getFile()
	score := 0
	for i := 0; i < len(data[0]); i++ {
		rockCount := 0
		for j := len(data) - 1; j >= 0; j-- {
			if data[j][i] == 'O' {
				rockCount += 1
			}
			if data[j][i] == '#' {
				score += calcRockDeposit(len(data), j + 1, rockCount)
				rockCount = 0
			}
		}
		score += calcRockDeposit(len(data), 0, rockCount)
	}
	fmt.Println(score)
}
