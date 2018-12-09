// Code generated by SQLBoiler (https://github.com/volatiletech/sqlboiler). DO NOT EDIT.
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

	"github.com/pkg/errors"
	"github.com/volatiletech/sqlboiler/boil"
	"github.com/volatiletech/sqlboiler/queries"
	"github.com/volatiletech/sqlboiler/queries/qm"
	"github.com/volatiletech/sqlboiler/strmangle"
)

// Organization is an object representing the database table.
type Organization struct {
	ID        string    `boil:"id" json:"id" toml:"id" yaml:"id"`
	Name      string    `boil:"name" json:"name" toml:"name" yaml:"name"`
	CreatedAt time.Time `boil:"created_at" json:"created_at" toml:"created_at" yaml:"created_at"`
	UpdatedAt time.Time `boil:"updated_at" json:"updated_at" toml:"updated_at" yaml:"updated_at"`

	R *organizationR `boil:"-" json:"-" toml:"-" yaml:"-"`
	L organizationL  `boil:"-" json:"-" toml:"-" yaml:"-"`
}

var OrganizationColumns = struct {
	ID        string
	Name      string
	CreatedAt string
	UpdatedAt string
}{
	ID:        "id",
	Name:      "name",
	CreatedAt: "created_at",
	UpdatedAt: "updated_at",
}

// OrganizationRels is where relationship names are stored.
var OrganizationRels = struct {
	Links        string
	Repositories string
	Topics       string
}{
	Links:        "Links",
	Repositories: "Repositories",
	Topics:       "Topics",
}

// organizationR is where relationships are stored.
type organizationR struct {
	Links        LinkSlice
	Repositories RepositorySlice
	Topics       TopicSlice
}

// NewStruct creates a new relationship struct
func (*organizationR) NewStruct() *organizationR {
	return &organizationR{}
}

// organizationL is where Load methods for each relationship are stored.
type organizationL struct{}

var (
	organizationColumns               = []string{"id", "name", "created_at", "updated_at"}
	organizationColumnsWithoutDefault = []string{"name"}
	organizationColumnsWithDefault    = []string{"id", "created_at", "updated_at"}
	organizationPrimaryKeyColumns     = []string{"id"}
)

type (
	// OrganizationSlice is an alias for a slice of pointers to Organization.
	// This should generally be used opposed to []Organization.
	OrganizationSlice []*Organization
	// OrganizationHook is the signature for custom Organization hook methods
	OrganizationHook func(context.Context, boil.ContextExecutor, *Organization) error

	organizationQuery struct {
		*queries.Query
	}
)

// Cache for insert, update and upsert
var (
	organizationType                 = reflect.TypeOf(&Organization{})
	organizationMapping              = queries.MakeStructMapping(organizationType)
	organizationPrimaryKeyMapping, _ = queries.BindMapping(organizationType, organizationMapping, organizationPrimaryKeyColumns)
	organizationInsertCacheMut       sync.RWMutex
	organizationInsertCache          = make(map[string]insertCache)
	organizationUpdateCacheMut       sync.RWMutex
	organizationUpdateCache          = make(map[string]updateCache)
	organizationUpsertCacheMut       sync.RWMutex
	organizationUpsertCache          = make(map[string]insertCache)
)

var (
	// Force time package dependency for automated UpdatedAt/CreatedAt.
	_ = time.Second
)

var organizationBeforeInsertHooks []OrganizationHook
var organizationBeforeUpdateHooks []OrganizationHook
var organizationBeforeDeleteHooks []OrganizationHook
var organizationBeforeUpsertHooks []OrganizationHook

var organizationAfterInsertHooks []OrganizationHook
var organizationAfterSelectHooks []OrganizationHook
var organizationAfterUpdateHooks []OrganizationHook
var organizationAfterDeleteHooks []OrganizationHook
var organizationAfterUpsertHooks []OrganizationHook

// doBeforeInsertHooks executes all "before insert" hooks.
func (o *Organization) doBeforeInsertHooks(ctx context.Context, exec boil.ContextExecutor) (err error) {
	for _, hook := range organizationBeforeInsertHooks {
		if err := hook(ctx, exec, o); err != nil {
			return err
		}
	}

	return nil
}

// doBeforeUpdateHooks executes all "before Update" hooks.
func (o *Organization) doBeforeUpdateHooks(ctx context.Context, exec boil.ContextExecutor) (err error) {
	for _, hook := range organizationBeforeUpdateHooks {
		if err := hook(ctx, exec, o); err != nil {
			return err
		}
	}

	return nil
}

// doBeforeDeleteHooks executes all "before Delete" hooks.
func (o *Organization) doBeforeDeleteHooks(ctx context.Context, exec boil.ContextExecutor) (err error) {
	for _, hook := range organizationBeforeDeleteHooks {
		if err := hook(ctx, exec, o); err != nil {
			return err
		}
	}

	return nil
}

// doBeforeUpsertHooks executes all "before Upsert" hooks.
func (o *Organization) doBeforeUpsertHooks(ctx context.Context, exec boil.ContextExecutor) (err error) {
	for _, hook := range organizationBeforeUpsertHooks {
		if err := hook(ctx, exec, o); err != nil {
			return err
		}
	}

	return nil
}

