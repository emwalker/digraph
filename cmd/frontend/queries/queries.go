package queries

// Some helpful constants.
const (
	ErrSQLNoRows = "sql: no rows in result set"
)

func realError(err error) bool {
	return err != nil && err.Error() != ErrSQLNoRows
}
