// Code generated by SQLBoiler 3.7.1 (https://github.com/volatiletech/sqlboiler). DO NOT EDIT.
// This file is meant to be re-generated in place and/or deleted at any time.

package models

import (
	"context"
	"database/sql"
	"fmt"
	"reflect"
	"strconv"
	"strings"
	"sync"
	"time"

	"github.com/friendsofgo/errors"
	"github.com/volatiletech/sqlboiler/boil"
	"github.com/volatiletech/sqlboiler/queries"
	"github.com/volatiletech/sqlboiler/queries/qm"
	"github.com/volatiletech/sqlboiler/queries/qmhelper"
	"github.com/volatiletech/sqlboiler/strmangle"
)

// UserLink is an object representing the database table.
type UserLink struct {
	ID             string    `boil:"id" json:"id" toml:"id" yaml:"id"`
	OrganizationID string    `boil:"organization_id" json:"organization_id" toml:"organization_id" yaml:"organization_id"`
	RepositoryID   string    `boil:"repository_id" json:"repository_id" toml:"repository_id" yaml:"repository_id"`
	UserID         string    `boil:"user_id" json:"user_id" toml:"user_id" yaml:"user_id"`
	LinkID         string    `boil:"link_id" json:"link_id" toml:"link_id" yaml:"link_id"`
	CreatedAt      time.Time `boil:"created_at" json:"created_at" toml:"created_at" yaml:"created_at"`
	Action         string    `boil:"action" json:"action" toml:"action" yaml:"action"`

	R *userLinkR `boil:"-" json:"-" toml:"-" yaml:"-"`
	L userLinkL  `boil:"-" json:"-" toml:"-" yaml:"-"`
}

var UserLinkColumns = struct {
	ID             string
	OrganizationID string
	RepositoryID   string
	UserID         string
	LinkID         string
	CreatedAt      string
	Action         string
}{
	ID:             "id",
	OrganizationID: "organization_id",
	RepositoryID:   "repository_id",
	UserID:         "user_id",
	LinkID:         "link_id",
	CreatedAt:      "created_at",
	Action:         "action",
}

// Generated where

var UserLinkWhere = struct {
	ID             whereHelperstring
	OrganizationID whereHelperstring
	RepositoryID   whereHelperstring
	UserID         whereHelperstring
	LinkID         whereHelperstring
	CreatedAt      whereHelpertime_Time
	Action         whereHelperstring
}{
	ID:             whereHelperstring{field: "\"user_links\".\"id\""},
	OrganizationID: whereHelperstring{field: "\"user_links\".\"organization_id\""},
	RepositoryID:   whereHelperstring{field: "\"user_links\".\"repository_id\""},
	UserID:         whereHelperstring{field: "\"user_links\".\"user_id\""},
	LinkID:         whereHelperstring{field: "\"user_links\".\"link_id\""},
	CreatedAt:      whereHelpertime_Time{field: "\"user_links\".\"created_at\""},
	Action:         whereHelperstring{field: "\"user_links\".\"action\""},
}

// UserLinkRels is where relationship names are stored.
var UserLinkRels = struct {
	Link           string
	Organization   string
	Repository     string
	User           string
	UserLinkTopics string
}{
	Link:           "Link",
	Organization:   "Organization",
	Repository:     "Repository",
	User:           "User",
	UserLinkTopics: "UserLinkTopics",
}

// userLinkR is where relationships are stored.
type userLinkR struct {
	Link           *Link
	Organization   *Organization
	Repository     *Repository
	User           *User
	UserLinkTopics UserLinkTopicSlice
}

// NewStruct creates a new relationship struct
func (*userLinkR) NewStruct() *userLinkR {
	return &userLinkR{}
}

// userLinkL is where Load methods for each relationship are stored.
type userLinkL struct{}

var (
	userLinkAllColumns            = []string{"id", "organization_id", "repository_id", "user_id", "link_id", "created_at", "action"}
	userLinkColumnsWithoutDefault = []string{"organization_id", "repository_id", "user_id", "link_id", "action"}
	userLinkColumnsWithDefault    = []string{"id", "created_at"}
	userLinkPrimaryKeyColumns     = []string{"id"}
)

type (
	// UserLinkSlice is an alias for a slice of pointers to UserLink.
	// This should generally be used opposed to []UserLink.
	UserLinkSlice []*UserLink

	userLinkQuery struct {
		*queries.Query
	}
)

// Cache for insert, update and upsert
var (
	userLinkType                 = reflect.TypeOf(&UserLink{})
	userLinkMapping              = queries.MakeStructMapping(userLinkType)
	userLinkPrimaryKeyMapping, _ = queries.BindMapping(userLinkType, userLinkMapping, userLinkPrimaryKeyColumns)
	userLinkInsertCacheMut       sync.RWMutex
	userLinkInsertCache          = make(map[string]insertCache)
	userLinkUpdateCacheMut       sync.RWMutex
	userLinkUpdateCache          = make(map[string]updateCache)
	userLinkUpsertCacheMut       sync.RWMutex
	userLinkUpsertCache          = make(map[string]insertCache)
)

var (
	// Force time package dependency for automated UpdatedAt/CreatedAt.
	_ = time.Second
	// Force qmhelper dependency for where clause generation (which doesn't
	// always happen)
	_ = qmhelper.Where
)

// One returns a single userLink record from the query.
func (q userLinkQuery) One(ctx context.Context, exec boil.ContextExecutor) (*UserLink, error) {
	o := &UserLink{}

	queries.SetLimit(q.Query, 1)

	err := q.Bind(ctx, exec, o)
	if err != nil {
		if errors.Cause(err) == sql.ErrNoRows {
			return nil, sql.ErrNoRows
		}
		return nil, errors.Wrap(err, "models: failed to execute a one query for user_links")
	}

	return o, nil
}