// doAfterInsertHooks executes all "after Insert" hooks.
func (o *Organization) doAfterInsertHooks(ctx context.Context, exec boil.ContextExecutor) (err error) {
	for _, hook := range organizationAfterInsertHooks {
		if err := hook(ctx, exec, o); err != nil {
			return err
		}
	}

	return nil
}

// doAfterSelectHooks executes all "after Select" hooks.
func (o *Organization) doAfterSelectHooks(ctx context.Context, exec boil.ContextExecutor) (err error) {
	for _, hook := range organizationAfterSelectHooks {
		if err := hook(ctx, exec, o); err != nil {
			return err
		}
	}

	return nil
}

// doAfterUpdateHooks executes all "after Update" hooks.
func (o *Organization) doAfterUpdateHooks(ctx context.Context, exec boil.ContextExecutor) (err error) {
	for _, hook := range organizationAfterUpdateHooks {
		if err := hook(ctx, exec, o); err != nil {
			return err
		}
	}

	return nil
}

// doAfterDeleteHooks executes all "after Delete" hooks.
func (o *Organization) doAfterDeleteHooks(ctx context.Context, exec boil.ContextExecutor) (err error) {
	for _, hook := range organizationAfterDeleteHooks {
		if err := hook(ctx, exec, o); err != nil {
			return err
		}
	}

	return nil
}

// doAfterUpsertHooks executes all "after Upsert" hooks.
func (o *Organization) doAfterUpsertHooks(ctx context.Context, exec boil.ContextExecutor) (err error) {
	for _, hook := range organizationAfterUpsertHooks {
		if err := hook(ctx, exec, o); err != nil {
			return err
		}
	}

	return nil
}

// AddOrganizationHook registers your hook function for all future operations.
func AddOrganizationHook(hookPoint boil.HookPoint, organizationHook OrganizationHook) {
	switch hookPoint {
	case boil.BeforeInsertHook:
		organizationBeforeInsertHooks = append(organizationBeforeInsertHooks, organizationHook)
	case boil.BeforeUpdateHook:
		organizationBeforeUpdateHooks = append(organizationBeforeUpdateHooks, organizationHook)
	case boil.BeforeDeleteHook:
		organizationBeforeDeleteHooks = append(organizationBeforeDeleteHooks, organizationHook)
	case boil.BeforeUpsertHook:
		organizationBeforeUpsertHooks = append(organizationBeforeUpsertHooks, organizationHook)
	case boil.AfterInsertHook:
		organizationAfterInsertHooks = append(organizationAfterInsertHooks, organizationHook)
	case boil.AfterSelectHook:
		organizationAfterSelectHooks = append(organizationAfterSelectHooks, organizationHook)
	case boil.AfterUpdateHook:
		organizationAfterUpdateHooks = append(organizationAfterUpdateHooks, organizationHook)
	case boil.AfterDeleteHook:
		organizationAfterDeleteHooks = append(organizationAfterDeleteHooks, organizationHook)
	case boil.AfterUpsertHook:
		organizationAfterUpsertHooks = append(organizationAfterUpsertHooks, organizationHook)
	}
}

// One returns a single organization record from the query.
func (q organizationQuery) One(ctx context.Context, exec boil.ContextExecutor) (*Organization, error) {
	o := &Organization{}

	queries.SetLimit(q.Query, 1)

	err := q.Bind(ctx, exec, o)
	if err != nil {
		if errors.Cause(err) == sql.ErrNoRows {
			return nil, sql.ErrNoRows
		}
		return nil, errors.Wrap(err, "models: failed to execute a one query for organizations")
	}

	if err := o.doAfterSelectHooks(ctx, exec); err != nil {
		return o, err
	}

	return o, nil
}

// All returns all Organization records from the query.
func (q organizationQuery) All(ctx context.Context, exec boil.ContextExecutor) (OrganizationSlice, error) {
	var o []*Organization

	err := q.Bind(ctx, exec, &o)
	if err != nil {
		return nil, errors.Wrap(err, "models: failed to assign all query results to Organization slice")
	}

	if len(organizationAfterSelectHooks) != 0 {
		for _, obj := range o {
			if err := obj.doAfterSelectHooks(ctx, exec); err != nil {
				return o, err
			}
		}
	}

	return o, nil
}

// Count returns the count of all Organization records in the query.
func (q organizationQuery) Count(ctx context.Context, exec boil.ContextExecutor) (int64, error) {
	var count int64

	queries.SetSelect(q.Query, nil)
	queries.SetCount(q.Query)

	err := q.Query.QueryRowContext(ctx, exec).Scan(&count)
	if err != nil {
		return 0, errors.Wrap(err, "models: failed to count organizations rows")
	}

	return count, nil
}

// Exists checks if the row exists in the table.
func (q organizationQuery) Exists(ctx context.Context, exec boil.ContextExecutor) (bool, error) {
	var count int64

	queries.SetSelect(q.Query, nil)
	queries.SetCount(q.Query)
	queries.SetLimit(q.Query, 1)

	err := q.Query.QueryRowContext(ctx, exec).Scan(&count)
	if err != nil {
		return false, errors.Wrap(err, "models: failed to check if organizations exists")
	}

	return count > 0, nil
}

