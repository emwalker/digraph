package queries

// Some helpful constants.
const (
	ErrSQLNoRows = "sql: no rows in result set"
)

// IsRealError returns true if err is non-nil and is not a "sql: no rows in result set" error
func IsRealError(err error) bool {
	return err != nil && err.Error() != ErrSQLNoRows
}
