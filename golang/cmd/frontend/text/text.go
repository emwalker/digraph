package text

import (
	"strings"
)

// Squash removes extra whitespace from a string.
func Squash(str string) string {
	return strings.Join(strings.Fields(str), " ")
}
