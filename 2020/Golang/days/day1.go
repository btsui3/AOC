package days

import (
	"io/ioutil"
	"net/http"
	"strconv"
	"strings"
)

// GetExpenseReportProduct takes no input and returns an int
func GetExpenseReportProduct() int {
	var sessionID = "53616c7465645f5fdba1ca082a2a18c88311d76b8744021d54793ca0fe09ce7a361fc8ab754b4ddb6d0ea24a424a76b6"
	byteSlice, err := HTTPwithCookies("https://adventofcode.com/2020/day/1/input", sessionID)
	if err != nil {
		panic(err)
	}
	input := string(byteSlice)
	stringSlice := strings.Split(input, "\n")
	var intSlice = []int{}

	for _, numStr := range stringSlice {

		if numStr != "" {
			numInt, err := strconv.Atoi(numStr)
			if err != nil {
				panic(err)
			}
			intSlice = append(intSlice, numInt)
		}

	}
	// Part 1: Hash Map
	//TwoProducts(intSlice)

	// Part 2: Brute Force
	cond0, ret0 := ThreeProducts(intSlice)
	if cond0 {
		return ret0
	}
	return 0
}

func ThreeProducts(intSlice []int) (bool, int) {
	target := 2020
	for i := 0; i < len(intSlice); i++ {
		for j := i + 1; j < len(intSlice); j++ {
			for k := j + 1; k < len(intSlice); k++ {
				if target == intSlice[i]+intSlice[j]+intSlice[k] {
					println(intSlice[i] * intSlice[j] * intSlice[k])
					return true, intSlice[i] * intSlice[j] * intSlice[k]
				}
			}
		}
	}
	return false, 0
}

func TwoProducts(intSlice []int) {
	dict := map[int]int{}
	target := 2020
	for i := 0; i < len(intSlice); i++ {
		if val, ok := dict[intSlice[i]]; ok && i != val {
			result := intSlice[i] * intSlice[dict[intSlice[i]]]
			println(result)
		}
		dict[target-intSlice[i]] = i

	}
}

// HTTPwithCookies makes a request with a session cookie
func HTTPwithCookies(url string, sessionid string) (b []byte, err error) {
	req, err := http.NewRequest("GET", url, nil)
	if err != nil {
		return
	}

	req.AddCookie(&http.Cookie{Name: "session", Value: sessionid})
	client := &http.Client{}
	resp, err := client.Do(req)
	if err != nil {
		return
	}
	defer resp.Body.Close()

	return ioutil.ReadAll(resp.Body)
}
