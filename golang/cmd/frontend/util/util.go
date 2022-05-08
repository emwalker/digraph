package util

// Present returns true if the string pointed to exists and is non-empty
func Present(ptr *string) bool {
	return ptr != nil && *ptr != ""
}