// Links retrieves all the link's Links with an executor.
func (o *Organization) Links(mods ...qm.QueryMod) linkQuery {
	var queryMods []qm.QueryMod
	if len(mods) != 0 {
		queryMods = append(queryMods, mods...)
	}

	queryMods = append(queryMods,
		qm.Where("\"links\".\"organization_id\"=?", o.ID),
	)

	query := Links(queryMods...)
	queries.SetFrom(query.Query, "\"links\"")

	if len(queries.GetSelect(query.Query)) == 0 {
		queries.SetSelect(query.Query, []string{"\"links\".*"})
	}

	return query
}

// Repositories retrieves all the repository's Repositories with an executor.
func (o *Organization) Repositories(mods ...qm.QueryMod) repositoryQuery {
	var queryMods []qm.QueryMod
	if len(mods) != 0 {
		queryMods = append(queryMods, mods...)
	}

	queryMods = append(queryMods,
		qm.Where("\"repositories\".\"organization_id\"=?", o.ID),
	)

	query := Repositories(queryMods...)
	queries.SetFrom(query.Query, "\"repositories\"")

	if len(queries.GetSelect(query.Query)) == 0 {
		queries.SetSelect(query.Query, []string{"\"repositories\".*"})
	}

	return query
}

// Topics retrieves all the topic's Topics with an executor.
func (o *Organization) Topics(mods ...qm.QueryMod) topicQuery {
	var queryMods []qm.QueryMod
	if len(mods) != 0 {
		queryMods = append(queryMods, mods...)
	}

	queryMods = append(queryMods,
		qm.Where("\"topics\".\"organization_id\"=?", o.ID),
	)

	query := Topics(queryMods...)
	queries.SetFrom(query.Query, "\"topics\"")

	if len(queries.GetSelect(query.Query)) == 0 {
		queries.SetSelect(query.Query, []string{"\"topics\".*"})
	}

	return query
}

// LoadLinks allows an eager lookup of values, cached into the
// loaded structs of the objects. This is for a 1-M or N-M relationship.
func (organizationL) LoadLinks(ctx context.Context, e boil.ContextExecutor, singular bool, maybeOrganization interface{}, mods queries.Applicator) error {
	var slice []*Organization
	var object *Organization

	if singular {
		object = maybeOrganization.(*Organization)
	} else {
		slice = *maybeOrganization.(*[]*Organization)
	}

	args := make([]interface{}, 0, 1)
	if singular {
		if object.R == nil {
			object.R = &organizationR{}
		}
		args = append(args, object.ID)
	} else {
	Outer:
		for _, obj := range slice {
			if obj.R == nil {
				obj.R = &organizationR{}
			}

			for _, a := range args {
				if a == obj.ID {
					continue Outer
				}
			}

			args = append(args, obj.ID)
		}
	}

	query := NewQuery(qm.From(`links`), qm.WhereIn(`organization_id in ?`, args...))
	if mods != nil {
		mods.Apply(query)
	}

	results, err := query.QueryContext(ctx, e)
	if err != nil {
		return errors.Wrap(err, "failed to eager load links")
	}

	var resultSlice []*Link
	if err = queries.Bind(results, &resultSlice); err != nil {
		return errors.Wrap(err, "failed to bind eager loaded slice links")
	}

	if err = results.Close(); err != nil {
		return errors.Wrap(err, "failed to close results in eager load on links")
	}
	if err = results.Err(); err != nil {
		return errors.Wrap(err, "error occurred during iteration of eager loaded relations for links")
	}

	if len(linkAfterSelectHooks) != 0 {
		for _, obj := range resultSlice {
			if err := obj.doAfterSelectHooks(ctx, e); err != nil {
				return err
			}
		}
	}
	if singular {
		object.R.Links = resultSlice
		for _, foreign := range resultSlice {
			if foreign.R == nil {
				foreign.R = &linkR{}
			}
			foreign.R.Organization = object
		}
		return nil
	}

	for _, foreign := range resultSlice {
		for _, local := range slice {
			if local.ID == foreign.OrganizationID {
				local.R.Links = append(local.R.Links, foreign)
				if foreign.R == nil {
					foreign.R = &linkR{}
				}
				foreign.R.Organization = local
				break
			}
		}
	}

	return nil
}

