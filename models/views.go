package models

type View struct {
	CurrentOrganizationLogin string
	CurrentRepositoryName    *string
	RepositoryIds            []string
	ViewerID                 string
}

func (v *View) RepositoryIdsForQuery() []interface{} {
	var ids []interface{}
	for _, id := range v.RepositoryIds {
		ids = append(ids, id)
	}
	return ids
}

func (v *View) RepositoriesSelected() bool {
	return len(v.RepositoryIds) > 0
}