// All returns all UserLink records from the query.
func (q userLinkQuery) All(ctx context.Context, exec boil.ContextExecutor) (UserLinkSlice, error) {
	var o []*UserLink

	err := q.Bind(ctx, exec, &o)
	if err != nil {
		return nil, errors.Wrap(err, "models: failed to assign all query results to UserLink slice")
	}

	return o, nil
}

// Count returns the count of all UserLink records in the query.
func (q userLinkQuery) Count(ctx context.Context, exec boil.ContextExecutor) (int64, error) {
	var count int64

	queries.SetSelect(q.Query, nil)
	queries.SetCount(q.Query)

	err := q.Query.QueryRowContext(ctx, exec).Scan(&count)
	if err != nil {
		return 0, errors.Wrap(err, "models: failed to count user_links rows")
	}

	return count, nil
}

// Exists checks if the row exists in the table.
func (q userLinkQuery) Exists(ctx context.Context, exec boil.ContextExecutor) (bool, error) {
	var count int64

	queries.SetSelect(q.Query, nil)
	queries.SetCount(q.Query)
	queries.SetLimit(q.Query, 1)

	err := q.Query.QueryRowContext(ctx, exec).Scan(&count)
	if err != nil {
		return false, errors.Wrap(err, "models: failed to check if user_links exists")
	}

	return count > 0, nil
}

// Link pointed to by the foreign key.
func (o *UserLink) Link(mods ...qm.QueryMod) linkQuery {
	queryMods := []qm.QueryMod{
		qm.Where("\"id\" = ?", o.LinkID),
	}

	queryMods = append(queryMods, mods...)

	query := Links(queryMods...)
	queries.SetFrom(query.Query, "\"links\"")

	return query
}

// Organization pointed to by the foreign key.
func (o *UserLink) Organization(mods ...qm.QueryMod) organizationQuery {
	queryMods := []qm.QueryMod{
		qm.Where("\"id\" = ?", o.OrganizationID),
	}

	queryMods = append(queryMods, mods...)

	query := Organizations(queryMods...)
	queries.SetFrom(query.Query, "\"organizations\"")

	return query
}

// Repository pointed to by the foreign key.
func (o *UserLink) Repository(mods ...qm.QueryMod) repositoryQuery {
	queryMods := []qm.QueryMod{
		qm.Where("\"id\" = ?", o.RepositoryID),
	}

	queryMods = append(queryMods, mods...)

	query := Repositories(queryMods...)
	queries.SetFrom(query.Query, "\"repositories\"")

	return query
}

// User pointed to by the foreign key.
func (o *UserLink) User(mods ...qm.QueryMod) userQuery {
	queryMods := []qm.QueryMod{
		qm.Where("\"id\" = ?", o.UserID),
	}

	queryMods = append(queryMods, mods...)

	query := Users(queryMods...)
	queries.SetFrom(query.Query, "\"users\"")

	return query
}

// UserLinkTopics retrieves all the user_link_topic's UserLinkTopics with an executor.
func (o *UserLink) UserLinkTopics(mods ...qm.QueryMod) userLinkTopicQuery {
	var queryMods []qm.QueryMod
	if len(mods) != 0 {
		queryMods = append(queryMods, mods...)
	}

	queryMods = append(queryMods,
		qm.Where("\"user_link_topics\".\"user_link_id\"=?", o.ID),
	)

	query := UserLinkTopics(queryMods...)
	queries.SetFrom(query.Query, "\"user_link_topics\"")

	if len(queries.GetSelect(query.Query)) == 0 {
		queries.SetSelect(query.Query, []string{"\"user_link_topics\".*"})
	}

	return query
}

// LoadLink allows an eager lookup of values, cached into the
// loaded structs of the objects. This is for an N-1 relationship.
func (userLinkL) LoadLink(ctx context.Context, e boil.ContextExecutor, singular bool, maybeUserLink interface{}, mods queries.Applicator) error {
	var slice []*UserLink
	var object *UserLink

	if singular {
		object = maybeUserLink.(*UserLink)
	} else {
		slice = *maybeUserLink.(*[]*UserLink)
	}

	args := make([]interface{}, 0, 1)
	if singular {
		if object.R == nil {
			object.R = &userLinkR{}
		}
		args = append(args, object.LinkID)

	} else {
	Outer:
		for _, obj := range slice {
			if obj.R == nil {
				obj.R = &userLinkR{}
			}

			for _, a := range args {
				if a == obj.LinkID {
					continue Outer
				}
			}

			args = append(args, obj.LinkID)

		}
	}

	if len(args) == 0 {
		return nil
	}

	query := NewQuery(qm.From(`links`), qm.WhereIn(`links.id in ?`, args...))
	if mods != nil {
		mods.Apply(query)
	}

	results, err := query.QueryContext(ctx, e)
	if err != nil {
		return errors.Wrap(err, "failed to eager load Link")
	}

	var resultSlice []*Link
	if err = queries.Bind(results, &resultSlice); err != nil {
		return errors.Wrap(err, "failed to bind eager loaded slice Link")
	}

	if err = results.Close(); err != nil {
		return errors.Wrap(err, "failed to close results of eager load for links")
	}
	if err = results.Err(); err != nil {
		return errors.Wrap(err, "error occurred during iteration of eager loaded relations for links")
	}

	if len(resultSlice) == 0 {
		return nil
	}

	if singular {
		foreign := resultSlice[0]
		object.R.Link = foreign
		if foreign.R == nil {
			foreign.R = &linkR{}
		}
		foreign.R.UserLinks = append(foreign.R.UserLinks, object)
		return nil
	}

	for _, local := range slice {
		for _, foreign := range resultSlice {
			if local.LinkID == foreign.ID {
				local.R.Link = foreign
				if foreign.R == nil {
					foreign.R = &linkR{}
				}
				foreign.R.UserLinks = append(foreign.R.UserLinks, local)
				break
			}
		}
	}

	return nil
}

