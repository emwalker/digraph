package util

// Present returns true if the string pointed to exists and is non-empty
func Present(ptr *string) bool {
	return ptr != nil && *ptr != ""
}

func StringsToInterfaces(ids []string) []interface{} {
	var translatedIds []interface{}
	for _, id := range ids {
		translatedIds = append(translatedIds, id)
	}
	return translatedIds
}
