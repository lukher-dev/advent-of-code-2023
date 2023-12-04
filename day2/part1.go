package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"strconv"
	"strings"
)

type round struct {
	red   int
	green int
	blue  int
}

type game struct {
	gameNumber int
	rounds     []round
}

func contains(s []string, e string) string {
	for _, a := range s {
		split := strings.Split(a, e)
		if len(split) > 1 {
			return split[0]
		}
	}
	return "0"
}

func newRound(line string) round {
	rounds := strings.Split(line, ",")
	blue, _ := strconv.Atoi(strings.Trim(contains(rounds, "blue"), " "))
	red, _ := strconv.Atoi(strings.Trim(contains(rounds, "red"), " "))
	green, _ := strconv.Atoi(strings.Trim(contains(rounds, "green"), " "))

	fmt.Println("rounds", rounds)
	fmt.Println("blue: ", blue)
	fmt.Println("red: ", red)
	fmt.Println("green: ", green)
	return round{red: red, blue: blue, green: green}
}

func newRounds(line string) []round {
	rounds := strings.Split(line, ";")

	var list []round
	for _, x := range rounds {
		list = append(list, newRound(x))
	}

	return list
}

func newGame(line string) game {
	res1 := strings.Split(line, ":")
	res, _ := strconv.Atoi(strings.Replace(res1[0], "Game ", "", -1))
	rounds := newRounds(res1[1])
	p := game{gameNumber: res, rounds: rounds}
	fmt.Printf("%+v\n", p)
	return p
}

func chackIfGameisPossible(game game) bool {
	for _, x := range game.rounds {
		if x.blue > 14 || x.green > 13 || x.red > 12 {
			return false
		}
	}
	return true
}

func main() {
	file, err := os.Open("./input.txt")
	if err != nil {
		log.Fatal(err)
	}
	defer file.Close()

	scanner := bufio.NewScanner(file)

	result := 0
	var games []game
	for scanner.Scan() {
		game := newGame(scanner.Text())
		games = append(games, game)
		if chackIfGameisPossible(game) {
			result += game.gameNumber
		}
		fmt.Println(scanner.Text())
	}

	fmt.Println("Result: ", result)

	if err := scanner.Err(); err != nil {
		log.Fatal(err)
	}
}