// LoadOrganization allows an eager lookup of values, cached into the
// loaded structs of the objects. This is for an N-1 relationship.
func (userLinkL) LoadOrganization(ctx context.Context, e boil.ContextExecutor, singular bool, maybeUserLink interface{}, mods queries.Applicator) error {
	var slice []*UserLink
	var object *UserLink

	if singular {
		object = maybeUserLink.(*UserLink)
	} else {
		slice = *maybeUserLink.(*[]*UserLink)
	}

	args := make([]interface{}, 0, 1)
	if singular {
		if object.R == nil {
			object.R = &userLinkR{}
		}
		args = append(args, object.OrganizationID)

	} else {
	Outer:
		for _, obj := range slice {
			if obj.R == nil {
				obj.R = &userLinkR{}
			}

			for _, a := range args {
				if a == obj.OrganizationID {
					continue Outer
				}
			}

			args = append(args, obj.OrganizationID)

		}
	}

	if len(args) == 0 {
		return nil
	}

	query := NewQuery(qm.From(`organizations`), qm.WhereIn(`organizations.id in ?`, args...))
	if mods != nil {
		mods.Apply(query)
	}

	results, err := query.QueryContext(ctx, e)
	if err != nil {
		return errors.Wrap(err, "failed to eager load Organization")
	}

	var resultSlice []*Organization
	if err = queries.Bind(results, &resultSlice); err != nil {
		return errors.Wrap(err, "failed to bind eager loaded slice Organization")
	}

	if err = results.Close(); err != nil {
		return errors.Wrap(err, "failed to close results of eager load for organizations")
	}
	if err = results.Err(); err != nil {
		return errors.Wrap(err, "error occurred during iteration of eager loaded relations for organizations")
	}

	if len(resultSlice) == 0 {
		return nil
	}

	if singular {
		foreign := resultSlice[0]
		object.R.Organization = foreign
		if foreign.R == nil {
			foreign.R = &organizationR{}
		}
		foreign.R.UserLinks = append(foreign.R.UserLinks, object)
		return nil
	}

	for _, local := range slice {
		for _, foreign := range resultSlice {
			if local.OrganizationID == foreign.ID {
				local.R.Organization = foreign
				if foreign.R == nil {
					foreign.R = &organizationR{}
				}
				foreign.R.UserLinks = append(foreign.R.UserLinks, local)
				break
			}
		}
	}

	return nil
}

// LoadRepository allows an eager lookup of values, cached into the
// loaded structs of the objects. This is for an N-1 relationship.
func (userLinkL) LoadRepository(ctx context.Context, e boil.ContextExecutor, singular bool, maybeUserLink interface{}, mods queries.Applicator) error {
	var slice []*UserLink
	var object *UserLink

	if singular {
		object = maybeUserLink.(*UserLink)
	} else {
		slice = *maybeUserLink.(*[]*UserLink)
	}

	args := make([]interface{}, 0, 1)
	if singular {
		if object.R == nil {
			object.R = &userLinkR{}
		}
		args = append(args, object.RepositoryID)

	} else {
	Outer:
		for _, obj := range slice {
			if obj.R == nil {
				obj.R = &userLinkR{}
			}

			for _, a := range args {
				if a == obj.RepositoryID {
					continue Outer
				}
			}

			args = append(args, obj.RepositoryID)

		}
	}

	if len(args) == 0 {
		return nil
	}

	query := NewQuery(qm.From(`repositories`), qm.WhereIn(`repositories.id in ?`, args...))
	if mods != nil {
		mods.Apply(query)
	}

	results, err := query.QueryContext(ctx, e)
	if err != nil {
		return errors.Wrap(err, "failed to eager load Repository")
	}

	var resultSlice []*Repository
	if err = queries.Bind(results, &resultSlice); err != nil {
		return errors.Wrap(err, "failed to bind eager loaded slice Repository")
	}

	if err = results.Close(); err != nil {
		return errors.Wrap(err, "failed to close results of eager load for repositories")
	}
	if err = results.Err(); err != nil {
		return errors.Wrap(err, "error occurred during iteration of eager loaded relations for repositories")
	}

	if len(resultSlice) == 0 {
		return nil
	}

	if singular {
		foreign := resultSlice[0]
		object.R.Repository = foreign
		if foreign.R == nil {
			foreign.R = &repositoryR{}
		}
		foreign.R.UserLinks = append(foreign.R.UserLinks, object)
		return nil
	}

	for _, local := range slice {
		for _, foreign := range resultSlice {
			if local.RepositoryID == foreign.ID {
				local.R.Repository = foreign
				if foreign.R == nil {
					foreign.R = &repositoryR{}
				}
				foreign.R.UserLinks = append(foreign.R.UserLinks, local)
				break
			}
		}
	}

	return nil
}

