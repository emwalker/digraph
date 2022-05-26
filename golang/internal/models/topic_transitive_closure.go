// Code generated by SQLBoiler 4.11.0 (https://github.com/volatiletech/sqlboiler). DO NOT EDIT.
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
	"github.com/volatiletech/sqlboiler/v4/boil"
	"github.com/volatiletech/sqlboiler/v4/queries"
	"github.com/volatiletech/sqlboiler/v4/queries/qm"
	"github.com/volatiletech/sqlboiler/v4/queries/qmhelper"
	"github.com/volatiletech/strmangle"
)

// TopicTransitiveClosure is an object representing the database table.
type TopicTransitiveClosure struct {
	ID       int    `boil:"id" json:"id" toml:"id" yaml:"id"`
	ParentID string `boil:"parent_id" json:"parent_id" toml:"parent_id" yaml:"parent_id"`
	ChildID  string `boil:"child_id" json:"child_id" toml:"child_id" yaml:"child_id"`

	R *topicTransitiveClosureR `boil:"-" json:"-" toml:"-" yaml:"-"`
	L topicTransitiveClosureL  `boil:"-" json:"-" toml:"-" yaml:"-"`
}

var TopicTransitiveClosureColumns = struct {
	ID       string
	ParentID string
	ChildID  string
}{
	ID:       "id",
	ParentID: "parent_id",
	ChildID:  "child_id",
}

var TopicTransitiveClosureTableColumns = struct {
	ID       string
	ParentID string
	ChildID  string
}{
	ID:       "topic_transitive_closure.id",
	ParentID: "topic_transitive_closure.parent_id",
	ChildID:  "topic_transitive_closure.child_id",
}

// Generated where

var TopicTransitiveClosureWhere = struct {
	ID       whereHelperint
	ParentID whereHelperstring
	ChildID  whereHelperstring
}{
	ID:       whereHelperint{field: "\"topic_transitive_closure\".\"id\""},
	ParentID: whereHelperstring{field: "\"topic_transitive_closure\".\"parent_id\""},
	ChildID:  whereHelperstring{field: "\"topic_transitive_closure\".\"child_id\""},
}

// TopicTransitiveClosureRels is where relationship names are stored.
var TopicTransitiveClosureRels = struct {
	Child  string
	Parent string
}{
	Child:  "Child",
	Parent: "Parent",
}

// topicTransitiveClosureR is where relationships are stored.
type topicTransitiveClosureR struct {
	Child  *Topic `boil:"Child" json:"Child" toml:"Child" yaml:"Child"`
	Parent *Topic `boil:"Parent" json:"Parent" toml:"Parent" yaml:"Parent"`
}

// NewStruct creates a new relationship struct
func (*topicTransitiveClosureR) NewStruct() *topicTransitiveClosureR {
	return &topicTransitiveClosureR{}
}

func (r *topicTransitiveClosureR) GetChild() *Topic {
	if r == nil {
		return nil
	}
	return r.Child
}

func (r *topicTransitiveClosureR) GetParent() *Topic {
	if r == nil {
		return nil
	}
	return r.Parent
}

// topicTransitiveClosureL is where Load methods for each relationship are stored.
type topicTransitiveClosureL struct{}

var (
	topicTransitiveClosureAllColumns            = []string{"id", "parent_id", "child_id"}
	topicTransitiveClosureColumnsWithoutDefault = []string{"parent_id", "child_id"}
	topicTransitiveClosureColumnsWithDefault    = []string{"id"}
	topicTransitiveClosurePrimaryKeyColumns     = []string{"id"}
	topicTransitiveClosureGeneratedColumns      = []string{}
)

type (
	// TopicTransitiveClosureSlice is an alias for a slice of pointers to TopicTransitiveClosure.
	// This should almost always be used instead of []TopicTransitiveClosure.
	TopicTransitiveClosureSlice []*TopicTransitiveClosure

	topicTransitiveClosureQuery struct {
		*queries.Query
	}
)

