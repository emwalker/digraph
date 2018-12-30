package models

import (
	"context"

	"github.com/google/uuid"
	"github.com/volatiletech/sqlboiler/boil"
	"github.com/volatiletech/sqlboiler/queries/qm"
)

// LinkValue wraps a link with additional fields that are not obtained from the database.
type LinkValue struct {
	*Link
	NewlyAdded bool
}

// TopicValue wraps a topic with additional fields that are not obtained from the database.
type TopicValue struct {
	*Topic
	NewlyAdded bool
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

// DefaultRepo returns the user's default repo.
func (u *User) DefaultRepo(ctx context.Context, exec boil.ContextExecutor) (*Repository, error) {
	// log.Printf("Looking for %s and %s (%s)", u.Login, u.ID, u.Name)
	return Repositories(
		qm.InnerJoin("organizations o on o.id = repositories.organization_id"),
		qm.Where("o.login = ? and repositories.system and repositories.owner_id = ?", u.Login, u.ID),
	).One(ctx, exec)
}

// RootTopic returns the root topic of the repository.
func (r *Repository) RootTopic(ctx context.Context, exec boil.ContextExecutor) (*TopicValue, error) {
	topic, err := r.Topics(qm.Where("root")).One(ctx, exec)
	return &TopicValue{Topic: topic}, err
}

// IsPrivate is true if the repository is a private repo.
func (r *Repository) IsPrivate() bool {
	return r.System && r.Name == "system:default"
}

// DisplayColor returns a string of the hex color to use for the topic.
func (r *Repository) DisplayColor() string {
	return "#dbedff"
}
