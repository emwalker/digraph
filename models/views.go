package models

type View struct {
	OrganizationIds []string
}

func (v *View) OrganizationIdsForQuery() []interface{} {
	var ids []interface{}
	for _, organizationId := range v.OrganizationIds {
		ids = append(ids, organizationId)
	}
	return ids
}
