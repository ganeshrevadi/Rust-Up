package strain

// Keep returns elements of list for which the predicate is true.
func Keep[T any](list []T, pred func(T) bool) []T {
	result := []T{}
	for _, v := range list {
		if pred(v) {
			result = append(result, v)
		}
	}
	return result
}

// Discard returns elements of list for which the predicate is false.
func Discard[T any](list []T, pred func(T) bool) []T {
	result := []T{}
	for _, v := range list {
		if !pred(v) {
			result = append(result, v)
		}
	}
	return result
}