// Cache for insert, update and upsert
var (
	topicTransitiveClosureType                 = reflect.TypeOf(&TopicTransitiveClosure{})
	topicTransitiveClosureMapping              = queries.MakeStructMapping(topicTransitiveClosureType)
	topicTransitiveClosurePrimaryKeyMapping, _ = queries.BindMapping(topicTransitiveClosureType, topicTransitiveClosureMapping, topicTransitiveClosurePrimaryKeyColumns)
	topicTransitiveClosureInsertCacheMut       sync.RWMutex
	topicTransitiveClosureInsertCache          = make(map[string]insertCache)
	topicTransitiveClosureUpdateCacheMut       sync.RWMutex
	topicTransitiveClosureUpdateCache          = make(map[string]updateCache)
	topicTransitiveClosureUpsertCacheMut       sync.RWMutex
	topicTransitiveClosureUpsertCache          = make(map[string]insertCache)
)

var (
	// Force time package dependency for automated UpdatedAt/CreatedAt.
	_ = time.Second
	// Force qmhelper dependency for where clause generation (which doesn't
	// always happen)
	_ = qmhelper.Where
)

// One returns a single topicTransitiveClosure record from the query.
func (q topicTransitiveClosureQuery) One(ctx context.Context, exec boil.ContextExecutor) (*TopicTransitiveClosure, error) {
	o := &TopicTransitiveClosure{}

	queries.SetLimit(q.Query, 1)

	err := q.Bind(ctx, exec, o)
	if err != nil {
		if errors.Is(err, sql.ErrNoRows) {
			return nil, sql.ErrNoRows
		}
		return nil, errors.Wrap(err, "models: failed to execute a one query for topic_transitive_closure")
	}

	return o, nil
}

// All returns all TopicTransitiveClosure records from the query.
func (q topicTransitiveClosureQuery) All(ctx context.Context, exec boil.ContextExecutor) (TopicTransitiveClosureSlice, error) {
	var o []*TopicTransitiveClosure

	err := q.Bind(ctx, exec, &o)
	if err != nil {
		return nil, errors.Wrap(err, "models: failed to assign all query results to TopicTransitiveClosure slice")
	}

	return o, nil
}

// Count returns the count of all TopicTransitiveClosure records in the query.
func (q topicTransitiveClosureQuery) Count(ctx context.Context, exec boil.ContextExecutor) (int64, error) {
	var count int64

	queries.SetSelect(q.Query, nil)
	queries.SetCount(q.Query)

	err := q.Query.QueryRowContext(ctx, exec).Scan(&count)
	if err != nil {
		return 0, errors.Wrap(err, "models: failed to count topic_transitive_closure rows")
	}

	return count, nil
}

// Exists checks if the row exists in the table.
func (q topicTransitiveClosureQuery) Exists(ctx context.Context, exec boil.ContextExecutor) (bool, error) {
	var count int64

	queries.SetSelect(q.Query, nil)
	queries.SetCount(q.Query)
	queries.SetLimit(q.Query, 1)

	err := q.Query.QueryRowContext(ctx, exec).Scan(&count)
	if err != nil {
		return false, errors.Wrap(err, "models: failed to check if topic_transitive_closure exists")
	}

	return count > 0, nil
}

// Child pointed to by the foreign key.
func (o *TopicTransitiveClosure) Child(mods ...qm.QueryMod) topicQuery {
	queryMods := []qm.QueryMod{
		qm.Where("\"id\" = ?", o.ChildID),
	}

	queryMods = append(queryMods, mods...)

	return Topics(queryMods...)
}

// Parent pointed to by the foreign key.
func (o *TopicTransitiveClosure) Parent(mods ...qm.QueryMod) topicQuery {
	queryMods := []qm.QueryMod{
		qm.Where("\"id\" = ?", o.ParentID),
	}

	queryMods = append(queryMods, mods...)

	return Topics(queryMods...)
}

