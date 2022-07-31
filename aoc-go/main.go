package main

import (
	"fmt"
	"log"
	"strconv"
	"strings"
)

func getInput() string {
	return `forward 5
down 5
forward 8
up 3
down 8
forward 2`
}

type dict = [2]int

func parseLine(line string) dict {

	instructions := strings.Split(line, " ")
	val, err     := strconv.Atoi(instructions[1])
	direction 	 := instructions[0]
	
	if err != nil {
		log.Fatal("There is a huge blunder")
	}

	switch direction {

		case "forward":
			return dict{val, 0}

		case "up":
			return dict{0, -val}

		default:
			return dict{0, val}

	}
}

func main() {

	lines := dict{0, 0}

	for _, line := range strings.Split(getInput(), "\n") {
		parsed := parseLine(line)
		lines[0] += parsed[0]
		lines[1] += parsed[1]
	}

	fmt.Printf("%+v, %d\n", lines, lines[0] * lines[1])
}
