package models

// View holds config information for what is seen in a given query.
type View struct {
	CurrentOrganizationLogin string
	CurrentRepositoryName    *string
	CurrentRepository        *Repository
	RepositoryIds            []string
	ViewerID                 string
}

// RepositoryIdsForQuery is a helper method that converts respository ids on the view into something
// that can be used by sqlboiler to query the database.
func (v *View) RepositoryIdsForQuery() []interface{} {
	var ids []interface{}
	for _, id := range v.RepositoryIds {
		ids = append(ids, id)
	}
	return ids
}

// RepositoriesSelected return true if there is at least one selected repository in the view.
func (v *View) RepositoriesSelected() bool {
	return len(v.RepositoryIds) > 0
}