// LoadUser allows an eager lookup of values, cached into the
// loaded structs of the objects. This is for an N-1 relationship.
func (userLinkL) LoadUser(ctx context.Context, e boil.ContextExecutor, singular bool, maybeUserLink interface{}, mods queries.Applicator) error {
	var slice []*UserLink
	var object *UserLink

	if singular {
		object = maybeUserLink.(*UserLink)
	} else {
		slice = *maybeUserLink.(*[]*UserLink)
	}

	args := make([]interface{}, 0, 1)
	if singular {
		if object.R == nil {
			object.R = &userLinkR{}
		}
		args = append(args, object.UserID)

	} else {
	Outer:
		for _, obj := range slice {
			if obj.R == nil {
				obj.R = &userLinkR{}
			}

			for _, a := range args {
				if a == obj.UserID {
					continue Outer
				}
			}

			args = append(args, obj.UserID)

		}
	}

	if len(args) == 0 {
		return nil
	}

	query := NewQuery(qm.From(`users`), qm.WhereIn(`users.id in ?`, args...))
	if mods != nil {
		mods.Apply(query)
	}

	results, err := query.QueryContext(ctx, e)
	if err != nil {
		return errors.Wrap(err, "failed to eager load User")
	}

	var resultSlice []*User
	if err = queries.Bind(results, &resultSlice); err != nil {
		return errors.Wrap(err, "failed to bind eager loaded slice User")
	}

	if err = results.Close(); err != nil {
		return errors.Wrap(err, "failed to close results of eager load for users")
	}
	if err = results.Err(); err != nil {
		return errors.Wrap(err, "error occurred during iteration of eager loaded relations for users")
	}

	if len(resultSlice) == 0 {
		return nil
	}

	if singular {
		foreign := resultSlice[0]
		object.R.User = foreign
		if foreign.R == nil {
			foreign.R = &userR{}
		}
		foreign.R.UserLinks = append(foreign.R.UserLinks, object)
		return nil
	}

	for _, local := range slice {
		for _, foreign := range resultSlice {
			if local.UserID == foreign.ID {
				local.R.User = foreign
				if foreign.R == nil {
					foreign.R = &userR{}
				}
				foreign.R.UserLinks = append(foreign.R.UserLinks, local)
				break
			}
		}
	}

	return nil
}

// LoadUserLinkTopics allows an eager lookup of values, cached into the
// loaded structs of the objects. This is for a 1-M or N-M relationship.
func (userLinkL) LoadUserLinkTopics(ctx context.Context, e boil.ContextExecutor, singular bool, maybeUserLink interface{}, mods queries.Applicator) error {
	var slice []*UserLink
	var object *UserLink

	if singular {
		object = maybeUserLink.(*UserLink)
	} else {
		slice = *maybeUserLink.(*[]*UserLink)
	}

	args := make([]interface{}, 0, 1)
	if singular {
		if object.R == nil {
			object.R = &userLinkR{}
		}
		args = append(args, object.ID)
	} else {
	Outer:
		for _, obj := range slice {
			if obj.R == nil {
				obj.R = &userLinkR{}
			}

			for _, a := range args {
				if a == obj.ID {
					continue Outer
				}
			}

			args = append(args, obj.ID)
		}
	}

	if len(args) == 0 {
		return nil
	}

	query := NewQuery(qm.From(`user_link_topics`), qm.WhereIn(`user_link_topics.user_link_id in ?`, args...))
	if mods != nil {
		mods.Apply(query)
	}

	results, err := query.QueryContext(ctx, e)
	if err != nil {
		return errors.Wrap(err, "failed to eager load user_link_topics")
	}

	var resultSlice []*UserLinkTopic
	if err = queries.Bind(results, &resultSlice); err != nil {
		return errors.Wrap(err, "failed to bind eager loaded slice user_link_topics")
	}

	if err = results.Close(); err != nil {
		return errors.Wrap(err, "failed to close results in eager load on user_link_topics")
	}
	if err = results.Err(); err != nil {
		return errors.Wrap(err, "error occurred during iteration of eager loaded relations for user_link_topics")
	}

	if singular {
		object.R.UserLinkTopics = resultSlice
		for _, foreign := range resultSlice {
			if foreign.R == nil {
				foreign.R = &userLinkTopicR{}
			}
			foreign.R.UserLink = object
		}
		return nil
	}

	for _, foreign := range resultSlice {
		for _, local := range slice {
			if local.ID == foreign.UserLinkID {
				local.R.UserLinkTopics = append(local.R.UserLinkTopics, foreign)
				if foreign.R == nil {
					foreign.R = &userLinkTopicR{}
				}
				foreign.R.UserLink = local
				break
			}
		}
	}

	return nil
}

// SetLink of the userLink to the related item.
// Sets o.R.Link to related.
// Adds o to related.R.UserLinks.
func (o *UserLink) SetLink(ctx context.Context, exec boil.ContextExecutor, insert bool, related *Link) error {
	var err error
	if insert {
		if err = related.Insert(ctx, exec, boil.Infer()); err != nil {
			return errors.Wrap(err, "failed to insert into foreign table")
		}
	}

	updateQuery := fmt.Sprintf(
		"UPDATE \"user_links\" SET %s WHERE %s",
		strmangle.SetParamNames("\"", "\"", 1, []string{"link_id"}),
		strmangle.WhereClause("\"", "\"", 2, userLinkPrimaryKeyColumns),
	)
	values := []interface{}{related.ID, o.ID}

	if boil.IsDebug(ctx) {
		writer := boil.DebugWriterFrom(ctx)
		fmt.Fprintln(writer, updateQuery)
		fmt.Fprintln(writer, values)
	}
	if _, err = exec.ExecContext(ctx, updateQuery, values...); err != nil {
		return errors.Wrap(err, "failed to update local table")
	}

	o.LinkID = related.ID
	if o.R == nil {
		o.R = &userLinkR{
			Link: related,
		}
	} else {
		o.R.Link = related
	}

	if related.R == nil {
		related.R = &linkR{
			UserLinks: UserLinkSlice{o},
		}
	} else {
		related.R.UserLinks = append(related.R.UserLinks, o)
	}

	return nil
}