// LoadChild allows an eager lookup of values, cached into the
// loaded structs of the objects. This is for an N-1 relationship.
func (topicTransitiveClosureL) LoadChild(ctx context.Context, e boil.ContextExecutor, singular bool, maybeTopicTransitiveClosure interface{}, mods queries.Applicator) error {
	var slice []*TopicTransitiveClosure
	var object *TopicTransitiveClosure

	if singular {
		object = maybeTopicTransitiveClosure.(*TopicTransitiveClosure)
	} else {
		slice = *maybeTopicTransitiveClosure.(*[]*TopicTransitiveClosure)
	}

	args := make([]interface{}, 0, 1)
	if singular {
		if object.R == nil {
			object.R = &topicTransitiveClosureR{}
		}
		args = append(args, object.ChildID)

	} else {
	Outer:
		for _, obj := range slice {
			if obj.R == nil {
				obj.R = &topicTransitiveClosureR{}
			}

			for _, a := range args {
				if a == obj.ChildID {
					continue Outer
				}
			}

			args = append(args, obj.ChildID)

		}
	}

	if len(args) == 0 {
		return nil
	}

	query := NewQuery(
		qm.From(`topics`),
		qm.WhereIn(`topics.id in ?`, args...),
	)
	if mods != nil {
		mods.Apply(query)
	}

	results, err := query.QueryContext(ctx, e)
	if err != nil {
		return errors.Wrap(err, "failed to eager load Topic")
	}

	var resultSlice []*Topic
	if err = queries.Bind(results, &resultSlice); err != nil {
		return errors.Wrap(err, "failed to bind eager loaded slice Topic")
	}

	if err = results.Close(); err != nil {
		return errors.Wrap(err, "failed to close results of eager load for topics")
	}
	if err = results.Err(); err != nil {
		return errors.Wrap(err, "error occurred during iteration of eager loaded relations for topics")
	}

	if len(resultSlice) == 0 {
		return nil
	}

	if singular {
		foreign := resultSlice[0]
		object.R.Child = foreign
		if foreign.R == nil {
			foreign.R = &topicR{}
		}
		foreign.R.ChildTopicTransitiveClosures = append(foreign.R.ChildTopicTransitiveClosures, object)
		return nil
	}

	for _, local := range slice {
		for _, foreign := range resultSlice {
			if local.ChildID == foreign.ID {
				local.R.Child = foreign
				if foreign.R == nil {
					foreign.R = &topicR{}
				}
				foreign.R.ChildTopicTransitiveClosures = append(foreign.R.ChildTopicTransitiveClosures, local)
				break
			}
		}
	}

	return nil
}

// LoadParent allows an eager lookup of values, cached into the
// loaded structs of the objects. This is for an N-1 relationship.
func (topicTransitiveClosureL) LoadParent(ctx context.Context, e boil.ContextExecutor, singular bool, maybeTopicTransitiveClosure interface{}, mods queries.Applicator) error {
	var slice []*TopicTransitiveClosure
	var object *TopicTransitiveClosure

	if singular {
		object = maybeTopicTransitiveClosure.(*TopicTransitiveClosure)
	} else {
		slice = *maybeTopicTransitiveClosure.(*[]*TopicTransitiveClosure)
	}

	args := make([]interface{}, 0, 1)
	if singular {
		if object.R == nil {
			object.R = &topicTransitiveClosureR{}
		}
		args = append(args, object.ParentID)

	} else {
	Outer:
		for _, obj := range slice {
			if obj.R == nil {
				obj.R = &topicTransitiveClosureR{}
			}

			for _, a := range args {
				if a == obj.ParentID {
					continue Outer
				}
			}

			args = append(args, obj.ParentID)

		}
	}

	if len(args) == 0 {
		return nil
	}

	query := NewQuery(
		qm.From(`topics`),
		qm.WhereIn(`topics.id in ?`, args...),
	)
	if mods != nil {
		mods.Apply(query)
	}

	results, err := query.QueryContext(ctx, e)
	if err != nil {
		return errors.Wrap(err, "failed to eager load Topic")
	}

	var resultSlice []*Topic
	if err = queries.Bind(results, &resultSlice); err != nil {
		return errors.Wrap(err, "failed to bind eager loaded slice Topic")
	}

	if err = results.Close(); err != nil {
		return errors.Wrap(err, "failed to close results of eager load for topics")
	}
	if err = results.Err(); err != nil {
		return errors.Wrap(err, "error occurred during iteration of eager loaded relations for topics")
	}

	if len(resultSlice) == 0 {
		return nil
	}

	if singular {
		foreign := resultSlice[0]
		object.R.Parent = foreign
		if foreign.R == nil {
			foreign.R = &topicR{}
		}
		foreign.R.ParentTopicTransitiveClosures = append(foreign.R.ParentTopicTransitiveClosures, object)
		return nil
	}

	for _, local := range slice {
		for _, foreign := range resultSlice {
			if local.ParentID == foreign.ID {
				local.R.Parent = foreign
				if foreign.R == nil {
					foreign.R = &topicR{}
				}
				foreign.R.ParentTopicTransitiveClosures = append(foreign.R.ParentTopicTransitiveClosures, local)
				break
			}
		}
	}

	return nil
}