// LoadRepositories allows an eager lookup of values, cached into the
// loaded structs of the objects. This is for a 1-M or N-M relationship.
func (organizationL) LoadRepositories(ctx context.Context, e boil.ContextExecutor, singular bool, maybeOrganization interface{}, mods queries.Applicator) error {
	var slice []*Organization
	var object *Organization

	if singular {
		object = maybeOrganization.(*Organization)
	} else {
		slice = *maybeOrganization.(*[]*Organization)
	}

	args := make([]interface{}, 0, 1)
	if singular {
		if object.R == nil {
			object.R = &organizationR{}
		}
		args = append(args, object.ID)
	} else {
	Outer:
		for _, obj := range slice {
			if obj.R == nil {
				obj.R = &organizationR{}
			}

			for _, a := range args {
				if a == obj.ID {
					continue Outer
				}
			}

			args = append(args, obj.ID)
		}
	}

	query := NewQuery(qm.From(`repositories`), qm.WhereIn(`organization_id in ?`, args...))
	if mods != nil {
		mods.Apply(query)
	}

	results, err := query.QueryContext(ctx, e)
	if err != nil {
		return errors.Wrap(err, "failed to eager load repositories")
	}

	var resultSlice []*Repository
	if err = queries.Bind(results, &resultSlice); err != nil {
		return errors.Wrap(err, "failed to bind eager loaded slice repositories")
	}

	if err = results.Close(); err != nil {
		return errors.Wrap(err, "failed to close results in eager load on repositories")
	}
	if err = results.Err(); err != nil {
		return errors.Wrap(err, "error occurred during iteration of eager loaded relations for repositories")
	}

	if len(repositoryAfterSelectHooks) != 0 {
		for _, obj := range resultSlice {
			if err := obj.doAfterSelectHooks(ctx, e); err != nil {
				return err
			}
		}
	}
	if singular {
		object.R.Repositories = resultSlice
		for _, foreign := range resultSlice {
			if foreign.R == nil {
				foreign.R = &repositoryR{}
			}
			foreign.R.Organization = object
		}
		return nil
	}

	for _, foreign := range resultSlice {
		for _, local := range slice {
			if local.ID == foreign.OrganizationID {
				local.R.Repositories = append(local.R.Repositories, foreign)
				if foreign.R == nil {
					foreign.R = &repositoryR{}
				}
				foreign.R.Organization = local
				break
			}
		}
	}

	return nil
}

// LoadTopics allows an eager lookup of values, cached into the
// loaded structs of the objects. This is for a 1-M or N-M relationship.
func (organizationL) LoadTopics(ctx context.Context, e boil.ContextExecutor, singular bool, maybeOrganization interface{}, mods queries.Applicator) error {
	var slice []*Organization
	var object *Organization

	if singular {
		object = maybeOrganization.(*Organization)
	} else {
		slice = *maybeOrganization.(*[]*Organization)
	}

	args := make([]interface{}, 0, 1)
	if singular {
		if object.R == nil {
			object.R = &organizationR{}
		}
		args = append(args, object.ID)
	} else {
	Outer:
		for _, obj := range slice {
			if obj.R == nil {
				obj.R = &organizationR{}
			}

			for _, a := range args {
				if a == obj.ID {
					continue Outer
				}
			}

			args = append(args, obj.ID)
		}
	}

	query := NewQuery(qm.From(`topics`), qm.WhereIn(`organization_id in ?`, args...))
	if mods != nil {
		mods.Apply(query)
	}

	results, err := query.QueryContext(ctx, e)
	if err != nil {
		return errors.Wrap(err, "failed to eager load topics")
	}

	var resultSlice []*Topic
	if err = queries.Bind(results, &resultSlice); err != nil {
		return errors.Wrap(err, "failed to bind eager loaded slice topics")
	}

	if err = results.Close(); err != nil {
		return errors.Wrap(err, "failed to close results in eager load on topics")
	}
	if err = results.Err(); err != nil {
		return errors.Wrap(err, "error occurred during iteration of eager loaded relations for topics")
	}

	if len(topicAfterSelectHooks) != 0 {
		for _, obj := range resultSlice {
			if err := obj.doAfterSelectHooks(ctx, e); err != nil {
				return err
			}
		}
	}
	if singular {
		object.R.Topics = resultSlice
		for _, foreign := range resultSlice {
			if foreign.R == nil {
				foreign.R = &topicR{}
			}
			foreign.R.Organization = object
		}
		return nil
	}

	for _, foreign := range resultSlice {
		for _, local := range slice {
			if local.ID == foreign.OrganizationID {
				local.R.Topics = append(local.R.Topics, foreign)
				if foreign.R == nil {
					foreign.R = &topicR{}
				}
				foreign.R.Organization = local
				break
			}
		}
	}

	return nil
}

// AddLinks adds the given related objects to the existing relationships
// of the organization, optionally inserting them as new records.
// Appends related to o.R.Links.
// Sets related.R.Organization appropriately.
func (o *Organization) AddLinks(ctx context.Context, exec boil.ContextExecutor, insert bool, related ...*Link) error {
	var err error
	for _, rel := range related {
		if insert {
			rel.OrganizationID = o.ID
			if err = rel.Insert(ctx, exec, boil.Infer()); err != nil {
				return errors.Wrap(err, "failed to insert into foreign table")
			}
		} else {
			updateQuery := fmt.Sprintf(
				"UPDATE \"links\" SET %s WHERE %s",
				strmangle.SetParamNames("\"", "\"", 1, []string{"organization_id"}),
				strmangle.WhereClause("\"", "\"", 2, linkPrimaryKeyColumns),
			)
			values := []interface{}{o.ID, rel.ID}

			if boil.DebugMode {
				fmt.Fprintln(boil.DebugWriter, updateQuery)
				fmt.Fprintln(boil.DebugWriter, values)
			}

			if _, err = exec.ExecContext(ctx, updateQuery, values...); err != nil {
				return errors.Wrap(err, "failed to update foreign table")
			}

			rel.OrganizationID = o.ID
		}
	}

	if o.R == nil {
		o.R = &organizationR{
			Links: related,
		}
	} else {
		o.R.Links = append(o.R.Links, related...)
	}

	for _, rel := range related {
		if rel.R == nil {
			rel.R = &linkR{
				Organization: o,
			}
		} else {
			rel.R.Organization = o
		}
	}
	return nil
}