// SetOrganization of the userLink to the related item.
// Sets o.R.Organization to related.
// Adds o to related.R.UserLinks.
func (o *UserLink) SetOrganization(ctx context.Context, exec boil.ContextExecutor, insert bool, related *Organization) error {
	var err error
	if insert {
		if err = related.Insert(ctx, exec, boil.Infer()); err != nil {
			return errors.Wrap(err, "failed to insert into foreign table")
		}
	}

	updateQuery := fmt.Sprintf(
		"UPDATE \"user_links\" SET %s WHERE %s",
		strmangle.SetParamNames("\"", "\"", 1, []string{"organization_id"}),
		strmangle.WhereClause("\"", "\"", 2, userLinkPrimaryKeyColumns),
	)
	values := []interface{}{related.ID, o.ID}

	if boil.IsDebug(ctx) {
		writer := boil.DebugWriterFrom(ctx)
		fmt.Fprintln(writer, updateQuery)
		fmt.Fprintln(writer, values)
	}
	if _, err = exec.ExecContext(ctx, updateQuery, values...); err != nil {
		return errors.Wrap(err, "failed to update local table")
	}

	o.OrganizationID = related.ID
	if o.R == nil {
		o.R = &userLinkR{
			Organization: related,
		}
	} else {
		o.R.Organization = related
	}

	if related.R == nil {
		related.R = &organizationR{
			UserLinks: UserLinkSlice{o},
		}
	} else {
		related.R.UserLinks = append(related.R.UserLinks, o)
	}

	return nil
}

// SetRepository of the userLink to the related item.
// Sets o.R.Repository to related.
// Adds o to related.R.UserLinks.
func (o *UserLink) SetRepository(ctx context.Context, exec boil.ContextExecutor, insert bool, related *Repository) error {
	var err error
	if insert {
		if err = related.Insert(ctx, exec, boil.Infer()); err != nil {
			return errors.Wrap(err, "failed to insert into foreign table")
		}
	}

	updateQuery := fmt.Sprintf(
		"UPDATE \"user_links\" SET %s WHERE %s",
		strmangle.SetParamNames("\"", "\"", 1, []string{"repository_id"}),
		strmangle.WhereClause("\"", "\"", 2, userLinkPrimaryKeyColumns),
	)
	values := []interface{}{related.ID, o.ID}

	if boil.IsDebug(ctx) {
		writer := boil.DebugWriterFrom(ctx)
		fmt.Fprintln(writer, updateQuery)
		fmt.Fprintln(writer, values)
	}
	if _, err = exec.ExecContext(ctx, updateQuery, values...); err != nil {
		return errors.Wrap(err, "failed to update local table")
	}

	o.RepositoryID = related.ID
	if o.R == nil {
		o.R = &userLinkR{
			Repository: related,
		}
	} else {
		o.R.Repository = related
	}

	if related.R == nil {
		related.R = &repositoryR{
			UserLinks: UserLinkSlice{o},
		}
	} else {
		related.R.UserLinks = append(related.R.UserLinks, o)
	}

	return nil
}

// SetUser of the userLink to the related item.
// Sets o.R.User to related.
// Adds o to related.R.UserLinks.
func (o *UserLink) SetUser(ctx context.Context, exec boil.ContextExecutor, insert bool, related *User) error {
	var err error
	if insert {
		if err = related.Insert(ctx, exec, boil.Infer()); err != nil {
			return errors.Wrap(err, "failed to insert into foreign table")
		}
	}

	updateQuery := fmt.Sprintf(
		"UPDATE \"user_links\" SET %s WHERE %s",
		strmangle.SetParamNames("\"", "\"", 1, []string{"user_id"}),
		strmangle.WhereClause("\"", "\"", 2, userLinkPrimaryKeyColumns),
	)
	values := []interface{}{related.ID, o.ID}

	if boil.IsDebug(ctx) {
		writer := boil.DebugWriterFrom(ctx)
		fmt.Fprintln(writer, updateQuery)
		fmt.Fprintln(writer, values)
	}
	if _, err = exec.ExecContext(ctx, updateQuery, values...); err != nil {
		return errors.Wrap(err, "failed to update local table")
	}

	o.UserID = related.ID
	if o.R == nil {
		o.R = &userLinkR{
			User: related,
		}
	} else {
		o.R.User = related
	}

	if related.R == nil {
		related.R = &userR{
			UserLinks: UserLinkSlice{o},
		}
	} else {
		related.R.UserLinks = append(related.R.UserLinks, o)
	}

	return nil
}

// AddUserLinkTopics adds the given related objects to the existing relationships
// of the user_link, optionally inserting them as new records.
// Appends related to o.R.UserLinkTopics.
// Sets related.R.UserLink appropriately.
func (o *UserLink) AddUserLinkTopics(ctx context.Context, exec boil.ContextExecutor, insert bool, related ...*UserLinkTopic) error {
	var err error
	for _, rel := range related {
		if insert {
			rel.UserLinkID = o.ID
			if err = rel.Insert(ctx, exec, boil.Infer()); err != nil {
				return errors.Wrap(err, "failed to insert into foreign table")
			}
		} else {
			updateQuery := fmt.Sprintf(
				"UPDATE \"user_link_topics\" SET %s WHERE %s",
				strmangle.SetParamNames("\"", "\"", 1, []string{"user_link_id"}),
				strmangle.WhereClause("\"", "\"", 2, userLinkTopicPrimaryKeyColumns),
			)
			values := []interface{}{o.ID, rel.ID}

			if boil.IsDebug(ctx) {
				writer := boil.DebugWriterFrom(ctx)
				fmt.Fprintln(writer, updateQuery)
				fmt.Fprintln(writer, values)
			}
			if _, err = exec.ExecContext(ctx, updateQuery, values...); err != nil {
				return errors.Wrap(err, "failed to update foreign table")
			}

			rel.UserLinkID = o.ID
		}
	}

	if o.R == nil {
		o.R = &userLinkR{
			UserLinkTopics: related,
		}
	} else {
		o.R.UserLinkTopics = append(o.R.UserLinkTopics, related...)
	}

	for _, rel := range related {
		if rel.R == nil {
			rel.R = &userLinkTopicR{
				UserLink: o,
			}
		} else {
			rel.R.UserLink = o
		}
	}
	return nil
}

// UserLinks retrieves all the records using an executor.
func UserLinks(mods ...qm.QueryMod) userLinkQuery {
	mods = append(mods, qm.From("\"user_links\""))
	return userLinkQuery{NewQuery(mods...)}
}

