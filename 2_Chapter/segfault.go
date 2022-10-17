// Section 2.5.4 Values vs. references 

package main
import "errors"
import "fmt"

func main() {
	result, err := construct_fixed(23)
	if err !=nil{
		fmt.Println("Be a ", *result)
	}

	result, err = construct_fixed(42)
	fmt.Println("Be a ", *result) // Boom
}

func construct(n int) *string {
	if n < 42 {
		message := "Ninja Coder"
		return &message // returns the address of message
	} else {
		return nil
	}
}

func construct_fixed(n int) (*string, error) {
	if n < 42 {
		message := "Ninja Coder"
		// returns the address of message
		return &message, nil
	} else {
		return nil, errors.New("argument out of range")
	}
}