// AddRepositories adds the given related objects to the existing relationships
// of the organization, optionally inserting them as new records.
// Appends related to o.R.Repositories.
// Sets related.R.Organization appropriately.
func (o *Organization) AddRepositories(ctx context.Context, exec boil.ContextExecutor, insert bool, related ...*Repository) error {
	var err error
	for _, rel := range related {
		if insert {
			rel.OrganizationID = o.ID
			if err = rel.Insert(ctx, exec, boil.Infer()); err != nil {
				return errors.Wrap(err, "failed to insert into foreign table")
			}
		} else {
			updateQuery := fmt.Sprintf(
				"UPDATE \"repositories\" SET %s WHERE %s",
				strmangle.SetParamNames("\"", "\"", 1, []string{"organization_id"}),
				strmangle.WhereClause("\"", "\"", 2, repositoryPrimaryKeyColumns),
			)
			values := []interface{}{o.ID, rel.ID}

			if boil.DebugMode {
				fmt.Fprintln(boil.DebugWriter, updateQuery)
				fmt.Fprintln(boil.DebugWriter, values)
			}

			if _, err = exec.ExecContext(ctx, updateQuery, values...); err != nil {
				return errors.Wrap(err, "failed to update foreign table")
			}

			rel.OrganizationID = o.ID
		}
	}

	if o.R == nil {
		o.R = &organizationR{
			Repositories: related,
		}
	} else {
		o.R.Repositories = append(o.R.Repositories, related...)
	}

	for _, rel := range related {
		if rel.R == nil {
			rel.R = &repositoryR{
				Organization: o,
			}
		} else {
			rel.R.Organization = o
		}
	}
	return nil
}

// AddTopics adds the given related objects to the existing relationships
// of the organization, optionally inserting them as new records.
// Appends related to o.R.Topics.
// Sets related.R.Organization appropriately.
func (o *Organization) AddTopics(ctx context.Context, exec boil.ContextExecutor, insert bool, related ...*Topic) error {
	var err error
	for _, rel := range related {
		if insert {
			rel.OrganizationID = o.ID
			if err = rel.Insert(ctx, exec, boil.Infer()); err != nil {
				return errors.Wrap(err, "failed to insert into foreign table")
			}
		} else {
			updateQuery := fmt.Sprintf(
				"UPDATE \"topics\" SET %s WHERE %s",
				strmangle.SetParamNames("\"", "\"", 1, []string{"organization_id"}),
				strmangle.WhereClause("\"", "\"", 2, topicPrimaryKeyColumns),
			)
			values := []interface{}{o.ID, rel.ID}

			if boil.DebugMode {
				fmt.Fprintln(boil.DebugWriter, updateQuery)
				fmt.Fprintln(boil.DebugWriter, values)
			}

			if _, err = exec.ExecContext(ctx, updateQuery, values...); err != nil {
				return errors.Wrap(err, "failed to update foreign table")
			}

			rel.OrganizationID = o.ID
		}
	}

	if o.R == nil {
		o.R = &organizationR{
			Topics: related,
		}
	} else {
		o.R.Topics = append(o.R.Topics, related...)
	}

	for _, rel := range related {
		if rel.R == nil {
			rel.R = &topicR{
				Organization: o,
			}
		} else {
			rel.R.Organization = o
		}
	}
	return nil
}

// Organizations retrieves all the records using an executor.
func Organizations(mods ...qm.QueryMod) organizationQuery {
	mods = append(mods, qm.From("\"organizations\""))
	return organizationQuery{NewQuery(mods...)}
}

// FindOrganization retrieves a single record by ID with an executor.
// If selectCols is empty Find will return all columns.
func FindOrganization(ctx context.Context, exec boil.ContextExecutor, iD string, selectCols ...string) (*Organization, error) {
	organizationObj := &Organization{}

	sel := "*"
	if len(selectCols) > 0 {
		sel = strings.Join(strmangle.IdentQuoteSlice(dialect.LQ, dialect.RQ, selectCols), ",")
	}
	query := fmt.Sprintf(
		"select %s from \"organizations\" where \"id\"=$1", sel,
	)

	q := queries.Raw(query, iD)

	err := q.Bind(ctx, exec, organizationObj)
	if err != nil {
		if errors.Cause(err) == sql.ErrNoRows {
			return nil, sql.ErrNoRows
		}
		return nil, errors.Wrap(err, "models: unable to select from organizations")
	}

	return organizationObj, nil
}