// FindUserLink retrieves a single record by ID with an executor.
// If selectCols is empty Find will return all columns.
func FindUserLink(ctx context.Context, exec boil.ContextExecutor, iD string, selectCols ...string) (*UserLink, error) {
	userLinkObj := &UserLink{}

	sel := "*"
	if len(selectCols) > 0 {
		sel = strings.Join(strmangle.IdentQuoteSlice(dialect.LQ, dialect.RQ, selectCols), ",")
	}
	query := fmt.Sprintf(
		"select %s from \"user_links\" where \"id\"=$1", sel,
	)

	q := queries.Raw(query, iD)

	err := q.Bind(ctx, exec, userLinkObj)
	if err != nil {
		if errors.Cause(err) == sql.ErrNoRows {
			return nil, sql.ErrNoRows
		}
		return nil, errors.Wrap(err, "models: unable to select from user_links")
	}

	return userLinkObj, nil
}

// Insert a single record using an executor.
// See boil.Columns.InsertColumnSet documentation to understand column list inference for inserts.
func (o *UserLink) Insert(ctx context.Context, exec boil.ContextExecutor, columns boil.Columns) error {
	if o == nil {
		return errors.New("models: no user_links provided for insertion")
	}

	var err error
	if !boil.TimestampsAreSkipped(ctx) {
		currTime := time.Now().In(boil.GetLocation())

		if o.CreatedAt.IsZero() {
			o.CreatedAt = currTime
		}
	}

	nzDefaults := queries.NonZeroDefaultSet(userLinkColumnsWithDefault, o)

	key := makeCacheKey(columns, nzDefaults)
	userLinkInsertCacheMut.RLock()
	cache, cached := userLinkInsertCache[key]
	userLinkInsertCacheMut.RUnlock()

	if !cached {
		wl, returnColumns := columns.InsertColumnSet(
			userLinkAllColumns,
			userLinkColumnsWithDefault,
			userLinkColumnsWithoutDefault,
			nzDefaults,
		)

		cache.valueMapping, err = queries.BindMapping(userLinkType, userLinkMapping, wl)
		if err != nil {
			return err
		}
		cache.retMapping, err = queries.BindMapping(userLinkType, userLinkMapping, returnColumns)
		if err != nil {
			return err
		}
		if len(wl) != 0 {
			cache.query = fmt.Sprintf("INSERT INTO \"user_links\" (\"%s\") %%sVALUES (%s)%%s", strings.Join(wl, "\",\""), strmangle.Placeholders(dialect.UseIndexPlaceholders, len(wl), 1, 1))
		} else {
			cache.query = "INSERT INTO \"user_links\" %sDEFAULT VALUES%s"
		}

		var queryOutput, queryReturning string

		if len(cache.retMapping) != 0 {
			queryReturning = fmt.Sprintf(" RETURNING \"%s\"", strings.Join(returnColumns, "\",\""))
		}

		cache.query = fmt.Sprintf(cache.query, queryOutput, queryReturning)
	}

	value := reflect.Indirect(reflect.ValueOf(o))
	vals := queries.ValuesFromMapping(value, cache.valueMapping)

	if boil.IsDebug(ctx) {
		writer := boil.DebugWriterFrom(ctx)
		fmt.Fprintln(writer, cache.query)
		fmt.Fprintln(writer, vals)
	}

	if len(cache.retMapping) != 0 {
		err = exec.QueryRowContext(ctx, cache.query, vals...).Scan(queries.PtrsFromMapping(value, cache.retMapping)...)
	} else {
		_, err = exec.ExecContext(ctx, cache.query, vals...)
	}

	if err != nil {
		return errors.Wrap(err, "models: unable to insert into user_links")
	}

	if !cached {
		userLinkInsertCacheMut.Lock()
		userLinkInsertCache[key] = cache
		userLinkInsertCacheMut.Unlock()
	}

	return nil
}

// Update uses an executor to update the UserLink.
// See boil.Columns.UpdateColumnSet documentation to understand column list inference for updates.
// Update does not automatically update the record in case of default values. Use .Reload() to refresh the records.
func (o *UserLink) Update(ctx context.Context, exec boil.ContextExecutor, columns boil.Columns) (int64, error) {
	var err error
	key := makeCacheKey(columns, nil)
	userLinkUpdateCacheMut.RLock()
	cache, cached := userLinkUpdateCache[key]
	userLinkUpdateCacheMut.RUnlock()

	if !cached {
		wl := columns.UpdateColumnSet(
			userLinkAllColumns,
			userLinkPrimaryKeyColumns,
		)

		if !columns.IsWhitelist() {
			wl = strmangle.SetComplement(wl, []string{"created_at"})
		}
		if len(wl) == 0 {
			return 0, errors.New("models: unable to update user_links, could not build whitelist")
		}

		cache.query = fmt.Sprintf("UPDATE \"user_links\" SET %s WHERE %s",
			strmangle.SetParamNames("\"", "\"", 1, wl),
			strmangle.WhereClause("\"", "\"", len(wl)+1, userLinkPrimaryKeyColumns),
		)
		cache.valueMapping, err = queries.BindMapping(userLinkType, userLinkMapping, append(wl, userLinkPrimaryKeyColumns...))
		if err != nil {
			return 0, err
		}
	}

	values := queries.ValuesFromMapping(reflect.Indirect(reflect.ValueOf(o)), cache.valueMapping)

	if boil.IsDebug(ctx) {
		writer := boil.DebugWriterFrom(ctx)
		fmt.Fprintln(writer, cache.query)
		fmt.Fprintln(writer, values)
	}
	var result sql.Result
	result, err = exec.ExecContext(ctx, cache.query, values...)
	if err != nil {
		return 0, errors.Wrap(err, "models: unable to update user_links row")
	}

	rowsAff, err := result.RowsAffected()
	if err != nil {
		return 0, errors.Wrap(err, "models: failed to get rows affected by update for user_links")
	}

	if !cached {
		userLinkUpdateCacheMut.Lock()
		userLinkUpdateCache[key] = cache
		userLinkUpdateCacheMut.Unlock()
	}

	return rowsAff, nil
}

