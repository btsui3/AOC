package days

import (
	"fmt"
	"strings"
)

// Three : advent of code, day three part1 and 2.
func Three() {
	input := GetInput(3)
	inputSlice := strings.Split(input, "\n")
	slopes := [5][2]int{
		{1, 1},
		{3, 1}, // Part 1
		{5, 1},
		{7, 1},
		{1, 2},
	}

	trees := []int{}

	for slope := 0; slope < len(slopes); slope++ {
		right := slopes[slope][0]
		down := slopes[slope][1]
		treesAtSlope := treesInSlope(inputSlice, right, down)
		trees = append(trees, treesAtSlope)
	}

	treesMultiplied := 1
	for tree := 0; tree < len(trees); tree++ {
		treesMultiplied = treesMultiplied * trees[tree]
	}

	fmt.Print("Trees Part 1: ")
	fmt.Println(trees[1])
	fmt.Print("Trees Part 2: ")
	fmt.Println(treesMultiplied)
}

func treesInSlope(grid []string, right int, down int) int {
	trees := 0
	row := 0
	column := 0

	for row < len(grid) {

		var runeAt = grid[row][column]

		if runeAt == '#' {
			trees++
		}

		row = row + down
		column = (column + right) % len(grid[0]) // Pattern Repeats -> Use modulus
	}

	return trees
}
