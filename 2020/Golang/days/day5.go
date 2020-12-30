package days

import (
	"fmt"
	"strings"
)

// Five is AOC fifth day
func Five() {
	println(fmt.Sprintf("Part 1: %d", day5Part1(GetInput(5))))
	println(fmt.Sprintf("Part 2: %d", day5Part2(GetInput(5))))
}

func day5Part1(input string) int {
	var best int
	for _, seatDetails := range strings.Split(input, "\n") {
		if len(seatDetails) == 0 {
			continue
		}
		rowLeft, rowRight := 0, 127
		for _, char := range seatDetails[0:6] {
			if char == 'F' {
				rowRight -= (rowRight - rowLeft + 1) / 2
			} else {
				rowLeft += (rowRight - rowLeft + 1) / 2
			}
		}
		if seatDetails[6] == 'B' {
			rowLeft = rowRight
		}

		colLeft, colRight := 0, 7
		for _, col := range seatDetails[7:9] {
			if col == 'L' {
				colRight -= (colRight - colLeft + 1) / 2
			} else {
				colLeft += (colRight - colLeft + 1) / 2
			}
		}

		if seatDetails[9] == 'R' {
			colLeft = colRight
		}

		id := rowLeft*8 + colLeft
		if id > best {
			best = id
		}
	}

	return best
}

func day5Part2(input string) int {
	allSeatIDs := make(map[int]bool)

	for _, seatDetails := range strings.Split(input, "\n") {
		rowLeft, rowRight := 0, 127
		for _, char := range seatDetails[0:6] {
			if char == 'F' {
				rowRight -= (rowRight - rowLeft + 1) / 2
			} else {
				rowLeft += (rowRight - rowLeft + 1) / 2
			}
		}
		if seatDetails[6] == 'B' {
			rowLeft = rowRight
		}

		colLeft, colRight := 0, 7
		for _, col := range seatDetails[7:9] {
			if col == 'L' {
				colRight -= (colRight - colLeft + 1) / 2
			} else {
				colLeft += (colRight - colLeft + 1) / 2
			}
		}

		if seatDetails[9] == 'R' {
			colLeft = colRight
		}

		id := rowLeft*8 + colLeft
		allSeatIDs[id] = true
	}

	var mySeatID int
	for id := range allSeatIDs {
		if allSeatIDs[id] && allSeatIDs[id+2] && !allSeatIDs[id+1] {
			mySeatID = id + 1
			break
		}
	}

	return mySeatID
}