// UpdateAll updates all rows with the specified column values.
func (q userLinkQuery) UpdateAll(ctx context.Context, exec boil.ContextExecutor, cols M) (int64, error) {
	queries.SetUpdate(q.Query, cols)

	result, err := q.Query.ExecContext(ctx, exec)
	if err != nil {
		return 0, errors.Wrap(err, "models: unable to update all for user_links")
	}

	rowsAff, err := result.RowsAffected()
	if err != nil {
		return 0, errors.Wrap(err, "models: unable to retrieve rows affected for user_links")
	}

	return rowsAff, nil
}

// UpdateAll updates all rows with the specified column values, using an executor.
func (o UserLinkSlice) UpdateAll(ctx context.Context, exec boil.ContextExecutor, cols M) (int64, error) {
	ln := int64(len(o))
	if ln == 0 {
		return 0, nil
	}

	if len(cols) == 0 {
		return 0, errors.New("models: update all requires at least one column argument")
	}

	colNames := make([]string, len(cols))
	args := make([]interface{}, len(cols))

	i := 0
	for name, value := range cols {
		colNames[i] = name
		args[i] = value
		i++
	}

	// Append all of the primary key values for each column
	for _, obj := range o {
		pkeyArgs := queries.ValuesFromMapping(reflect.Indirect(reflect.ValueOf(obj)), userLinkPrimaryKeyMapping)
		args = append(args, pkeyArgs...)
	}

	sql := fmt.Sprintf("UPDATE \"user_links\" SET %s WHERE %s",
		strmangle.SetParamNames("\"", "\"", 1, colNames),
		strmangle.WhereClauseRepeated(string(dialect.LQ), string(dialect.RQ), len(colNames)+1, userLinkPrimaryKeyColumns, len(o)))

	if boil.IsDebug(ctx) {
		writer := boil.DebugWriterFrom(ctx)
		fmt.Fprintln(writer, sql)
		fmt.Fprintln(writer, args...)
	}
	result, err := exec.ExecContext(ctx, sql, args...)
	if err != nil {
		return 0, errors.Wrap(err, "models: unable to update all in userLink slice")
	}

	rowsAff, err := result.RowsAffected()
	if err != nil {
		return 0, errors.Wrap(err, "models: unable to retrieve rows affected all in update all userLink")
	}
	return rowsAff, nil
}

// Upsert attempts an insert using an executor, and does an update or ignore on conflict.
// See boil.Columns documentation for how to properly use updateColumns and insertColumns.
func (o *UserLink) Upsert(ctx context.Context, exec boil.ContextExecutor, updateOnConflict bool, conflictColumns []string, updateColumns, insertColumns boil.Columns) error {
	if o == nil {
		return errors.New("models: no user_links provided for upsert")
	}
	if !boil.TimestampsAreSkipped(ctx) {
		currTime := time.Now().In(boil.GetLocation())

		if o.CreatedAt.IsZero() {
			o.CreatedAt = currTime
		}
	}

	nzDefaults := queries.NonZeroDefaultSet(userLinkColumnsWithDefault, o)

	// Build cache key in-line uglily - mysql vs psql problems
	buf := strmangle.GetBuffer()
	if updateOnConflict {
		buf.WriteByte('t')
	} else {
		buf.WriteByte('f')
	}
	buf.WriteByte('.')
	for _, c := range conflictColumns {
		buf.WriteString(c)
	}
	buf.WriteByte('.')
	buf.WriteString(strconv.Itoa(updateColumns.Kind))
	for _, c := range updateColumns.Cols {
		buf.WriteString(c)
	}
	buf.WriteByte('.')
	buf.WriteString(strconv.Itoa(insertColumns.Kind))
	for _, c := range insertColumns.Cols {
		buf.WriteString(c)
	}
	buf.WriteByte('.')
	for _, c := range nzDefaults {
		buf.WriteString(c)
	}
	key := buf.String()
	strmangle.PutBuffer(buf)

	userLinkUpsertCacheMut.RLock()
	cache, cached := userLinkUpsertCache[key]
	userLinkUpsertCacheMut.RUnlock()

	var err error

	if !cached {
		insert, ret := insertColumns.InsertColumnSet(
			userLinkAllColumns,
			userLinkColumnsWithDefault,
			userLinkColumnsWithoutDefault,
			nzDefaults,
		)
		update := updateColumns.UpdateColumnSet(
			userLinkAllColumns,
			userLinkPrimaryKeyColumns,
		)

		if updateOnConflict && len(update) == 0 {
			return errors.New("models: unable to upsert user_links, could not build update column list")
		}

		conflict := conflictColumns
		if len(conflict) == 0 {
			conflict = make([]string, len(userLinkPrimaryKeyColumns))
			copy(conflict, userLinkPrimaryKeyColumns)
		}
		cache.query = buildUpsertQueryPostgres(dialect, "\"user_links\"", updateOnConflict, ret, update, conflict, insert)

		cache.valueMapping, err = queries.BindMapping(userLinkType, userLinkMapping, insert)
		if err != nil {
			return err
		}
		if len(ret) != 0 {
			cache.retMapping, err = queries.BindMapping(userLinkType, userLinkMapping, ret)
			if err != nil {
				return err
			}
		}
	}

	value := reflect.Indirect(reflect.ValueOf(o))
	vals := queries.ValuesFromMapping(value, cache.valueMapping)
	var returns []interface{}
	if len(cache.retMapping) != 0 {
		returns = queries.PtrsFromMapping(value, cache.retMapping)
	}

	if boil.IsDebug(ctx) {
		writer := boil.DebugWriterFrom(ctx)
		fmt.Fprintln(writer, cache.query)
		fmt.Fprintln(writer, vals)
	}
	if len(cache.retMapping) != 0 {
		err = exec.QueryRowContext(ctx, cache.query, vals...).Scan(returns...)
		if err == sql.ErrNoRows {
			err = nil // Postgres doesn't return anything when there's no update
		}
	} else {
		_, err = exec.ExecContext(ctx, cache.query, vals...)
	}
	if err != nil {
		return errors.Wrap(err, "models: unable to upsert user_links")
	}

	if !cached {
		userLinkUpsertCacheMut.Lock()
		userLinkUpsertCache[key] = cache
		userLinkUpsertCacheMut.Unlock()
	}

	return nil
}

