package queries

// Some helpful constants.
const (
	ErrSQLNoRows = "sql: no rows in result set"
)

func isRealError(err error) bool {
	return err != nil && err.Error() != ErrSQLNoRows
}
