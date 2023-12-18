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
	direction int64
	steps     int64
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

		color := splitInput[2][2 : len(splitInput[2])-1]
		direction, _ := strconv.ParseInt(color[5:6], 16, 64)
		steps, _ := strconv.ParseInt(color[0:5], 16, 64)
		data = append(data, instruction{
			direction: direction,
			steps:     steps,
			color:     color,
		})
	}
	return data
}

func getAllPolygonPoints(data []instruction) float64 {
	nodes := [][2]int64{}
	x := int64(0)
	y := int64(0)
	area := 0.0
	edge := 0.0
	nodes = append(nodes, [2]int64{x, y})
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
		nodes = append(nodes, [2]int64{x, y})
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
