package models

type View struct {
	ViewerID      string
	RepositoryIds []string
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
