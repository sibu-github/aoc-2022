package main

import (
	"fmt"
	"bufio"
	"os"
	"strconv"
)

func main(){
	var highest_1 = 0;
	var highest_2 = 0;
	var highest_3 = 0;
	var sum = 0;

	file, err := os.Open("inputs.txt")
	if err != nil {
		fmt.Println("input file not found");
		return;
	}
	defer file.Close()
	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		var line = scanner.Text()
		if line == "" {
			if sum > highest_1 {
				highest_3 = highest_2
				highest_2 = highest_1
				highest_1 = sum
			} else if sum > highest_2 {
				highest_3 = highest_2
				highest_2 = sum
			} else if sum > highest_3 {
				highest_3 = sum
			}
			sum = 0
			continue
		}
		num, err := strconv.Atoi(line)
		if err != nil {
			fmt.Println("not able to convert value to int")
			return
		}
        sum += num
    }
	if sum > highest_1 {
		highest_3 = highest_2
		highest_2 = highest_1
		highest_1 = sum
	} else if sum > highest_2 {
		highest_3 = highest_2
		highest_2 = sum
	} else if sum > highest_3 {
		highest_3 = sum
	}
	fmt.Println("Highest is: ", highest_1);
	fmt.Println("Total of highest 3: ", highest_1 + highest_2 + highest_3);
}