// Insert a single record using an executor.
// See boil.Columns.InsertColumnSet documentation to understand column list inference for inserts.
func (o *Organization) Insert(ctx context.Context, exec boil.ContextExecutor, columns boil.Columns) error {
	if o == nil {
		return errors.New("models: no organizations provided for insertion")
	}

	var err error
	currTime := time.Now().In(boil.GetLocation())

	if o.CreatedAt.IsZero() {
		o.CreatedAt = currTime
	}
	if o.UpdatedAt.IsZero() {
		o.UpdatedAt = currTime
	}

	if err := o.doBeforeInsertHooks(ctx, exec); err != nil {
		return err
	}

	nzDefaults := queries.NonZeroDefaultSet(organizationColumnsWithDefault, o)

	key := makeCacheKey(columns, nzDefaults)
	organizationInsertCacheMut.RLock()
	cache, cached := organizationInsertCache[key]
	organizationInsertCacheMut.RUnlock()

	if !cached {
		wl, returnColumns := columns.InsertColumnSet(
			organizationColumns,
			organizationColumnsWithDefault,
			organizationColumnsWithoutDefault,
			nzDefaults,
		)

		cache.valueMapping, err = queries.BindMapping(organizationType, organizationMapping, wl)
		if err != nil {
			return err
		}
		cache.retMapping, err = queries.BindMapping(organizationType, organizationMapping, returnColumns)
		if err != nil {
			return err
		}
		if len(wl) != 0 {
			cache.query = fmt.Sprintf("INSERT INTO \"organizations\" (\"%s\") %%sVALUES (%s)%%s", strings.Join(wl, "\",\""), strmangle.Placeholders(dialect.UseIndexPlaceholders, len(wl), 1, 1))
		} else {
			cache.query = "INSERT INTO \"organizations\" %sDEFAULT VALUES%s"
		}

		var queryOutput, queryReturning string

		if len(cache.retMapping) != 0 {
			queryReturning = fmt.Sprintf(" RETURNING \"%s\"", strings.Join(returnColumns, "\",\""))
		}

		cache.query = fmt.Sprintf(cache.query, queryOutput, queryReturning)
	}

	value := reflect.Indirect(reflect.ValueOf(o))
	vals := queries.ValuesFromMapping(value, cache.valueMapping)

	if boil.DebugMode {
		fmt.Fprintln(boil.DebugWriter, cache.query)
		fmt.Fprintln(boil.DebugWriter, vals)
	}

	if len(cache.retMapping) != 0 {
		err = exec.QueryRowContext(ctx, cache.query, vals...).Scan(queries.PtrsFromMapping(value, cache.retMapping)...)
	} else {
		_, err = exec.ExecContext(ctx, cache.query, vals...)
	}

	if err != nil {
		return errors.Wrap(err, "models: unable to insert into organizations")
	}

	if !cached {
		organizationInsertCacheMut.Lock()
		organizationInsertCache[key] = cache
		organizationInsertCacheMut.Unlock()
	}

	return o.doAfterInsertHooks(ctx, exec)
}

// Update uses an executor to update the Organization.
// See boil.Columns.UpdateColumnSet documentation to understand column list inference for updates.
// Update does not automatically update the record in case of default values. Use .Reload() to refresh the records.
func (o *Organization) Update(ctx context.Context, exec boil.ContextExecutor, columns boil.Columns) (int64, error) {
	currTime := time.Now().In(boil.GetLocation())

	o.UpdatedAt = currTime

	var err error
	if err = o.doBeforeUpdateHooks(ctx, exec); err != nil {
		return 0, err
	}
	key := makeCacheKey(columns, nil)
	organizationUpdateCacheMut.RLock()
	cache, cached := organizationUpdateCache[key]
	organizationUpdateCacheMut.RUnlock()

	if !cached {
		wl := columns.UpdateColumnSet(
			organizationColumns,
			organizationPrimaryKeyColumns,
		)

		if !columns.IsWhitelist() {
			wl = strmangle.SetComplement(wl, []string{"created_at"})
		}
		if len(wl) == 0 {
			return 0, errors.New("models: unable to update organizations, could not build whitelist")
		}

		cache.query = fmt.Sprintf("UPDATE \"organizations\" SET %s WHERE %s",
			strmangle.SetParamNames("\"", "\"", 1, wl),
			strmangle.WhereClause("\"", "\"", len(wl)+1, organizationPrimaryKeyColumns),
		)
		cache.valueMapping, err = queries.BindMapping(organizationType, organizationMapping, append(wl, organizationPrimaryKeyColumns...))
		if err != nil {
			return 0, err
		}
	}

	values := queries.ValuesFromMapping(reflect.Indirect(reflect.ValueOf(o)), cache.valueMapping)

	if boil.DebugMode {
		fmt.Fprintln(boil.DebugWriter, cache.query)
		fmt.Fprintln(boil.DebugWriter, values)
	}

	var result sql.Result
	result, err = exec.ExecContext(ctx, cache.query, values...)
	if err != nil {
		return 0, errors.Wrap(err, "models: unable to update organizations row")
	}

	rowsAff, err := result.RowsAffected()
	if err != nil {
		return 0, errors.Wrap(err, "models: failed to get rows affected by update for organizations")
	}

	if !cached {
		organizationUpdateCacheMut.Lock()
		organizationUpdateCache[key] = cache
		organizationUpdateCacheMut.Unlock()
	}

	return rowsAff, o.doAfterUpdateHooks(ctx, exec)
}

