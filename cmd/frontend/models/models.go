package models

import (
	"context"
	"encoding/hex"
	"fmt"
	"log"

	"github.com/google/uuid"
	"github.com/volatiletech/sqlboiler/v4/boil"
	"github.com/volatiletech/sqlboiler/v4/queries/qm"
)

// LinkValue wraps a link with additional fields that are not obtained from the database.
type LinkValue struct {
	*Link
	NewlyAdded bool
	View       *View
}

// TopicValue wraps a topic with additional fields that are not obtained from the database.
type TopicValue struct {
	*Topic
	NewlyAdded bool
	View       *View
}

// Synonym holds a single synonym.
type Synonym struct {
	Locale string
	Name   string
}

type synonymLookup struct {
	locales map[LocaleIdentifier]Synonym
}

// SynonymList holds the set of synonyms for a topic.
type SynonymList struct {
	Values []Synonym
	lookup *synonymLookup
}

func newLookup(s *SynonymList) *synonymLookup {
	locales := map[LocaleIdentifier]Synonym{}

	for _, synonym := range s.Values {
		locale := synonym.LocaleIdentifier()

		if !locale.IsValid() {
			log.Printf("Invalid locale: %s", locale)
			continue
		}

		if _, ok := locales[locale]; !ok {
			locales[locale] = synonym
		}
	}

	lookup := synonymLookup{locales: locales}
	return &lookup
}

func (l *synonymLookup) nameInLocale(locale LocaleIdentifier) (Synonym, bool) {
	synonym, ok := l.locales[locale]
	if ok {
		return synonym, ok
	}
	return Synonym{}, false
}

// LocaleIdentifier returns the locale of the synonym.
func (s *Synonym) LocaleIdentifier() LocaleIdentifier {
	return LocaleIdentifier(s.Locale)
}

// IsNamespaceable tags Link as implementing the Namespaceable interface.
func (Link) IsNamespaceable() {}

// IsResourceIdentifiable tags Link as implementing the ResourceIdentifiable interface.
func (Link) IsResourceIdentifiable() {}

// IsSearchResultItem tags Link as being in the SearchResultItem union.
func (Link) IsSearchResultItem() {}

// String returns info on a user that can be printed to the log
func (l Link) String() string {
	return fmt.Sprintf("link %s (%s)", l.URL, l.ID)
}

// IsResourceIdentifiable tags Repository as implementing the ResourceIdentifiable interface.
func (Repository) IsResourceIdentifiable() {}

// String returns a string that can be included in logs.
func (r Repository) String() string {
	return fmt.Sprintf("repo %s (%s)", r.Name, r.ID)
}

// IsNamespaceable tags Topic as implementing the Namespaceable interface.
func (Topic) IsNamespaceable() {}

// IsResourceIdentifiable tags Topic as implementing the ResourceIdentifiable interface.
func (Topic) IsResourceIdentifiable() {}

// IsSearchResultItem tags Topic as being in the SearchResultItem union.
func (Topic) IsSearchResultItem() {}

// String returns info on a user that can be printed to the log.
func (t Topic) String() string {
	return fmt.Sprintf("topic %s (%s)", t.Name, t.ID)
}

// SynonymList returns the synonyms json field.
func (t Topic) SynonymList() (*SynonymList, error) {
	var synonyms SynonymList
	if err := t.Synonyms.Unmarshal(&synonyms.Values); err != nil {
		return nil, err
	}
	return &synonyms, nil
}

// NameForLocale chooses a suitable topic name from among the available synonyms in light of
// the current locale.
func (s *SynonymList) NameForLocale(locale LocaleIdentifier) (string, bool) {
	if s.lookup == nil {
		// compute indexes
		s.lookup = newLookup(s)
	}

	synonym, ok := s.lookup.nameInLocale(locale)
	if ok {
		return synonym.Name, true
	}

	synonym, ok = s.lookup.nameInLocale(LocaleIdentifierEn)
	if ok {
		return synonym.Name, true
	}

	if len(s.Values) > 0 {
		return s.Values[0].Name, true
	}

	return "<missing name>", false
}

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
func (r *Repository) RootTopic(
	ctx context.Context, exec boil.ContextExecutor, view *View,
) (*TopicValue, error) {
	topic, err := r.Topics(qm.Where("root")).One(ctx, exec)
	return &TopicValue{Topic: topic, View: view}, err
}

// IsPrivate is true if the repository is a private repo.
func (r *Repository) IsPrivate() bool {
	return r.System && r.Name == "system:default"
}

// DisplayColor returns a string of the hex color to use for the topic.
func (r *Repository) DisplayColor() string {
	return "#dbedff"
}

// HexID provides a hex-encoded version of the session id that can be used for things like
// setting cookies.
func (s Session) HexID() string {
	return hex.EncodeToString(s.SessionID)
}

// DisplayName returns the name to be used for the user in case the Name field is not set.
func (u User) DisplayName() string {
	switch {
	case u.Name != "":
		return u.Name
	case u.Login.Valid:
		return u.Login.String
	default:
		return "<missing name>"
	}
}

// IsGuest returns true if the user is not backed by a row in the database.
func (u User) IsGuest() bool {
	return u.ID == ""
}

// String returns info on a user that can be printed to the log
func (u User) String() string {
	if u.PrimaryEmail == "" {
		return fmt.Sprintf("user %s", u.DisplayName())
	}
	return fmt.Sprintf("user %s (%s)", u.DisplayName(), u.PrimaryEmail)
}

// DefaultView returns a view that can be used in return values for mutations and similar situations
func (u User) DefaultView() *View {
	return &View{ViewerID: u.ID}
}

// Filter filters a query according to the repos that have been selected and that the user can
// access.
func (v View) Filter(mods []qm.QueryMod) []qm.QueryMod {
	if v.ViewerID == "" {
		log.Print("No viewer id provided, restricting results to the general repo")
		return append(mods,
			qm.InnerJoin("organizations o on o.id = r.organization_id"),
			qm.Where("r.system and o.public"),
		)
	}

	if len(v.RepositoryIds) > 0 {
		ids := v.RepositoryIdsForQuery()
		return append(mods, qm.WhereIn("r.id in ?", ids...))
	}

	return append(mods,
		qm.InnerJoin("organization_members om on r.organization_id = om.organization_id"),
		qm.WhereIn("om.user_id = ? ", v.ViewerID),
	)
}
