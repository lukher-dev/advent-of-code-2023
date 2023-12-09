package main

import (
	"bufio"
	"fmt"
	"log"
	"math"
	"os"
	"strconv"
	"strings"
)

func getFile() [][]float64 {
	file, err := os.Open("./input.txt")
	if err != nil {
		log.Fatal(err)
	}
	defer file.Close()

	scanner := bufio.NewScanner(file)
	var data [][]float64
	for scanner.Scan() {
		var array []float64
		for _, i := range strings.Fields(strings.Split(scanner.Text(), ":")[1]) {
			j, _ := strconv.Atoi(i)
			array = append(array, float64(j))
		}
		data = append(data, array)
	}
	return data
}

func main() {
	data := getFile()
	result := 1
	for i := range data[0] {
		start := (data[0][i] - math.Sqrt(math.Pow(data[0][i], 2)-4*data[1][i])) / 2
		finish := (data[0][i] + math.Sqrt(math.Pow(data[0][i], 2)-4*data[1][i])) / 2

		result *= int(math.Ceil(finish)) - int(math.Floor(start)) - 1
	}
	fmt.Println("result: ", result)
}
