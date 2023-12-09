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

func getFile() []float64 {
	file, err := os.Open("./input.txt")
	if err != nil {
		log.Fatal(err)
	}
	defer file.Close()

	scanner := bufio.NewScanner(file)

	var data []float64
	for scanner.Scan() {
		input, _ := strconv.Atoi(strings.Join(strings.Fields(strings.Split(scanner.Text(), ":")[1]), ""))
		data = append(data, float64(input))
	}
	return data
}

func main() {
	data := getFile()

	start := (data[0] - math.Sqrt(math.Pow(data[0], 2)-4*data[1])) / 2
	finish := (data[0] + math.Sqrt(math.Pow(data[0], 2)-4*data[1])) / 2

	result := int(math.Ceil(finish)) - int(math.Floor(start)) - 1
	fmt.Println("result: ", result)
}