// Delete deletes a single UserLink record with an executor.
// Delete will match against the primary key column to find the record to delete.
func (o *UserLink) Delete(ctx context.Context, exec boil.ContextExecutor) (int64, error) {
	if o == nil {
		return 0, errors.New("models: no UserLink provided for delete")
	}

	args := queries.ValuesFromMapping(reflect.Indirect(reflect.ValueOf(o)), userLinkPrimaryKeyMapping)
	sql := "DELETE FROM \"user_links\" WHERE \"id\"=$1"

	if boil.IsDebug(ctx) {
		writer := boil.DebugWriterFrom(ctx)
		fmt.Fprintln(writer, sql)
		fmt.Fprintln(writer, args...)
	}
	result, err := exec.ExecContext(ctx, sql, args...)
	if err != nil {
		return 0, errors.Wrap(err, "models: unable to delete from user_links")
	}

	rowsAff, err := result.RowsAffected()
	if err != nil {
		return 0, errors.Wrap(err, "models: failed to get rows affected by delete for user_links")
	}

	return rowsAff, nil
}

// DeleteAll deletes all matching rows.
func (q userLinkQuery) DeleteAll(ctx context.Context, exec boil.ContextExecutor) (int64, error) {
	if q.Query == nil {
		return 0, errors.New("models: no userLinkQuery provided for delete all")
	}

	queries.SetDelete(q.Query)

	result, err := q.Query.ExecContext(ctx, exec)
	if err != nil {
		return 0, errors.Wrap(err, "models: unable to delete all from user_links")
	}

	rowsAff, err := result.RowsAffected()
	if err != nil {
		return 0, errors.Wrap(err, "models: failed to get rows affected by deleteall for user_links")
	}

	return rowsAff, nil
}

// DeleteAll deletes all rows in the slice, using an executor.
func (o UserLinkSlice) DeleteAll(ctx context.Context, exec boil.ContextExecutor) (int64, error) {
	if len(o) == 0 {
		return 0, nil
	}

	var args []interface{}
	for _, obj := range o {
		pkeyArgs := queries.ValuesFromMapping(reflect.Indirect(reflect.ValueOf(obj)), userLinkPrimaryKeyMapping)
		args = append(args, pkeyArgs...)
	}

	sql := "DELETE FROM \"user_links\" WHERE " +
		strmangle.WhereClauseRepeated(string(dialect.LQ), string(dialect.RQ), 1, userLinkPrimaryKeyColumns, len(o))

	if boil.IsDebug(ctx) {
		writer := boil.DebugWriterFrom(ctx)
		fmt.Fprintln(writer, sql)
		fmt.Fprintln(writer, args)
	}
	result, err := exec.ExecContext(ctx, sql, args...)
	if err != nil {
		return 0, errors.Wrap(err, "models: unable to delete all from userLink slice")
	}

	rowsAff, err := result.RowsAffected()
	if err != nil {
		return 0, errors.Wrap(err, "models: failed to get rows affected by deleteall for user_links")
	}

	return rowsAff, nil
}

// Reload refetches the object from the database
// using the primary keys with an executor.
func (o *UserLink) Reload(ctx context.Context, exec boil.ContextExecutor) error {
	ret, err := FindUserLink(ctx, exec, o.ID)
	if err != nil {
		return err
	}

	*o = *ret
	return nil
}

// ReloadAll refetches every row with matching primary key column values
// and overwrites the original object slice with the newly updated slice.
func (o *UserLinkSlice) ReloadAll(ctx context.Context, exec boil.ContextExecutor) error {
	if o == nil || len(*o) == 0 {
		return nil
	}

	slice := UserLinkSlice{}
	var args []interface{}
	for _, obj := range *o {
		pkeyArgs := queries.ValuesFromMapping(reflect.Indirect(reflect.ValueOf(obj)), userLinkPrimaryKeyMapping)
		args = append(args, pkeyArgs...)
	}

	sql := "SELECT \"user_links\".* FROM \"user_links\" WHERE " +
		strmangle.WhereClauseRepeated(string(dialect.LQ), string(dialect.RQ), 1, userLinkPrimaryKeyColumns, len(*o))

	q := queries.Raw(sql, args...)

	err := q.Bind(ctx, exec, &slice)
	if err != nil {
		return errors.Wrap(err, "models: unable to reload all in UserLinkSlice")
	}

	*o = slice

	return nil
}

// UserLinkExists checks if the UserLink row exists.
func UserLinkExists(ctx context.Context, exec boil.ContextExecutor, iD string) (bool, error) {
	var exists bool
	sql := "select exists(select 1 from \"user_links\" where \"id\"=$1 limit 1)"

	if boil.IsDebug(ctx) {
		writer := boil.DebugWriterFrom(ctx)
		fmt.Fprintln(writer, sql)
		fmt.Fprintln(writer, iD)
	}
	row := exec.QueryRowContext(ctx, sql, iD)

	err := row.Scan(&exists)
	if err != nil {
		return false, errors.Wrap(err, "models: unable to check if user_links exists")
	}

	return exists, nil
}