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

type instruction struct {
	direction int
	steps     int
	color     string
}

func getFile() []instruction {
	file, err := os.Open("./input.txt")
	if err != nil {
		log.Fatal(err)
	}
	defer file.Close()

	scanner := bufio.NewScanner(file)
	var data []instruction
	for scanner.Scan() {
		splitInput := strings.Split(scanner.Text(), " ")
		steps, _ := strconv.Atoi(splitInput[1])
		dir := 0
		if splitInput[0] == "R" {
			dir = 1
		}
		if splitInput[0] == "D" {
			dir = 2
		}
		if splitInput[0] == "L" {
			dir = 3
		}

		data = append(data, instruction{
			direction: dir,
			steps:     steps,
			color:     splitInput[2][1 : len(splitInput[2])-1],
		})
	}
	return data
}

func getAllPolygonPoints(data []instruction) float64 {
	nodes := [][2]int{}
	x := 0
	y := 0
	area := 0.0
	edge := 0.0
	nodes = append(nodes, [2]int{x, y})
	for i, v := range data {
		if v.direction == 0 {
			x -= v.steps
		}
		if v.direction == 1 {
			y += v.steps
		}
		if v.direction == 2 {
			x += v.steps
		}
		if v.direction == 3 {
			y -= v.steps
		}
		nodes = append(nodes, [2]int{x, y})
		edge += float64(v.steps)
		area += float64(nodes[i][0]*nodes[i+1][1]-nodes[i+1][0]*nodes[i][1]) / 2
	}
	return math.Abs(area) + 1 - (edge / 2) + edge
}

func main() {
	data := getFile()
	result := getAllPolygonPoints(data)
	fmt.Println(result)
}
