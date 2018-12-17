package models

import (
	"context"

	"github.com/google/uuid"
	"github.com/volatiletech/sqlboiler/boil"
	"github.com/volatiletech/sqlboiler/queries/qm"
)

type TopicValue struct {
	*Topic
	View *View
}

// IsNamespaceable tags Link as implementing the Namespaceable interface.
func (Link) IsNamespaceable() {}

// IsResourceIdentifiable tags Link as implementing the ResourceIdentifiable interface.
func (Link) IsResourceIdentifiable() {}

// IsSearchResultItem tags Link as being in the SearchResultItem union.
func (Link) IsSearchResultItem() {}

// IsResourceIdentifiable tags Repository as implementing the ResourceIdentifiable interface.
func (Repository) IsResourceIdentifiable() {}

// IsNamespaceable tags Topic as implementing the Namespaceable interface.
func (Topic) IsNamespaceable() {}

// IsResourceIdentifiable tags Topic as implementing the ResourceIdentifiable interface.
func (Topic) IsResourceIdentifiable() {}

// IsSearchResultItem tags Topic as being in the SearchResultItem union.
func (Topic) IsSearchResultItem() {}

// IsResourceIdentifiable tags Organization as implementing the ResourceIdentifiable interface.
func (Organization) IsResourceIdentifiable() {}

// NewAlert returns an initialized alert.
func NewAlert(typ AlertType, text string) *Alert {
	return &Alert{
		ID:   uuid.New().String(),
		Text: text,
		Type: typ,
	}
}

func (u *User) DefaultRepo(ctx context.Context, exec boil.ContextExecutor) (*Repository, error) {
	// log.Printf("Looking for %s and %s (%s)", u.Login, u.ID, u.Name)
	return Repositories(
		qm.Load("Organization"),
		qm.InnerJoin("organizations o on o.id = repositories.organization_id"),
		qm.Where("o.login = ? and repositories.system and repositories.owner_id = ?", u.Login, u.ID),
	).One(ctx, exec)
}

func (r *Repository) RootTopic(ctx context.Context, exec boil.ContextExecutor) (*TopicValue, error) {
	topic, err := r.Topics(qm.Where("root")).One(ctx, exec)
	return &TopicValue{Topic: topic}, err
}

func (r *Repository) IsPrivate() bool {
	return r.System && r.Name == "system:default"
}

func (t *TopicValue) DisplayColor() string {
	return "#dbedff"
}