// SetChild of the topicTransitiveClosure to the related item.
// Sets o.R.Child to related.
// Adds o to related.R.ChildTopicTransitiveClosures.
func (o *TopicTransitiveClosure) SetChild(ctx context.Context, exec boil.ContextExecutor, insert bool, related *Topic) error {
	var err error
	if insert {
		if err = related.Insert(ctx, exec, boil.Infer()); err != nil {
			return errors.Wrap(err, "failed to insert into foreign table")
		}
	}

	updateQuery := fmt.Sprintf(
		"UPDATE \"topic_transitive_closure\" SET %s WHERE %s",
		strmangle.SetParamNames("\"", "\"", 1, []string{"child_id"}),
		strmangle.WhereClause("\"", "\"", 2, topicTransitiveClosurePrimaryKeyColumns),
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

	o.ChildID = related.ID
	if o.R == nil {
		o.R = &topicTransitiveClosureR{
			Child: related,
		}
	} else {
		o.R.Child = related
	}

	if related.R == nil {
		related.R = &topicR{
			ChildTopicTransitiveClosures: TopicTransitiveClosureSlice{o},
		}
	} else {
		related.R.ChildTopicTransitiveClosures = append(related.R.ChildTopicTransitiveClosures, o)
	}

	return nil
}

// SetParent of the topicTransitiveClosure to the related item.
// Sets o.R.Parent to related.
// Adds o to related.R.ParentTopicTransitiveClosures.
func (o *TopicTransitiveClosure) SetParent(ctx context.Context, exec boil.ContextExecutor, insert bool, related *Topic) error {
	var err error
	if insert {
		if err = related.Insert(ctx, exec, boil.Infer()); err != nil {
			return errors.Wrap(err, "failed to insert into foreign table")
		}
	}

	updateQuery := fmt.Sprintf(
		"UPDATE \"topic_transitive_closure\" SET %s WHERE %s",
		strmangle.SetParamNames("\"", "\"", 1, []string{"parent_id"}),
		strmangle.WhereClause("\"", "\"", 2, topicTransitiveClosurePrimaryKeyColumns),
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

	o.ParentID = related.ID
	if o.R == nil {
		o.R = &topicTransitiveClosureR{
			Parent: related,
		}
	} else {
		o.R.Parent = related
	}

	if related.R == nil {
		related.R = &topicR{
			ParentTopicTransitiveClosures: TopicTransitiveClosureSlice{o},
		}
	} else {
		related.R.ParentTopicTransitiveClosures = append(related.R.ParentTopicTransitiveClosures, o)
	}

	return nil
}

// TopicTransitiveClosures retrieves all the records using an executor.
func TopicTransitiveClosures(mods ...qm.QueryMod) topicTransitiveClosureQuery {
	mods = append(mods, qm.From("\"topic_transitive_closure\""))
	q := NewQuery(mods...)
	if len(queries.GetSelect(q)) == 0 {
		queries.SetSelect(q, []string{"\"topic_transitive_closure\".*"})
	}

	return topicTransitiveClosureQuery{q}
}

// FindTopicTransitiveClosure retrieves a single record by ID with an executor.
// If selectCols is empty Find will return all columns.
func FindTopicTransitiveClosure(ctx context.Context, exec boil.ContextExecutor, iD int, selectCols ...string) (*TopicTransitiveClosure, error) {
	topicTransitiveClosureObj := &TopicTransitiveClosure{}

	sel := "*"
	if len(selectCols) > 0 {
		sel = strings.Join(strmangle.IdentQuoteSlice(dialect.LQ, dialect.RQ, selectCols), ",")
	}
	query := fmt.Sprintf(
		"select %s from \"topic_transitive_closure\" where \"id\"=$1", sel,
	)

	q := queries.Raw(query, iD)

	err := q.Bind(ctx, exec, topicTransitiveClosureObj)
	if err != nil {
		if errors.Is(err, sql.ErrNoRows) {
			return nil, sql.ErrNoRows
		}
		return nil, errors.Wrap(err, "models: unable to select from topic_transitive_closure")
	}

	return topicTransitiveClosureObj, nil
}

// Insert a single record using an executor.
// See boil.Columns.InsertColumnSet documentation to understand column list inference for inserts.
func (o *TopicTransitiveClosure) Insert(ctx context.Context, exec boil.ContextExecutor, columns boil.Columns) error {
	if o == nil {
		return errors.New("models: no topic_transitive_closure provided for insertion")
	}

	var err error

	nzDefaults := queries.NonZeroDefaultSet(topicTransitiveClosureColumnsWithDefault, o)

	key := makeCacheKey(columns, nzDefaults)
	topicTransitiveClosureInsertCacheMut.RLock()
	cache, cached := topicTransitiveClosureInsertCache[key]
	topicTransitiveClosureInsertCacheMut.RUnlock()

	if !cached {
		wl, returnColumns := columns.InsertColumnSet(
			topicTransitiveClosureAllColumns,
			topicTransitiveClosureColumnsWithDefault,
			topicTransitiveClosureColumnsWithoutDefault,
			nzDefaults,
		)

		cache.valueMapping, err = queries.BindMapping(topicTransitiveClosureType, topicTransitiveClosureMapping, wl)
		if err != nil {
			return err
		}
		cache.retMapping, err = queries.BindMapping(topicTransitiveClosureType, topicTransitiveClosureMapping, returnColumns)
		if err != nil {
			return err
		}
		if len(wl) != 0 {
			cache.query = fmt.Sprintf("INSERT INTO \"topic_transitive_closure\" (\"%s\") %%sVALUES (%s)%%s", strings.Join(wl, "\",\""), strmangle.Placeholders(dialect.UseIndexPlaceholders, len(wl), 1, 1))
		} else {
			cache.query = "INSERT INTO \"topic_transitive_closure\" %sDEFAULT VALUES%s"
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
		return errors.Wrap(err, "models: unable to insert into topic_transitive_closure")
	}

	if !cached {
		topicTransitiveClosureInsertCacheMut.Lock()
		topicTransitiveClosureInsertCache[key] = cache
		topicTransitiveClosureInsertCacheMut.Unlock()
	}

	return nil
}

// Update uses an executor to update the TopicTransitiveClosure.
// See boil.Columns.UpdateColumnSet documentation to understand column list inference for updates.
// Update does not automatically update the record in case of default values. Use .Reload() to refresh the records.
func (o *TopicTransitiveClosure) Update(ctx context.Context, exec boil.ContextExecutor, columns boil.Columns) (int64, error) {
	var err error
	key := makeCacheKey(columns, nil)
	topicTransitiveClosureUpdateCacheMut.RLock()
	cache, cached := topicTransitiveClosureUpdateCache[key]
	topicTransitiveClosureUpdateCacheMut.RUnlock()

	if !cached {
		wl := columns.UpdateColumnSet(
			topicTransitiveClosureAllColumns,
			topicTransitiveClosurePrimaryKeyColumns,
		)

		if !columns.IsWhitelist() {
			wl = strmangle.SetComplement(wl, []string{"created_at"})
		}
		if len(wl) == 0 {
			return 0, errors.New("models: unable to update topic_transitive_closure, could not build whitelist")
		}

		cache.query = fmt.Sprintf("UPDATE \"topic_transitive_closure\" SET %s WHERE %s",
			strmangle.SetParamNames("\"", "\"", 1, wl),
			strmangle.WhereClause("\"", "\"", len(wl)+1, topicTransitiveClosurePrimaryKeyColumns),
		)
		cache.valueMapping, err = queries.BindMapping(topicTransitiveClosureType, topicTransitiveClosureMapping, append(wl, topicTransitiveClosurePrimaryKeyColumns...))
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
		return 0, errors.Wrap(err, "models: unable to update topic_transitive_closure row")
	}

	rowsAff, err := result.RowsAffected()
	if err != nil {
		return 0, errors.Wrap(err, "models: failed to get rows affected by update for topic_transitive_closure")
	}

	if !cached {
		topicTransitiveClosureUpdateCacheMut.Lock()
		topicTransitiveClosureUpdateCache[key] = cache
		topicTransitiveClosureUpdateCacheMut.Unlock()
	}

	return rowsAff, nil
}

// UpdateAll updates all rows with the specified column values.
func (q topicTransitiveClosureQuery) UpdateAll(ctx context.Context, exec boil.ContextExecutor, cols M) (int64, error) {
	queries.SetUpdate(q.Query, cols)

	result, err := q.Query.ExecContext(ctx, exec)
	if err != nil {
		return 0, errors.Wrap(err, "models: unable to update all for topic_transitive_closure")
	}

	rowsAff, err := result.RowsAffected()
	if err != nil {
		return 0, errors.Wrap(err, "models: unable to retrieve rows affected for topic_transitive_closure")
	}

	return rowsAff, nil
}

// UpdateAll updates all rows with the specified column values, using an executor.
func (o TopicTransitiveClosureSlice) UpdateAll(ctx context.Context, exec boil.ContextExecutor, cols M) (int64, error) {
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
		pkeyArgs := queries.ValuesFromMapping(reflect.Indirect(reflect.ValueOf(obj)), topicTransitiveClosurePrimaryKeyMapping)
		args = append(args, pkeyArgs...)
	}

	sql := fmt.Sprintf("UPDATE \"topic_transitive_closure\" SET %s WHERE %s",
		strmangle.SetParamNames("\"", "\"", 1, colNames),
		strmangle.WhereClauseRepeated(string(dialect.LQ), string(dialect.RQ), len(colNames)+1, topicTransitiveClosurePrimaryKeyColumns, len(o)))

	if boil.IsDebug(ctx) {
		writer := boil.DebugWriterFrom(ctx)
		fmt.Fprintln(writer, sql)
		fmt.Fprintln(writer, args...)
	}
	result, err := exec.ExecContext(ctx, sql, args...)
	if err != nil {
		return 0, errors.Wrap(err, "models: unable to update all in topicTransitiveClosure slice")
	}

	rowsAff, err := result.RowsAffected()
	if err != nil {
		return 0, errors.Wrap(err, "models: unable to retrieve rows affected all in update all topicTransitiveClosure")
	}
	return rowsAff, nil
}

// Upsert attempts an insert using an executor, and does an update or ignore on conflict.
// See boil.Columns documentation for how to properly use updateColumns and insertColumns.
func (o *TopicTransitiveClosure) Upsert(ctx context.Context, exec boil.ContextExecutor, updateOnConflict bool, conflictColumns []string, updateColumns, insertColumns boil.Columns) error {
	if o == nil {
		return errors.New("models: no topic_transitive_closure provided for upsert")
	}

	nzDefaults := queries.NonZeroDefaultSet(topicTransitiveClosureColumnsWithDefault, o)

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

	topicTransitiveClosureUpsertCacheMut.RLock()
	cache, cached := topicTransitiveClosureUpsertCache[key]
	topicTransitiveClosureUpsertCacheMut.RUnlock()

	var err error

	if !cached {
		insert, ret := insertColumns.InsertColumnSet(
			topicTransitiveClosureAllColumns,
			topicTransitiveClosureColumnsWithDefault,
			topicTransitiveClosureColumnsWithoutDefault,
			nzDefaults,
		)

		update := updateColumns.UpdateColumnSet(
			topicTransitiveClosureAllColumns,
			topicTransitiveClosurePrimaryKeyColumns,
		)

		if updateOnConflict && len(update) == 0 {
			return errors.New("models: unable to upsert topic_transitive_closure, could not build update column list")
		}

		conflict := conflictColumns
		if len(conflict) == 0 {
			conflict = make([]string, len(topicTransitiveClosurePrimaryKeyColumns))
			copy(conflict, topicTransitiveClosurePrimaryKeyColumns)
		}
		cache.query = buildUpsertQueryPostgres(dialect, "\"topic_transitive_closure\"", updateOnConflict, ret, update, conflict, insert)

		cache.valueMapping, err = queries.BindMapping(topicTransitiveClosureType, topicTransitiveClosureMapping, insert)
		if err != nil {
			return err
		}
		if len(ret) != 0 {
			cache.retMapping, err = queries.BindMapping(topicTransitiveClosureType, topicTransitiveClosureMapping, ret)
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
		if errors.Is(err, sql.ErrNoRows) {
			err = nil // Postgres doesn't return anything when there's no update
		}
	} else {
		_, err = exec.ExecContext(ctx, cache.query, vals...)
	}
	if err != nil {
		return errors.Wrap(err, "models: unable to upsert topic_transitive_closure")
	}

	if !cached {
		topicTransitiveClosureUpsertCacheMut.Lock()
		topicTransitiveClosureUpsertCache[key] = cache
		topicTransitiveClosureUpsertCacheMut.Unlock()
	}

	return nil
}

// Delete deletes a single TopicTransitiveClosure record with an executor.
// Delete will match against the primary key column to find the record to delete.
func (o *TopicTransitiveClosure) Delete(ctx context.Context, exec boil.ContextExecutor) (int64, error) {
	if o == nil {
		return 0, errors.New("models: no TopicTransitiveClosure provided for delete")
	}

	args := queries.ValuesFromMapping(reflect.Indirect(reflect.ValueOf(o)), topicTransitiveClosurePrimaryKeyMapping)
	sql := "DELETE FROM \"topic_transitive_closure\" WHERE \"id\"=$1"

	if boil.IsDebug(ctx) {
		writer := boil.DebugWriterFrom(ctx)
		fmt.Fprintln(writer, sql)
		fmt.Fprintln(writer, args...)
	}
	result, err := exec.ExecContext(ctx, sql, args...)
	if err != nil {
		return 0, errors.Wrap(err, "models: unable to delete from topic_transitive_closure")
	}

	rowsAff, err := result.RowsAffected()
	if err != nil {
		return 0, errors.Wrap(err, "models: failed to get rows affected by delete for topic_transitive_closure")
	}

	return rowsAff, nil
}

// DeleteAll deletes all matching rows.
func (q topicTransitiveClosureQuery) DeleteAll(ctx context.Context, exec boil.ContextExecutor) (int64, error) {
	if q.Query == nil {
		return 0, errors.New("models: no topicTransitiveClosureQuery provided for delete all")
	}

	queries.SetDelete(q.Query)

	result, err := q.Query.ExecContext(ctx, exec)
	if err != nil {
		return 0, errors.Wrap(err, "models: unable to delete all from topic_transitive_closure")
	}

	rowsAff, err := result.RowsAffected()
	if err != nil {
		return 0, errors.Wrap(err, "models: failed to get rows affected by deleteall for topic_transitive_closure")
	}

	return rowsAff, nil
}

// DeleteAll deletes all rows in the slice, using an executor.
func (o TopicTransitiveClosureSlice) DeleteAll(ctx context.Context, exec boil.ContextExecutor) (int64, error) {
	if len(o) == 0 {
		return 0, nil
	}

	var args []interface{}
	for _, obj := range o {
		pkeyArgs := queries.ValuesFromMapping(reflect.Indirect(reflect.ValueOf(obj)), topicTransitiveClosurePrimaryKeyMapping)
		args = append(args, pkeyArgs...)
	}

	sql := "DELETE FROM \"topic_transitive_closure\" WHERE " +
		strmangle.WhereClauseRepeated(string(dialect.LQ), string(dialect.RQ), 1, topicTransitiveClosurePrimaryKeyColumns, len(o))

	if boil.IsDebug(ctx) {
		writer := boil.DebugWriterFrom(ctx)
		fmt.Fprintln(writer, sql)
		fmt.Fprintln(writer, args)
	}
	result, err := exec.ExecContext(ctx, sql, args...)
	if err != nil {
		return 0, errors.Wrap(err, "models: unable to delete all from topicTransitiveClosure slice")
	}

	rowsAff, err := result.RowsAffected()
	if err != nil {
		return 0, errors.Wrap(err, "models: failed to get rows affected by deleteall for topic_transitive_closure")
	}

	return rowsAff, nil
}

// Reload refetches the object from the database
// using the primary keys with an executor.
func (o *TopicTransitiveClosure) Reload(ctx context.Context, exec boil.ContextExecutor) error {
	ret, err := FindTopicTransitiveClosure(ctx, exec, o.ID)
	if err != nil {
		return err
	}

	*o = *ret
	return nil
}

// ReloadAll refetches every row with matching primary key column values
// and overwrites the original object slice with the newly updated slice.
func (o *TopicTransitiveClosureSlice) ReloadAll(ctx context.Context, exec boil.ContextExecutor) error {
	if o == nil || len(*o) == 0 {
		return nil
	}

	slice := TopicTransitiveClosureSlice{}
	var args []interface{}
	for _, obj := range *o {
		pkeyArgs := queries.ValuesFromMapping(reflect.Indirect(reflect.ValueOf(obj)), topicTransitiveClosurePrimaryKeyMapping)
		args = append(args, pkeyArgs...)
	}

	sql := "SELECT \"topic_transitive_closure\".* FROM \"topic_transitive_closure\" WHERE " +
		strmangle.WhereClauseRepeated(string(dialect.LQ), string(dialect.RQ), 1, topicTransitiveClosurePrimaryKeyColumns, len(*o))

	q := queries.Raw(sql, args...)

	err := q.Bind(ctx, exec, &slice)
	if err != nil {
		return errors.Wrap(err, "models: unable to reload all in TopicTransitiveClosureSlice")
	}

	*o = slice

	return nil
}

// TopicTransitiveClosureExists checks if the TopicTransitiveClosure row exists.
func TopicTransitiveClosureExists(ctx context.Context, exec boil.ContextExecutor, iD int) (bool, error) {
	var exists bool
	sql := "select exists(select 1 from \"topic_transitive_closure\" where \"id\"=$1 limit 1)"

	if boil.IsDebug(ctx) {
		writer := boil.DebugWriterFrom(ctx)
		fmt.Fprintln(writer, sql)
		fmt.Fprintln(writer, iD)
	}
	row := exec.QueryRowContext(ctx, sql, iD)

	err := row.Scan(&exists)
	if err != nil {
		return false, errors.Wrap(err, "models: unable to check if topic_transitive_closure exists")
	}

	return exists, nil
}
