package greetings

import (
	"io/ioutil"
	"net/http"
	"strings"
)

//Day2 solves day2
func Day2() {
	input := GrabInput()

	// for _, password := range input {
	// 	for i := 0; i < len(password); i++ {
	// 		println(password[i])
	// 	}

	// 	//println(password)
	// }

	for i := 0; i < len(input); i++ {
		for j := 0; j < len(input[i]); j++ {
			println(input[i][j])
		}
	}

}

// GrabInput gets input s
func GrabInput() []string {
	var sessionID = "53616c7465645f5fdba1ca082a2a18c88311d76b8744021d54793ca0fe09ce7a361fc8ab754b4ddb6d0ea24a424a76b6"
	byteSlice, err := HTTPwithCookies("https://adventofcode.com/2020/day/2/input", sessionID)
	if err != nil {
		panic(err)
	}
	input := string(byteSlice)
	stringSlice := strings.Split(input, "\n")
	return stringSlice
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