// UpdateAll updates all rows with the specified column values.
func (q organizationQuery) UpdateAll(ctx context.Context, exec boil.ContextExecutor, cols M) (int64, error) {
	queries.SetUpdate(q.Query, cols)

	result, err := q.Query.ExecContext(ctx, exec)
	if err != nil {
		return 0, errors.Wrap(err, "models: unable to update all for organizations")
	}

	rowsAff, err := result.RowsAffected()
	if err != nil {
		return 0, errors.Wrap(err, "models: unable to retrieve rows affected for organizations")
	}

	return rowsAff, nil
}

// UpdateAll updates all rows with the specified column values, using an executor.
func (o OrganizationSlice) UpdateAll(ctx context.Context, exec boil.ContextExecutor, cols M) (int64, error) {
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
		pkeyArgs := queries.ValuesFromMapping(reflect.Indirect(reflect.ValueOf(obj)), organizationPrimaryKeyMapping)
		args = append(args, pkeyArgs...)
	}

	sql := fmt.Sprintf("UPDATE \"organizations\" SET %s WHERE %s",
		strmangle.SetParamNames("\"", "\"", 1, colNames),
		strmangle.WhereClauseRepeated(string(dialect.LQ), string(dialect.RQ), len(colNames)+1, organizationPrimaryKeyColumns, len(o)))

	if boil.DebugMode {
		fmt.Fprintln(boil.DebugWriter, sql)
		fmt.Fprintln(boil.DebugWriter, args...)
	}

	result, err := exec.ExecContext(ctx, sql, args...)
	if err != nil {
		return 0, errors.Wrap(err, "models: unable to update all in organization slice")
	}

	rowsAff, err := result.RowsAffected()
	if err != nil {
		return 0, errors.Wrap(err, "models: unable to retrieve rows affected all in update all organization")
	}
	return rowsAff, nil
}

