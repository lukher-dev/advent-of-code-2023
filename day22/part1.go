package main

import (
	"bufio"
	"fmt"
	"log"
	"math"
	"os"
	"sort"
	"strconv"
	"strings"
)

type brick struct {
	start           [3]int
	end             [3]int
	axis            int
	brick           [][3]int
	supports        []int
	getsSupportedBy []int
}

func stringSliceToInt(s []string) []int {
	integers := make([]int, len(s))
	for i := range s {
		tmp, err := strconv.Atoi(s[i])
		if err != nil {
			log.Fatal(err)
		}
		integers[i] = tmp
	}
	return integers
}

func drawBrick(data [2][3]int, axis int) [][3]int {
	brick := [][3]int{}
	start := math.Min(float64(data[0][axis]), float64(data[1][axis]))
	end := math.Max(float64(data[0][axis]), float64(data[1][axis]))
	for j := start; j <= end; j++ {
		coords := data[0]
		coords[axis] = int(j)
		brick = append(brick, coords)
	}
	return brick
}

func createOccupied(data []brick) map[[3]int]int {
	occupied := make(map[[3]int]int)
	for k := 0; k < len(data); k++ {
		for j := 0; j < len(data[k].brick); j++ {
			occupied[data[k].brick[j]] = k
		}
	}
	return occupied
}

func getFile() []brick {
	file, err := os.Open("./input.txt")
	if err != nil {
		log.Fatal(err)
	}
	defer file.Close()

	scanner := bufio.NewScanner(file)
	var data []brick
	for scanner.Scan() {
		splitInput := strings.Split(scanner.Text(), "~")

		start := stringSliceToInt(strings.Split(splitInput[0], ","))
		end := stringSliceToInt(strings.Split(splitInput[1], ","))
		axis := 0
		for i := 0; i < 3; i++ {
			if start[i] != end[i] {
				axis = i
				break
			}
		}
		data = append(data, brick{start: [3]int{start[0], start[1], start[2]}, end: [3]int{end[0], end[1], end[2]}, brick: drawBrick([2][3]int{{start[0], start[1], start[2]}, {end[0], end[1], end[2]}}, axis), axis: axis})
	}
	return data
}

func makeBricksFall(data []brick, occupied map[[3]int]int) ([]brick, map[[3]int]int) {
	for i := 0; i < len(data); i++ {
		falling := 1
		collision := make(map[int]bool)
		for true {
			if math.Min(float64(data[i].start[2]-falling), float64(data[i].end[2]-falling)) < 0 {
				falling -= 1
				break
			}
			newBrick := [][3]int{}
			for j := 0; j < len(data[i].brick); j++ {
				newBrick = append(newBrick, [3]int{data[i].brick[j][0], data[i].brick[j][1], data[i].brick[j][2] - falling})
			}
			for j := 0; j < len(newBrick); j++ {
				if val, ok := occupied[newBrick[j]]; ok {
					if val == i {
						continue
					}
					collision[val] = true
				}
			}
			if len(collision) > 0 {
				falling -= 1
				break
			}
			falling += 1
		}
		for j := 0; j < len(data[i].brick); j++ {
			delete(occupied, data[i].brick[j])
		}

		isSupportedBy := make([]int, 0, len(collision))
		for value, _ := range collision {
			isSupportedBy = append(isSupportedBy, value)
			data[value] = brick{
				start:           data[value].start,
				end:             data[value].end,
				axis:            data[value].axis,
				brick:           data[value].brick,
				supports:        append(data[value].supports, i),
				getsSupportedBy: data[value].getsSupportedBy,
			}
		}

		newBrick := brick{
			start:           [3]int{data[i].start[0], data[i].start[1], data[i].start[2] - falling},
			end:             [3]int{data[i].end[0], data[i].end[1], data[i].end[2] - falling},
			brick:           drawBrick([2][3]int{{data[i].start[0], data[i].start[1], data[i].start[2] - falling}, {data[i].end[0], data[i].end[1], data[i].end[2] - falling}}, data[i].axis),
			axis:            data[i].axis,
			getsSupportedBy: isSupportedBy,
		}
		data[i] = newBrick
		for j := 0; j < len(newBrick.brick); j++ {
			occupied[newBrick.brick[j]] = i
		}
	}
	return data, occupied
}

func main() {
	data := getFile()
	sort.Slice(data, func(i, j int) bool {
		return data[i].start[2] < data[j].start[2]
	})

	occupied := createOccupied(data)
	data, occupied = makeBricksFall(data, occupied)

	result := 0
	for i := 0; i < len(data); i++ {
		if len(data[i].supports) == 0 {
			result += 1
		} else {
			count := 0
			for j := 0; j < len(data[i].supports); j++ {
				if len(data[data[i].supports[j]].getsSupportedBy) > 1 {
					count += 1
				}
			}
			if count == len(data[i].supports) {
				result += 1
			}
		}
	}
	fmt.Println(result)
}
