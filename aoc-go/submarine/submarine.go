package submarine

import (
	"fmt"
	"log"
	"strconv"
	"strings"
)

type Point struct {
	x int
	y int
}

func getInput() string {
	return "forward 5\ndown 5\nforward 8\nup 3\ndown 8\nforward 2"
}

func parseLine(line string) Point {

	instructions := strings.Split(line, " ")
	val, err := strconv.Atoi(instructions[1])
	direction := instructions[0]

	if err != nil {
		log.Fatal("blunder")
	}

	switch direction {

	case "forward":
		return Point{x: val, y: 0}

	case "up":
		return Point{x: 0, y: -val}

	default:
		return Point{x: 0, y: val}

	}
}

func Submarine() {

	lines := Point{0, 0}

	for _, line := range strings.Split(getInput(), "\n") {
		parsed := parseLine(line)
		lines.x += parsed.x
		lines.y += parsed.y
	}

	fmt.Printf("%+v, %d\n", lines, lines.x*lines.y)
}