// Upsert attempts an insert using an executor, and does an update or ignore on conflict.
// See boil.Columns documentation for how to properly use updateColumns and insertColumns.
func (o *Organization) Upsert(ctx context.Context, exec boil.ContextExecutor, updateOnConflict bool, conflictColumns []string, updateColumns, insertColumns boil.Columns) error {
	if o == nil {
		return errors.New("models: no organizations provided for upsert")
	}
	currTime := time.Now().In(boil.GetLocation())

	if o.CreatedAt.IsZero() {
		o.CreatedAt = currTime
	}
	o.UpdatedAt = currTime

	if err := o.doBeforeUpsertHooks(ctx, exec); err != nil {
		return err
	}

	nzDefaults := queries.NonZeroDefaultSet(organizationColumnsWithDefault, o)

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

	organizationUpsertCacheMut.RLock()
	cache, cached := organizationUpsertCache[key]
	organizationUpsertCacheMut.RUnlock()

	var err error

	if !cached {
		insert, ret := insertColumns.InsertColumnSet(
			organizationColumns,
			organizationColumnsWithDefault,
			organizationColumnsWithoutDefault,
			nzDefaults,
		)
		update := updateColumns.UpdateColumnSet(
			organizationColumns,
			organizationPrimaryKeyColumns,
		)

		if len(update) == 0 {
			return errors.New("models: unable to upsert organizations, could not build update column list")
		}

		conflict := conflictColumns
		if len(conflict) == 0 {
			conflict = make([]string, len(organizationPrimaryKeyColumns))
			copy(conflict, organizationPrimaryKeyColumns)
		}
		cache.query = buildUpsertQueryPostgres(dialect, "\"organizations\"", updateOnConflict, ret, update, conflict, insert)

		cache.valueMapping, err = queries.BindMapping(organizationType, organizationMapping, insert)
		if err != nil {
			return err
		}
		if len(ret) != 0 {
			cache.retMapping, err = queries.BindMapping(organizationType, organizationMapping, ret)
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

	if boil.DebugMode {
		fmt.Fprintln(boil.DebugWriter, cache.query)
		fmt.Fprintln(boil.DebugWriter, vals)
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
		return errors.Wrap(err, "models: unable to upsert organizations")
	}

	if !cached {
		organizationUpsertCacheMut.Lock()
		organizationUpsertCache[key] = cache
		organizationUpsertCacheMut.Unlock()
	}

	return o.doAfterUpsertHooks(ctx, exec)
}

// Delete deletes a single Organization record with an executor.
// Delete will match against the primary key column to find the record to delete.
func (o *Organization) Delete(ctx context.Context, exec boil.ContextExecutor) (int64, error) {
	if o == nil {
		return 0, errors.New("models: no Organization provided for delete")
	}

	if err := o.doBeforeDeleteHooks(ctx, exec); err != nil {
		return 0, err
	}

	args := queries.ValuesFromMapping(reflect.Indirect(reflect.ValueOf(o)), organizationPrimaryKeyMapping)
	sql := "DELETE FROM \"organizations\" WHERE \"id\"=$1"

	if boil.DebugMode {
		fmt.Fprintln(boil.DebugWriter, sql)
		fmt.Fprintln(boil.DebugWriter, args...)
	}

	result, err := exec.ExecContext(ctx, sql, args...)
	if err != nil {
		return 0, errors.Wrap(err, "models: unable to delete from organizations")
	}

	rowsAff, err := result.RowsAffected()
	if err != nil {
		return 0, errors.Wrap(err, "models: failed to get rows affected by delete for organizations")
	}

	if err := o.doAfterDeleteHooks(ctx, exec); err != nil {
		return 0, err
	}

	return rowsAff, nil
}

// DeleteAll deletes all matching rows.
func (q organizationQuery) DeleteAll(ctx context.Context, exec boil.ContextExecutor) (int64, error) {
	if q.Query == nil {
		return 0, errors.New("models: no organizationQuery provided for delete all")
	}

	queries.SetDelete(q.Query)

	result, err := q.Query.ExecContext(ctx, exec)
	if err != nil {
		return 0, errors.Wrap(err, "models: unable to delete all from organizations")
	}

	rowsAff, err := result.RowsAffected()
	if err != nil {
		return 0, errors.Wrap(err, "models: failed to get rows affected by deleteall for organizations")
	}

	return rowsAff, nil
}

// DeleteAll deletes all rows in the slice, using an executor.
func (o OrganizationSlice) DeleteAll(ctx context.Context, exec boil.ContextExecutor) (int64, error) {
	if o == nil {
		return 0, errors.New("models: no Organization slice provided for delete all")
	}

	if len(o) == 0 {
		return 0, nil
	}

	if len(organizationBeforeDeleteHooks) != 0 {
		for _, obj := range o {
			if err := obj.doBeforeDeleteHooks(ctx, exec); err != nil {
				return 0, err
			}
		}
	}

	var args []interface{}
	for _, obj := range o {
		pkeyArgs := queries.ValuesFromMapping(reflect.Indirect(reflect.ValueOf(obj)), organizationPrimaryKeyMapping)
		args = append(args, pkeyArgs...)
	}

	sql := "DELETE FROM \"organizations\" WHERE " +
		strmangle.WhereClauseRepeated(string(dialect.LQ), string(dialect.RQ), 1, organizationPrimaryKeyColumns, len(o))

	if boil.DebugMode {
		fmt.Fprintln(boil.DebugWriter, sql)
		fmt.Fprintln(boil.DebugWriter, args)
	}

	result, err := exec.ExecContext(ctx, sql, args...)
	if err != nil {
		return 0, errors.Wrap(err, "models: unable to delete all from organization slice")
	}

	rowsAff, err := result.RowsAffected()
	if err != nil {
		return 0, errors.Wrap(err, "models: failed to get rows affected by deleteall for organizations")
	}

	if len(organizationAfterDeleteHooks) != 0 {
		for _, obj := range o {
			if err := obj.doAfterDeleteHooks(ctx, exec); err != nil {
				return 0, err
			}
		}
	}

	return rowsAff, nil
}

// Reload refetches the object from the database
// using the primary keys with an executor.
func (o *Organization) Reload(ctx context.Context, exec boil.ContextExecutor) error {
	ret, err := FindOrganization(ctx, exec, o.ID)
	if err != nil {
		return err
	}

	*o = *ret
	return nil
}

// ReloadAll refetches every row with matching primary key column values
// and overwrites the original object slice with the newly updated slice.
func (o *OrganizationSlice) ReloadAll(ctx context.Context, exec boil.ContextExecutor) error {
	if o == nil || len(*o) == 0 {
		return nil
	}

	slice := OrganizationSlice{}
	var args []interface{}
	for _, obj := range *o {
		pkeyArgs := queries.ValuesFromMapping(reflect.Indirect(reflect.ValueOf(obj)), organizationPrimaryKeyMapping)
		args = append(args, pkeyArgs...)
	}

	sql := "SELECT \"organizations\".* FROM \"organizations\" WHERE " +
		strmangle.WhereClauseRepeated(string(dialect.LQ), string(dialect.RQ), 1, organizationPrimaryKeyColumns, len(*o))

	q := queries.Raw(sql, args...)

	err := q.Bind(ctx, exec, &slice)
	if err != nil {
		return errors.Wrap(err, "models: unable to reload all in OrganizationSlice")
	}

	*o = slice

	return nil
}

// OrganizationExists checks if the Organization row exists.
func OrganizationExists(ctx context.Context, exec boil.ContextExecutor, iD string) (bool, error) {
	var exists bool
	sql := "select exists(select 1 from \"organizations\" where \"id\"=$1 limit 1)"

	if boil.DebugMode {
		fmt.Fprintln(boil.DebugWriter, sql)
		fmt.Fprintln(boil.DebugWriter, iD)
	}

	row := exec.QueryRowContext(ctx, sql, iD)

	err := row.Scan(&exists)
	if err != nil {
		return false, errors.Wrap(err, "models: unable to check if organizations exists")
	}

	return exists, nil
}
