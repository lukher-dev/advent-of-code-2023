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

func findPossibleDir(data []string, visitedNodes [][]int, i int, j int) [2]int {
	if (data[i][j] == 'S' || data[i][j] == '|' || data[i][j] == 'L' || data[i][j] == 'J') &&
		i > 0 &&
		visitedNodes[i-1][j] == 0 &&
		(data[i-1][j] == '7' || data[i-1][j] == 'F' || data[i-1][j] == '|') {
		// fmt.Println("up")
		return [2]int{i - 1, j}
	}
	if (data[i][j] == 'S' || data[i][j] == '|' || data[i][j] == 'F' || data[i][j] == '7') &&
		i+1 < len(data) &&
		visitedNodes[i+1][j] == 0 &&
		(data[i+1][j] == 'L' || data[i+1][j] == 'J' || data[i+1][j] == '|') {
		// fmt.Println("down")
		return [2]int{i + 1, j}
	}
	if (data[i][j] == 'S' || data[i][j] == '-' || data[i][j] == '7' || data[i][j] == 'J') &&
		j > 0 &&
		visitedNodes[i][j-1] == 0 &&
		(data[i][j-1] == 'L' || data[i][j-1] == 'F' || data[i][j-1] == '-') {
		// fmt.Println("left")
		return [2]int{i, j - 1}
	}
	if (data[i][j] == 'S' || data[i][j] == '-' || data[i][j] == 'L' || data[i][j] == 'F') &&
		j+1 < len(data[0]) &&
		visitedNodes[i][j+1] == 0 &&
		(data[i][j+1] == '7' || data[i][j+1] == 'J' || data[i][j+1] == '-') {
		// fmt.Println("right")
		return [2]int{i, j + 1}
	}
	return [2]int{-1, -1}
}

func findStartingPoints(data []string) [2]int {
	for i := 0; i < len(data); i++ {
		for j := 0; j < len(data[0]); j++ {
			if data[i][j] == 'S' {
				return [2]int{i, j}
			}
		}
	}
	return [2]int{-1, -1}
}

func findEnd(data []string) [][]int {
	traversedNodes := make([][]int, len(data))
	for i := 0; i < len(data); i++ {
		traversedNodes[i] = make([]int, len(data[0]))
	}
	var nodesToExplore [][2]int
	start := findStartingPoints(data)
	traversedNodes[start[0]][start[1]] = 1
	firstNode := findPossibleDir(data, traversedNodes, start[0], start[1])
	traversedNodes[firstNode[0]][firstNode[1]] = 1
	secondNode := findPossibleDir(data, traversedNodes, start[0], start[1])
	traversedNodes[secondNode[0]][secondNode[1]] = 1
	nodesToExplore = append(nodesToExplore, firstNode)
	nodesToExplore = append(nodesToExplore, secondNode)
	for {
		newNode := findPossibleDir(data, traversedNodes, nodesToExplore[0][0], nodesToExplore[0][1])
		if newNode[0] == -1 {
			return traversedNodes
		}
		traversedNodes[newNode[0]][newNode[1]] = 1
		nodesToExplore = append(nodesToExplore, newNode)
		nodesToExplore = nodesToExplore[1:]
	}
}

func main() {
	data := getFile()
	loop := findEnd(data)
	pixelCount := 0
	for i := 0; i < len(loop); i++ {
		wallCount := 0
		var insideWall byte
		for j := 0; j < len(loop[0]); j++ {
			if loop[i][j] == 1 && (data[i][j] == 'L' || data[i][j] == 'F') {
				insideWall = data[i][j]
			}
			if loop[i][j] == 1 &&
				i > 0 &&
				j+1 < len(data[i]) &&
				data[i][j] == 'S' &&
				(data[i-1][j] == '|' || data[i-1][j] == 'F' || data[i-1][j] == '7') &&
				(data[i][j+1] == '-' || data[i][j+1] == 'J' || data[i][j+1] == '7') {
				insideWall = 'L'
			}
			if loop[i][j] == 1 &&
				i+1 < len(data) &&
				j+1 < len(data[i]) &&
				data[i][j] == 'S' &&
				(data[i+1][j] == '|' || data[i+1][j] == 'J' || data[i+1][j] == 'L') &&
				(data[i][j+1] == '-' || data[i][j+1] == 'J' || data[i][j+1] == '7') {
				insideWall = 'F'
			}
			if loop[i][j] == 1 && (data[i][j] == '|' || (data[i][j] == '7' && insideWall == 'L') || (data[i][j] == 'J' && insideWall == 'F')) {
				insideWall = 0
				wallCount += 1
				continue
			}
			if wallCount%2 == 1 && loop[i][j] == 0 {
				pixelCount += 1
			}
		}
	}
	fmt.Println(pixelCount)
}
