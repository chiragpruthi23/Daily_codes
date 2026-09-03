package main

import (
	"bufio"
	"fmt"
	"os"
	"strings"
)

func isPalindrome(str string) bool {
	str = strings.ToLower(strings.ReplaceAll(str, " ", ""))
	for i := 0; i < len(str)/2; i++ {
		if str[i] != str[len(str)-1-i] {
			return false
		}
	}
	return true
}

func main() {
	reader := bufio.NewReader(os.Stdin)
	
	for {
		fmt.Println("\n=== Palindrome Checker ===")
		fmt.Println("1. Check palindrome")
		fmt.Println("2. Exit")
		fmt.Print("Choose: ")
		
		choice, _ := reader.ReadString('\n')
		choice = strings.TrimSpace(choice)
		
		if choice == "1" {
			fmt.Print("Enter text: ")
			text, _ := reader.ReadString('\n')
			text = strings.TrimSpace(text)
			
			if isPalindrome(text) {
				fmt.Println("✓ Is palindrome!")
			} else {
				fmt.Println("✗ Not palindrome!")
			}
		} else if choice == "2" {
			fmt.Println("Bye!")
			break
		}
	}
}
