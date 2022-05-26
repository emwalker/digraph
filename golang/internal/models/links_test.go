// Code generated by SQLBoiler 4.11.0 (https://github.com/volatiletech/sqlboiler). DO NOT EDIT.
// This file is meant to be re-generated in place and/or deleted at any time.

package models

import (
	"bytes"
	"context"
	"reflect"
	"testing"

	"github.com/volatiletech/randomize"
	"github.com/volatiletech/sqlboiler/v4/boil"
	"github.com/volatiletech/sqlboiler/v4/queries"
	"github.com/volatiletech/strmangle"
)

var (
	// Relationships sometimes use the reflection helper queries.Equal/queries.Assign
	// so force a package dependency in case they don't.
	_ = queries.Equal
)

func testLinks(t *testing.T) {
	t.Parallel()

	query := Links()

	if query.Query == nil {
		t.Error("expected a query, got nothing")
	}
}

func testLinksDelete(t *testing.T) {
	t.Parallel()

	seed := randomize.NewSeed()
	var err error
	o := &Link{}
	if err = randomize.Struct(seed, o, linkDBTypes, true, linkColumnsWithDefault...); err != nil {
		t.Errorf("Unable to randomize Link struct: %s", err)
	}

	ctx := context.Background()
	tx := MustTx(boil.BeginTx(ctx, nil))
	defer func() { _ = tx.Rollback() }()
	if err = o.Insert(ctx, tx, boil.Infer()); err != nil {
		t.Error(err)
	}

	if rowsAff, err := o.Delete(ctx, tx); err != nil {
		t.Error(err)
	} else if rowsAff != 1 {
		t.Error("should only have deleted one row, but affected:", rowsAff)
	}

	count, err := Links().Count(ctx, tx)
	if err != nil {
		t.Error(err)
	}

	if count != 0 {
		t.Error("want zero records, got:", count)
	}
}

func testLinksQueryDeleteAll(t *testing.T) {
	t.Parallel()

	seed := randomize.NewSeed()
	var err error
	o := &Link{}
	if err = randomize.Struct(seed, o, linkDBTypes, true, linkColumnsWithDefault...); err != nil {
		t.Errorf("Unable to randomize Link struct: %s", err)
	}

	ctx := context.Background()
	tx := MustTx(boil.BeginTx(ctx, nil))
	defer func() { _ = tx.Rollback() }()
	if err = o.Insert(ctx, tx, boil.Infer()); err != nil {
		t.Error(err)
	}

	if rowsAff, err := Links().DeleteAll(ctx, tx); err != nil {
		t.Error(err)
	} else if rowsAff != 1 {
		t.Error("should only have deleted one row, but affected:", rowsAff)
	}

	count, err := Links().Count(ctx, tx)
	if err != nil {
		t.Error(err)
	}

	if count != 0 {
		t.Error("want zero records, got:", count)
	}
}

func testLinksSliceDeleteAll(t *testing.T) {
	t.Parallel()

	seed := randomize.NewSeed()
	var err error
	o := &Link{}
	if err = randomize.Struct(seed, o, linkDBTypes, true, linkColumnsWithDefault...); err != nil {
		t.Errorf("Unable to randomize Link struct: %s", err)
	}

	ctx := context.Background()
	tx := MustTx(boil.BeginTx(ctx, nil))
	defer func() { _ = tx.Rollback() }()
	if err = o.Insert(ctx, tx, boil.Infer()); err != nil {
		t.Error(err)
	}

	slice := LinkSlice{o}

	if rowsAff, err := slice.DeleteAll(ctx, tx); err != nil {
		t.Error(err)
	} else if rowsAff != 1 {
		t.Error("should only have deleted one row, but affected:", rowsAff)
	}

	count, err := Links().Count(ctx, tx)
	if err != nil {
		t.Error(err)
	}

	if count != 0 {
		t.Error("want zero records, got:", count)
	}
}

func testLinksExists(t *testing.T) {
	t.Parallel()

	seed := randomize.NewSeed()
	var err error
	o := &Link{}
	if err = randomize.Struct(seed, o, linkDBTypes, true, linkColumnsWithDefault...); err != nil {
		t.Errorf("Unable to randomize Link struct: %s", err)
	}

	ctx := context.Background()
	tx := MustTx(boil.BeginTx(ctx, nil))
	defer func() { _ = tx.Rollback() }()
	if err = o.Insert(ctx, tx, boil.Infer()); err != nil {
		t.Error(err)
	}

	e, err := LinkExists(ctx, tx, o.ID)
	if err != nil {
		t.Errorf("Unable to check if Link exists: %s", err)
	}
	if !e {
		t.Errorf("Expected LinkExists to return true, but got false.")
	}
}

func testLinksFind(t *testing.T) {
	t.Parallel()

	seed := randomize.NewSeed()
	var err error
	o := &Link{}
	if err = randomize.Struct(seed, o, linkDBTypes, true, linkColumnsWithDefault...); err != nil {
		t.Errorf("Unable to randomize Link struct: %s", err)
	}

	ctx := context.Background()
	tx := MustTx(boil.BeginTx(ctx, nil))
	defer func() { _ = tx.Rollback() }()
	if err = o.Insert(ctx, tx, boil.Infer()); err != nil {
		t.Error(err)
	}

	linkFound, err := FindLink(ctx, tx, o.ID)
	if err != nil {
		t.Error(err)
	}

	if linkFound == nil {
		t.Error("want a record, got nil")
	}
}

func testLinksBind(t *testing.T) {
	t.Parallel()

	seed := randomize.NewSeed()
	var err error
	o := &Link{}
	if err = randomize.Struct(seed, o, linkDBTypes, true, linkColumnsWithDefault...); err != nil {
		t.Errorf("Unable to randomize Link struct: %s", err)
	}

	ctx := context.Background()
	tx := MustTx(boil.BeginTx(ctx, nil))
	defer func() { _ = tx.Rollback() }()
	if err = o.Insert(ctx, tx, boil.Infer()); err != nil {
		t.Error(err)
	}

	if err = Links().Bind(ctx, tx, o); err != nil {
		t.Error(err)
	}
}

func testLinksOne(t *testing.T) {
	t.Parallel()

	seed := randomize.NewSeed()
	var err error
	o := &Link{}
	if err = randomize.Struct(seed, o, linkDBTypes, true, linkColumnsWithDefault...); err != nil {
		t.Errorf("Unable to randomize Link struct: %s", err)
	}

	ctx := context.Background()
	tx := MustTx(boil.BeginTx(ctx, nil))
	defer func() { _ = tx.Rollback() }()
	if err = o.Insert(ctx, tx, boil.Infer()); err != nil {
		t.Error(err)
	}

	if x, err := Links().One(ctx, tx); err != nil {
		t.Error(err)
	} else if x == nil {
		t.Error("expected to get a non nil record")
	}
}

func testLinksAll(t *testing.T) {
	t.Parallel()

	seed := randomize.NewSeed()
	var err error
	linkOne := &Link{}
	linkTwo := &Link{}
	if err = randomize.Struct(seed, linkOne, linkDBTypes, false, linkColumnsWithDefault...); err != nil {
		t.Errorf("Unable to randomize Link struct: %s", err)
	}
	if err = randomize.Struct(seed, linkTwo, linkDBTypes, false, linkColumnsWithDefault...); err != nil {
		t.Errorf("Unable to randomize Link struct: %s", err)
	}

	ctx := context.Background()
	tx := MustTx(boil.BeginTx(ctx, nil))
	defer func() { _ = tx.Rollback() }()
	if err = linkOne.Insert(ctx, tx, boil.Infer()); err != nil {
		t.Error(err)
	}
	if err = linkTwo.Insert(ctx, tx, boil.Infer()); err != nil {
		t.Error(err)
	}

	slice, err := Links().All(ctx, tx)
	if err != nil {
		t.Error(err)
	}

	if len(slice) != 2 {
		t.Error("want 2 records, got:", len(slice))
	}
}

func testLinksCount(t *testing.T) {
	t.Parallel()

	var err error
	seed := randomize.NewSeed()
	linkOne := &Link{}
	linkTwo := &Link{}
	if err = randomize.Struct(seed, linkOne, linkDBTypes, false, linkColumnsWithDefault...); err != nil {
		t.Errorf("Unable to randomize Link struct: %s", err)
	}
	if err = randomize.Struct(seed, linkTwo, linkDBTypes, false, linkColumnsWithDefault...); err != nil {
		t.Errorf("Unable to randomize Link struct: %s", err)
	}

	ctx := context.Background()
	tx := MustTx(boil.BeginTx(ctx, nil))
	defer func() { _ = tx.Rollback() }()
	if err = linkOne.Insert(ctx, tx, boil.Infer()); err != nil {
		t.Error(err)
	}
	if err = linkTwo.Insert(ctx, tx, boil.Infer()); err != nil {
		t.Error(err)
	}

	count, err := Links().Count(ctx, tx)
	if err != nil {
		t.Error(err)
	}

	if count != 2 {
		t.Error("want 2 records, got:", count)
	}
}

func testLinksInsert(t *testing.T) {
	t.Parallel()

	seed := randomize.NewSeed()
	var err error
	o := &Link{}
	if err = randomize.Struct(seed, o, linkDBTypes, true, linkColumnsWithDefault...); err != nil {
		t.Errorf("Unable to randomize Link struct: %s", err)
	}

	ctx := context.Background()
	tx := MustTx(boil.BeginTx(ctx, nil))
	defer func() { _ = tx.Rollback() }()
	if err = o.Insert(ctx, tx, boil.Infer()); err != nil {
		t.Error(err)
	}

	count, err := Links().Count(ctx, tx)
	if err != nil {
		t.Error(err)
	}

	if count != 1 {
		t.Error("want one record, got:", count)
	}
}

func testLinksInsertWhitelist(t *testing.T) {
	t.Parallel()

	seed := randomize.NewSeed()
	var err error
	o := &Link{}
	if err = randomize.Struct(seed, o, linkDBTypes, true); err != nil {
		t.Errorf("Unable to randomize Link struct: %s", err)
	}

	ctx := context.Background()
	tx := MustTx(boil.BeginTx(ctx, nil))
	defer func() { _ = tx.Rollback() }()
	if err = o.Insert(ctx, tx, boil.Whitelist(linkColumnsWithoutDefault...)); err != nil {
		t.Error(err)
	}

	count, err := Links().Count(ctx, tx)
	if err != nil {
		t.Error(err)
	}

	if count != 1 {
		t.Error("want one record, got:", count)
	}
}

func testLinkToManyParentTopics(t *testing.T) {
	var err error
	ctx := context.Background()
	tx := MustTx(boil.BeginTx(ctx, nil))
	defer func() { _ = tx.Rollback() }()

	var a Link
	var b, c Topic

	seed := randomize.NewSeed()
	if err = randomize.Struct(seed, &a, linkDBTypes, true, linkColumnsWithDefault...); err != nil {
		t.Errorf("Unable to randomize Link struct: %s", err)
	}

	if err := a.Insert(ctx, tx, boil.Infer()); err != nil {
		t.Fatal(err)
	}

	if err = randomize.Struct(seed, &b, topicDBTypes, false, topicColumnsWithDefault...); err != nil {
		t.Fatal(err)
	}
	if err = randomize.Struct(seed, &c, topicDBTypes, false, topicColumnsWithDefault...); err != nil {
		t.Fatal(err)
	}

	if err = b.Insert(ctx, tx, boil.Infer()); err != nil {
		t.Fatal(err)
	}
	if err = c.Insert(ctx, tx, boil.Infer()); err != nil {
		t.Fatal(err)
	}

	_, err = tx.Exec("insert into \"link_topics\" (\"child_id\", \"parent_id\") values ($1, $2)", a.ID, b.ID)
	if err != nil {
		t.Fatal(err)
	}
	_, err = tx.Exec("insert into \"link_topics\" (\"child_id\", \"parent_id\") values ($1, $2)", a.ID, c.ID)
	if err != nil {
		t.Fatal(err)
	}

	check, err := a.ParentTopics().All(ctx, tx)
	if err != nil {
		t.Fatal(err)
	}

	bFound, cFound := false, false
	for _, v := range check {
		if v.ID == b.ID {
			bFound = true
		}
		if v.ID == c.ID {
			cFound = true
		}
	}

	if !bFound {
		t.Error("expected to find b")
	}
	if !cFound {
		t.Error("expected to find c")
	}

	slice := LinkSlice{&a}
	if err = a.L.LoadParentTopics(ctx, tx, false, (*[]*Link)(&slice), nil); err != nil {
		t.Fatal(err)
	}
	if got := len(a.R.ParentTopics); got != 2 {
		t.Error("number of eager loaded records wrong, got:", got)
	}

	a.R.ParentTopics = nil
	if err = a.L.LoadParentTopics(ctx, tx, true, &a, nil); err != nil {
		t.Fatal(err)
	}
	if got := len(a.R.ParentTopics); got != 2 {
		t.Error("number of eager loaded records wrong, got:", got)
	}

	if t.Failed() {
		t.Logf("%#v", check)
	}
}

func testLinkToManyChildLinkTransitiveClosures(t *testing.T) {
	var err error
	ctx := context.Background()
	tx := MustTx(boil.BeginTx(ctx, nil))
	defer func() { _ = tx.Rollback() }()

	var a Link
	var b, c LinkTransitiveClosure

	seed := randomize.NewSeed()
	if err = randomize.Struct(seed, &a, linkDBTypes, true, linkColumnsWithDefault...); err != nil {
		t.Errorf("Unable to randomize Link struct: %s", err)
	}

	if err := a.Insert(ctx, tx, boil.Infer()); err != nil {
		t.Fatal(err)
	}

	if err = randomize.Struct(seed, &b, linkTransitiveClosureDBTypes, false, linkTransitiveClosureColumnsWithDefault...); err != nil {
		t.Fatal(err)
	}
	if err = randomize.Struct(seed, &c, linkTransitiveClosureDBTypes, false, linkTransitiveClosureColumnsWithDefault...); err != nil {
		t.Fatal(err)
	}

	b.ChildID = a.ID
	c.ChildID = a.ID

	if err = b.Insert(ctx, tx, boil.Infer()); err != nil {
		t.Fatal(err)
	}
	if err = c.Insert(ctx, tx, boil.Infer()); err != nil {
		t.Fatal(err)
	}

	check, err := a.ChildLinkTransitiveClosures().All(ctx, tx)
	if err != nil {
		t.Fatal(err)
	}

	bFound, cFound := false, false
	for _, v := range check {
		if v.ChildID == b.ChildID {
			bFound = true
		}
		if v.ChildID == c.ChildID {
			cFound = true
		}
	}

	if !bFound {
		t.Error("expected to find b")
	}
	if !cFound {
		t.Error("expected to find c")
	}

	slice := LinkSlice{&a}
	if err = a.L.LoadChildLinkTransitiveClosures(ctx, tx, false, (*[]*Link)(&slice), nil); err != nil {
		t.Fatal(err)
	}
	if got := len(a.R.ChildLinkTransitiveClosures); got != 2 {
		t.Error("number of eager loaded records wrong, got:", got)
	}

	a.R.ChildLinkTransitiveClosures = nil
	if err = a.L.LoadChildLinkTransitiveClosures(ctx, tx, true, &a, nil); err != nil {
		t.Fatal(err)
	}
	if got := len(a.R.ChildLinkTransitiveClosures); got != 2 {
		t.Error("number of eager loaded records wrong, got:", got)
	}

	if t.Failed() {
		t.Logf("%#v", check)
	}
}

func testLinkToManyUserLinkReviews(t *testing.T) {
	var err error
	ctx := context.Background()
	tx := MustTx(boil.BeginTx(ctx, nil))
	defer func() { _ = tx.Rollback() }()

	var a Link
	var b, c UserLinkReview

	seed := randomize.NewSeed()
	if err = randomize.Struct(seed, &a, linkDBTypes, true, linkColumnsWithDefault...); err != nil {
		t.Errorf("Unable to randomize Link struct: %s", err)
	}

	if err := a.Insert(ctx, tx, boil.Infer()); err != nil {
		t.Fatal(err)
	}

	if err = randomize.Struct(seed, &b, userLinkReviewDBTypes, false, userLinkReviewColumnsWithDefault...); err != nil {
		t.Fatal(err)
	}
	if err = randomize.Struct(seed, &c, userLinkReviewDBTypes, false, userLinkReviewColumnsWithDefault...); err != nil {
		t.Fatal(err)
	}

	b.LinkID = a.ID
	c.LinkID = a.ID

	if err = b.Insert(ctx, tx, boil.Infer()); err != nil {
		t.Fatal(err)
	}
	if err = c.Insert(ctx, tx, boil.Infer()); err != nil {
		t.Fatal(err)
	}

	check, err := a.UserLinkReviews().All(ctx, tx)
	if err != nil {
		t.Fatal(err)
	}

	bFound, cFound := false, false
	for _, v := range check {
		if v.LinkID == b.LinkID {
			bFound = true
		}
		if v.LinkID == c.LinkID {
			cFound = true
		}
	}

	if !bFound {
		t.Error("expected to find b")
	}
	if !cFound {
		t.Error("expected to find c")
	}

	slice := LinkSlice{&a}
	if err = a.L.LoadUserLinkReviews(ctx, tx, false, (*[]*Link)(&slice), nil); err != nil {
		t.Fatal(err)
	}
	if got := len(a.R.UserLinkReviews); got != 2 {
		t.Error("number of eager loaded records wrong, got:", got)
	}

	a.R.UserLinkReviews = nil
	if err = a.L.LoadUserLinkReviews(ctx, tx, true, &a, nil); err != nil {
		t.Fatal(err)
	}
	if got := len(a.R.UserLinkReviews); got != 2 {
		t.Error("number of eager loaded records wrong, got:", got)
	}

	if t.Failed() {
		t.Logf("%#v", check)
	}
}

func testLinkToManyUserLinks(t *testing.T) {
	var err error
	ctx := context.Background()
	tx := MustTx(boil.BeginTx(ctx, nil))
	defer func() { _ = tx.Rollback() }()

	var a Link
	var b, c UserLink

	seed := randomize.NewSeed()
	if err = randomize.Struct(seed, &a, linkDBTypes, true, linkColumnsWithDefault...); err != nil {
		t.Errorf("Unable to randomize Link struct: %s", err)
	}

	if err := a.Insert(ctx, tx, boil.Infer()); err != nil {
		t.Fatal(err)
	}

	if err = randomize.Struct(seed, &b, userLinkDBTypes, false, userLinkColumnsWithDefault...); err != nil {
		t.Fatal(err)
	}
	if err = randomize.Struct(seed, &c, userLinkDBTypes, false, userLinkColumnsWithDefault...); err != nil {
		t.Fatal(err)
	}

	b.LinkID = a.ID
	c.LinkID = a.ID

	if err = b.Insert(ctx, tx, boil.Infer()); err != nil {
		t.Fatal(err)
	}
	if err = c.Insert(ctx, tx, boil.Infer()); err != nil {
		t.Fatal(err)
	}

	check, err := a.UserLinks().All(ctx, tx)
	if err != nil {
		t.Fatal(err)
	}

	bFound, cFound := false, false
	for _, v := range check {
		if v.LinkID == b.LinkID {
			bFound = true
		}
		if v.LinkID == c.LinkID {
			cFound = true
		}
	}

	if !bFound {
		t.Error("expected to find b")
	}
	if !cFound {
		t.Error("expected to find c")
	}

	slice := LinkSlice{&a}
	if err = a.L.LoadUserLinks(ctx, tx, false, (*[]*Link)(&slice), nil); err != nil {
		t.Fatal(err)
	}
	if got := len(a.R.UserLinks); got != 2 {
		t.Error("number of eager loaded records wrong, got:", got)
	}

	a.R.UserLinks = nil
	if err = a.L.LoadUserLinks(ctx, tx, true, &a, nil); err != nil {
		t.Fatal(err)
	}
	if got := len(a.R.UserLinks); got != 2 {
		t.Error("number of eager loaded records wrong, got:", got)
	}

	if t.Failed() {
		t.Logf("%#v", check)
	}
}

func testLinkToManyAddOpParentTopics(t *testing.T) {
	var err error

	ctx := context.Background()
	tx := MustTx(boil.BeginTx(ctx, nil))
	defer func() { _ = tx.Rollback() }()

	var a Link
	var b, c, d, e Topic

	seed := randomize.NewSeed()
	if err = randomize.Struct(seed, &a, linkDBTypes, false, strmangle.SetComplement(linkPrimaryKeyColumns, linkColumnsWithoutDefault)...); err != nil {
		t.Fatal(err)
	}
	foreigners := []*Topic{&b, &c, &d, &e}
	for _, x := range foreigners {
		if err = randomize.Struct(seed, x, topicDBTypes, false, strmangle.SetComplement(topicPrimaryKeyColumns, topicColumnsWithoutDefault)...); err != nil {
			t.Fatal(err)
		}
	}

	if err := a.Insert(ctx, tx, boil.Infer()); err != nil {
		t.Fatal(err)
	}
	if err = b.Insert(ctx, tx, boil.Infer()); err != nil {
		t.Fatal(err)
	}
	if err = c.Insert(ctx, tx, boil.Infer()); err != nil {
		t.Fatal(err)
	}

	foreignersSplitByInsertion := [][]*Topic{
		{&b, &c},
		{&d, &e},
	}

	for i, x := range foreignersSplitByInsertion {
		err = a.AddParentTopics(ctx, tx, i != 0, x...)
		if err != nil {
			t.Fatal(err)
		}

		first := x[0]
		second := x[1]

		if first.R.ChildLinks[0] != &a {
			t.Error("relationship was not added properly to the slice")
		}
		if second.R.ChildLinks[0] != &a {
			t.Error("relationship was not added properly to the slice")
		}

		if a.R.ParentTopics[i*2] != first {
			t.Error("relationship struct slice not set to correct value")
		}
		if a.R.ParentTopics[i*2+1] != second {
			t.Error("relationship struct slice not set to correct value")
		}

		count, err := a.ParentTopics().Count(ctx, tx)
		if err != nil {
			t.Fatal(err)
		}
		if want := int64((i + 1) * 2); count != want {
			t.Error("want", want, "got", count)
		}
	}
}

func testLinkToManySetOpParentTopics(t *testing.T) {
	var err error

	ctx := context.Background()
	tx := MustTx(boil.BeginTx(ctx, nil))
	defer func() { _ = tx.Rollback() }()

	var a Link
	var b, c, d, e Topic

	seed := randomize.NewSeed()
	if err = randomize.Struct(seed, &a, linkDBTypes, false, strmangle.SetComplement(linkPrimaryKeyColumns, linkColumnsWithoutDefault)...); err != nil {
		t.Fatal(err)
	}
	foreigners := []*Topic{&b, &c, &d, &e}
	for _, x := range foreigners {
		if err = randomize.Struct(seed, x, topicDBTypes, false, strmangle.SetComplement(topicPrimaryKeyColumns, topicColumnsWithoutDefault)...); err != nil {
			t.Fatal(err)
		}
	}

	if err = a.Insert(ctx, tx, boil.Infer()); err != nil {
		t.Fatal(err)
	}
	if err = b.Insert(ctx, tx, boil.Infer()); err != nil {
		t.Fatal(err)
	}
	if err = c.Insert(ctx, tx, boil.Infer()); err != nil {
		t.Fatal(err)
	}

	err = a.SetParentTopics(ctx, tx, false, &b, &c)
	if err != nil {
		t.Fatal(err)
	}

	count, err := a.ParentTopics().Count(ctx, tx)
	if err != nil {
		t.Fatal(err)
	}
	if count != 2 {
		t.Error("count was wrong:", count)
	}

	err = a.SetParentTopics(ctx, tx, true, &d, &e)
	if err != nil {
		t.Fatal(err)
	}

	count, err = a.ParentTopics().Count(ctx, tx)
	if err != nil {
		t.Fatal(err)
	}
	if count != 2 {
		t.Error("count was wrong:", count)
	}

	// The following checks cannot be implemented since we have no handle
	// to these when we call Set(). Leaving them here as wishful thinking
	// and to let people know there's dragons.
	//
	// if len(b.R.ChildLinks) != 0 {
	// 	t.Error("relationship was not removed properly from the slice")
	// }
	// if len(c.R.ChildLinks) != 0 {
	// 	t.Error("relationship was not removed properly from the slice")
	// }
	if d.R.ChildLinks[0] != &a {
		t.Error("relationship was not added properly to the slice")
	}
	if e.R.ChildLinks[0] != &a {
		t.Error("relationship was not added properly to the slice")
	}

	if a.R.ParentTopics[0] != &d {
		t.Error("relationship struct slice not set to correct value")
	}
	if a.R.ParentTopics[1] != &e {
		t.Error("relationship struct slice not set to correct value")
	}
}

func testLinkToManyRemoveOpParentTopics(t *testing.T) {
	var err error

	ctx := context.Background()
	tx := MustTx(boil.BeginTx(ctx, nil))
	defer func() { _ = tx.Rollback() }()

	var a Link
	var b, c, d, e Topic

	seed := randomize.NewSeed()
	if err = randomize.Struct(seed, &a, linkDBTypes, false, strmangle.SetComplement(linkPrimaryKeyColumns, linkColumnsWithoutDefault)...); err != nil {
		t.Fatal(err)
	}
	foreigners := []*Topic{&b, &c, &d, &e}
	for _, x := range foreigners {
		if err = randomize.Struct(seed, x, topicDBTypes, false, strmangle.SetComplement(topicPrimaryKeyColumns, topicColumnsWithoutDefault)...); err != nil {
			t.Fatal(err)
		}
	}

	if err := a.Insert(ctx, tx, boil.Infer()); err != nil {
		t.Fatal(err)
	}

	err = a.AddParentTopics(ctx, tx, true, foreigners...)
	if err != nil {
		t.Fatal(err)
	}

	count, err := a.ParentTopics().Count(ctx, tx)
	if err != nil {
		t.Fatal(err)
	}
	if count != 4 {
		t.Error("count was wrong:", count)
	}

	err = a.RemoveParentTopics(ctx, tx, foreigners[:2]...)
	if err != nil {
		t.Fatal(err)
	}

	count, err = a.ParentTopics().Count(ctx, tx)
	if err != nil {
		t.Fatal(err)
	}
	if count != 2 {
		t.Error("count was wrong:", count)
	}

	if len(b.R.ChildLinks) != 0 {
		t.Error("relationship was not removed properly from the slice")
	}
	if len(c.R.ChildLinks) != 0 {
		t.Error("relationship was not removed properly from the slice")
	}
	if d.R.ChildLinks[0] != &a {
		t.Error("relationship was not added properly to the foreign struct")
	}
	if e.R.ChildLinks[0] != &a {
		t.Error("relationship was not added properly to the foreign struct")
	}

	if len(a.R.ParentTopics) != 2 {
		t.Error("should have preserved two relationships")
	}

	// Removal doesn't do a stable deletion for performance so we have to flip the order
	if a.R.ParentTopics[1] != &d {
		t.Error("relationship to d should have been preserved")
	}
	if a.R.ParentTopics[0] != &e {
		t.Error("relationship to e should have been preserved")
	}
}

func testLinkToManyAddOpChildLinkTransitiveClosures(t *testing.T) {
	var err error

	ctx := context.Background()
	tx := MustTx(boil.BeginTx(ctx, nil))
	defer func() { _ = tx.Rollback() }()

	var a Link
	var b, c, d, e LinkTransitiveClosure

	seed := randomize.NewSeed()
	if err = randomize.Struct(seed, &a, linkDBTypes, false, strmangle.SetComplement(linkPrimaryKeyColumns, linkColumnsWithoutDefault)...); err != nil {
		t.Fatal(err)
	}
	foreigners := []*LinkTransitiveClosure{&b, &c, &d, &e}
	for _, x := range foreigners {
		if err = randomize.Struct(seed, x, linkTransitiveClosureDBTypes, false, strmangle.SetComplement(linkTransitiveClosurePrimaryKeyColumns, linkTransitiveClosureColumnsWithoutDefault)...); err != nil {
			t.Fatal(err)
		}
	}

	if err := a.Insert(ctx, tx, boil.Infer()); err != nil {
		t.Fatal(err)
	}
	if err = b.Insert(ctx, tx, boil.Infer()); err != nil {
		t.Fatal(err)
	}
	if err = c.Insert(ctx, tx, boil.Infer()); err != nil {
		t.Fatal(err)
	}

	foreignersSplitByInsertion := [][]*LinkTransitiveClosure{
		{&b, &c},
		{&d, &e},
	}

	for i, x := range foreignersSplitByInsertion {
		err = a.AddChildLinkTransitiveClosures(ctx, tx, i != 0, x...)
		if err != nil {
			t.Fatal(err)
		}

		first := x[0]
		second := x[1]

		if a.ID != first.ChildID {
			t.Error("foreign key was wrong value", a.ID, first.ChildID)
		}
		if a.ID != second.ChildID {
			t.Error("foreign key was wrong value", a.ID, second.ChildID)
		}

		if first.R.Child != &a {
			t.Error("relationship was not added properly to the foreign slice")
		}
		if second.R.Child != &a {
			t.Error("relationship was not added properly to the foreign slice")
		}

		if a.R.ChildLinkTransitiveClosures[i*2] != first {
			t.Error("relationship struct slice not set to correct value")
		}
		if a.R.ChildLinkTransitiveClosures[i*2+1] != second {
			t.Error("relationship struct slice not set to correct value")
		}

		count, err := a.ChildLinkTransitiveClosures().Count(ctx, tx)
		if err != nil {
			t.Fatal(err)
		}
		if want := int64((i + 1) * 2); count != want {
			t.Error("want", want, "got", count)
		}
	}
}
func testLinkToManyAddOpUserLinkReviews(t *testing.T) {
	var err error

	ctx := context.Background()
	tx := MustTx(boil.BeginTx(ctx, nil))
	defer func() { _ = tx.Rollback() }()

	var a Link
	var b, c, d, e UserLinkReview

	seed := randomize.NewSeed()
	if err = randomize.Struct(seed, &a, linkDBTypes, false, strmangle.SetComplement(linkPrimaryKeyColumns, linkColumnsWithoutDefault)...); err != nil {
		t.Fatal(err)
	}
	foreigners := []*UserLinkReview{&b, &c, &d, &e}
	for _, x := range foreigners {
		if err = randomize.Struct(seed, x, userLinkReviewDBTypes, false, strmangle.SetComplement(userLinkReviewPrimaryKeyColumns, userLinkReviewColumnsWithoutDefault)...); err != nil {
			t.Fatal(err)
		}
	}

	if err := a.Insert(ctx, tx, boil.Infer()); err != nil {
		t.Fatal(err)
	}
	if err = b.Insert(ctx, tx, boil.Infer()); err != nil {
		t.Fatal(err)
	}
	if err = c.Insert(ctx, tx, boil.Infer()); err != nil {
		t.Fatal(err)
	}

	foreignersSplitByInsertion := [][]*UserLinkReview{
		{&b, &c},
		{&d, &e},
	}

	for i, x := range foreignersSplitByInsertion {
		err = a.AddUserLinkReviews(ctx, tx, i != 0, x...)
		if err != nil {
			t.Fatal(err)
		}

		first := x[0]
		second := x[1]

		if a.ID != first.LinkID {
			t.Error("foreign key was wrong value", a.ID, first.LinkID)
		}
		if a.ID != second.LinkID {
			t.Error("foreign key was wrong value", a.ID, second.LinkID)
		}

		if first.R.Link != &a {
			t.Error("relationship was not added properly to the foreign slice")
		}
		if second.R.Link != &a {
			t.Error("relationship was not added properly to the foreign slice")
		}

		if a.R.UserLinkReviews[i*2] != first {
			t.Error("relationship struct slice not set to correct value")
		}
		if a.R.UserLinkReviews[i*2+1] != second {
			t.Error("relationship struct slice not set to correct value")
		}

		count, err := a.UserLinkReviews().Count(ctx, tx)
		if err != nil {
			t.Fatal(err)
		}
		if want := int64((i + 1) * 2); count != want {
			t.Error("want", want, "got", count)
		}
	}
}
func testLinkToManyAddOpUserLinks(t *testing.T) {
	var err error

	ctx := context.Background()
	tx := MustTx(boil.BeginTx(ctx, nil))
	defer func() { _ = tx.Rollback() }()

	var a Link
	var b, c, d, e UserLink

	seed := randomize.NewSeed()
	if err = randomize.Struct(seed, &a, linkDBTypes, false, strmangle.SetComplement(linkPrimaryKeyColumns, linkColumnsWithoutDefault)...); err != nil {
		t.Fatal(err)
	}
	foreigners := []*UserLink{&b, &c, &d, &e}
	for _, x := range foreigners {
		if err = randomize.Struct(seed, x, userLinkDBTypes, false, strmangle.SetComplement(userLinkPrimaryKeyColumns, userLinkColumnsWithoutDefault)...); err != nil {
			t.Fatal(err)
		}
	}

	if err := a.Insert(ctx, tx, boil.Infer()); err != nil {
		t.Fatal(err)
	}
	if err = b.Insert(ctx, tx, boil.Infer()); err != nil {
		t.Fatal(err)
	}
	if err = c.Insert(ctx, tx, boil.Infer()); err != nil {
		t.Fatal(err)
	}

	foreignersSplitByInsertion := [][]*UserLink{
		{&b, &c},
		{&d, &e},
	}

	for i, x := range foreignersSplitByInsertion {
		err = a.AddUserLinks(ctx, tx, i != 0, x...)
		if err != nil {
			t.Fatal(err)
		}

		first := x[0]
		second := x[1]

		if a.ID != first.LinkID {
			t.Error("foreign key was wrong value", a.ID, first.LinkID)
		}
		if a.ID != second.LinkID {
			t.Error("foreign key was wrong value", a.ID, second.LinkID)
		}

		if first.R.Link != &a {
			t.Error("relationship was not added properly to the foreign slice")
		}
		if second.R.Link != &a {
			t.Error("relationship was not added properly to the foreign slice")
		}

		if a.R.UserLinks[i*2] != first {
			t.Error("relationship struct slice not set to correct value")
		}
		if a.R.UserLinks[i*2+1] != second {
			t.Error("relationship struct slice not set to correct value")
		}

		count, err := a.UserLinks().Count(ctx, tx)
		if err != nil {
			t.Fatal(err)
		}
		if want := int64((i + 1) * 2); count != want {
			t.Error("want", want, "got", count)
		}
	}
}
func testLinkToOneOrganizationUsingOrganization(t *testing.T) {
	ctx := context.Background()
	tx := MustTx(boil.BeginTx(ctx, nil))
	defer func() { _ = tx.Rollback() }()

	var local Link
	var foreign Organization

	seed := randomize.NewSeed()
	if err := randomize.Struct(seed, &local, linkDBTypes, false, linkColumnsWithDefault...); err != nil {
		t.Errorf("Unable to randomize Link struct: %s", err)
	}
	if err := randomize.Struct(seed, &foreign, organizationDBTypes, false, organizationColumnsWithDefault...); err != nil {
		t.Errorf("Unable to randomize Organization struct: %s", err)
	}

	if err := foreign.Insert(ctx, tx, boil.Infer()); err != nil {
		t.Fatal(err)
	}

	local.OrganizationID = foreign.ID
	if err := local.Insert(ctx, tx, boil.Infer()); err != nil {
		t.Fatal(err)
	}

	check, err := local.Organization().One(ctx, tx)
	if err != nil {
		t.Fatal(err)
	}

	if check.ID != foreign.ID {
		t.Errorf("want: %v, got %v", foreign.ID, check.ID)
	}

	slice := LinkSlice{&local}
	if err = local.L.LoadOrganization(ctx, tx, false, (*[]*Link)(&slice), nil); err != nil {
		t.Fatal(err)
	}
	if local.R.Organization == nil {
		t.Error("struct should have been eager loaded")
	}

	local.R.Organization = nil
	if err = local.L.LoadOrganization(ctx, tx, true, &local, nil); err != nil {
		t.Fatal(err)
	}
	if local.R.Organization == nil {
		t.Error("struct should have been eager loaded")
	}
}

func testLinkToOneRepositoryUsingRepository(t *testing.T) {
	ctx := context.Background()
	tx := MustTx(boil.BeginTx(ctx, nil))
	defer func() { _ = tx.Rollback() }()

	var local Link
	var foreign Repository

	seed := randomize.NewSeed()
	if err := randomize.Struct(seed, &local, linkDBTypes, false, linkColumnsWithDefault...); err != nil {
		t.Errorf("Unable to randomize Link struct: %s", err)
	}
	if err := randomize.Struct(seed, &foreign, repositoryDBTypes, false, repositoryColumnsWithDefault...); err != nil {
		t.Errorf("Unable to randomize Repository struct: %s", err)
	}

	if err := foreign.Insert(ctx, tx, boil.Infer()); err != nil {
		t.Fatal(err)
	}

	local.RepositoryID = foreign.ID
	if err := local.Insert(ctx, tx, boil.Infer()); err != nil {
		t.Fatal(err)
	}

	check, err := local.Repository().One(ctx, tx)
	if err != nil {
		t.Fatal(err)
	}

	if check.ID != foreign.ID {
		t.Errorf("want: %v, got %v", foreign.ID, check.ID)
	}

	slice := LinkSlice{&local}
	if err = local.L.LoadRepository(ctx, tx, false, (*[]*Link)(&slice), nil); err != nil {
		t.Fatal(err)
	}
	if local.R.Repository == nil {
		t.Error("struct should have been eager loaded")
	}

	local.R.Repository = nil
	if err = local.L.LoadRepository(ctx, tx, true, &local, nil); err != nil {
		t.Fatal(err)
	}
	if local.R.Repository == nil {
		t.Error("struct should have been eager loaded")
	}
}

func testLinkToOneSetOpOrganizationUsingOrganization(t *testing.T) {
	var err error

	ctx := context.Background()
	tx := MustTx(boil.BeginTx(ctx, nil))
	defer func() { _ = tx.Rollback() }()

	var a Link
	var b, c Organization

	seed := randomize.NewSeed()
	if err = randomize.Struct(seed, &a, linkDBTypes, false, strmangle.SetComplement(linkPrimaryKeyColumns, linkColumnsWithoutDefault)...); err != nil {
		t.Fatal(err)
	}
	if err = randomize.Struct(seed, &b, organizationDBTypes, false, strmangle.SetComplement(organizationPrimaryKeyColumns, organizationColumnsWithoutDefault)...); err != nil {
		t.Fatal(err)
	}
	if err = randomize.Struct(seed, &c, organizationDBTypes, false, strmangle.SetComplement(organizationPrimaryKeyColumns, organizationColumnsWithoutDefault)...); err != nil {
		t.Fatal(err)
	}

	if err := a.Insert(ctx, tx, boil.Infer()); err != nil {
		t.Fatal(err)
	}
	if err = b.Insert(ctx, tx, boil.Infer()); err != nil {
		t.Fatal(err)
	}

	for i, x := range []*Organization{&b, &c} {
		err = a.SetOrganization(ctx, tx, i != 0, x)
		if err != nil {
			t.Fatal(err)
		}

		if a.R.Organization != x {
			t.Error("relationship struct not set to correct value")
		}

		if x.R.Links[0] != &a {
			t.Error("failed to append to foreign relationship struct")
		}
		if a.OrganizationID != x.ID {
			t.Error("foreign key was wrong value", a.OrganizationID)
		}

		zero := reflect.Zero(reflect.TypeOf(a.OrganizationID))
		reflect.Indirect(reflect.ValueOf(&a.OrganizationID)).Set(zero)

		if err = a.Reload(ctx, tx); err != nil {
			t.Fatal("failed to reload", err)
		}

		if a.OrganizationID != x.ID {
			t.Error("foreign key was wrong value", a.OrganizationID, x.ID)
		}
	}
}
func testLinkToOneSetOpRepositoryUsingRepository(t *testing.T) {
	var err error

	ctx := context.Background()
	tx := MustTx(boil.BeginTx(ctx, nil))
	defer func() { _ = tx.Rollback() }()

	var a Link
	var b, c Repository

	seed := randomize.NewSeed()
	if err = randomize.Struct(seed, &a, linkDBTypes, false, strmangle.SetComplement(linkPrimaryKeyColumns, linkColumnsWithoutDefault)...); err != nil {
		t.Fatal(err)
	}
	if err = randomize.Struct(seed, &b, repositoryDBTypes, false, strmangle.SetComplement(repositoryPrimaryKeyColumns, repositoryColumnsWithoutDefault)...); err != nil {
		t.Fatal(err)
	}
	if err = randomize.Struct(seed, &c, repositoryDBTypes, false, strmangle.SetComplement(repositoryPrimaryKeyColumns, repositoryColumnsWithoutDefault)...); err != nil {
		t.Fatal(err)
	}

	if err := a.Insert(ctx, tx, boil.Infer()); err != nil {
		t.Fatal(err)
	}
	if err = b.Insert(ctx, tx, boil.Infer()); err != nil {
		t.Fatal(err)
	}

	for i, x := range []*Repository{&b, &c} {
		err = a.SetRepository(ctx, tx, i != 0, x)
		if err != nil {
			t.Fatal(err)
		}

		if a.R.Repository != x {
			t.Error("relationship struct not set to correct value")
		}

		if x.R.Links[0] != &a {
			t.Error("failed to append to foreign relationship struct")
		}
		if a.RepositoryID != x.ID {
			t.Error("foreign key was wrong value", a.RepositoryID)
		}

		zero := reflect.Zero(reflect.TypeOf(a.RepositoryID))
		reflect.Indirect(reflect.ValueOf(&a.RepositoryID)).Set(zero)

		if err = a.Reload(ctx, tx); err != nil {
			t.Fatal("failed to reload", err)
		}

		if a.RepositoryID != x.ID {
			t.Error("foreign key was wrong value", a.RepositoryID, x.ID)
		}
	}
}

func testLinksReload(t *testing.T) {
	t.Parallel()

	seed := randomize.NewSeed()
	var err error
	o := &Link{}
	if err = randomize.Struct(seed, o, linkDBTypes, true, linkColumnsWithDefault...); err != nil {
		t.Errorf("Unable to randomize Link struct: %s", err)
	}

	ctx := context.Background()
	tx := MustTx(boil.BeginTx(ctx, nil))
	defer func() { _ = tx.Rollback() }()
	if err = o.Insert(ctx, tx, boil.Infer()); err != nil {
		t.Error(err)
	}

	if err = o.Reload(ctx, tx); err != nil {
		t.Error(err)
	}
}

func testLinksReloadAll(t *testing.T) {
	t.Parallel()

	seed := randomize.NewSeed()
	var err error
	o := &Link{}
	if err = randomize.Struct(seed, o, linkDBTypes, true, linkColumnsWithDefault...); err != nil {
		t.Errorf("Unable to randomize Link struct: %s", err)
	}

	ctx := context.Background()
	tx := MustTx(boil.BeginTx(ctx, nil))
	defer func() { _ = tx.Rollback() }()
	if err = o.Insert(ctx, tx, boil.Infer()); err != nil {
		t.Error(err)
	}

	slice := LinkSlice{o}

	if err = slice.ReloadAll(ctx, tx); err != nil {
		t.Error(err)
	}
}

func testLinksSelect(t *testing.T) {
	t.Parallel()

	seed := randomize.NewSeed()
	var err error
	o := &Link{}
	if err = randomize.Struct(seed, o, linkDBTypes, true, linkColumnsWithDefault...); err != nil {
		t.Errorf("Unable to randomize Link struct: %s", err)
	}

	ctx := context.Background()
	tx := MustTx(boil.BeginTx(ctx, nil))
	defer func() { _ = tx.Rollback() }()
	if err = o.Insert(ctx, tx, boil.Infer()); err != nil {
		t.Error(err)
	}

	slice, err := Links().All(ctx, tx)
	if err != nil {
		t.Error(err)
	}

	if len(slice) != 1 {
		t.Error("want one record, got:", len(slice))
	}
}

var (
	linkDBTypes = map[string]string{`OrganizationID`: `uuid`, `ID`: `uuid`, `URL`: `text`, `Title`: `text`, `Sha1`: `character varying`, `CreatedAt`: `timestamp with time zone`, `UpdatedAt`: `timestamp with time zone`, `RepositoryID`: `uuid`}
	_           = bytes.MinRead
)

func testLinksUpdate(t *testing.T) {
	t.Parallel()

	if 0 == len(linkPrimaryKeyColumns) {
		t.Skip("Skipping table with no primary key columns")
	}
	if len(linkAllColumns) == len(linkPrimaryKeyColumns) {
		t.Skip("Skipping table with only primary key columns")
	}

	seed := randomize.NewSeed()
	var err error
	o := &Link{}
	if err = randomize.Struct(seed, o, linkDBTypes, true, linkColumnsWithDefault...); err != nil {
		t.Errorf("Unable to randomize Link struct: %s", err)
	}

	ctx := context.Background()
	tx := MustTx(boil.BeginTx(ctx, nil))
	defer func() { _ = tx.Rollback() }()
	if err = o.Insert(ctx, tx, boil.Infer()); err != nil {
		t.Error(err)
	}

	count, err := Links().Count(ctx, tx)
	if err != nil {
		t.Error(err)
	}

	if count != 1 {
		t.Error("want one record, got:", count)
	}

	if err = randomize.Struct(seed, o, linkDBTypes, true, linkPrimaryKeyColumns...); err != nil {
		t.Errorf("Unable to randomize Link struct: %s", err)
	}

	if rowsAff, err := o.Update(ctx, tx, boil.Infer()); err != nil {
		t.Error(err)
	} else if rowsAff != 1 {
		t.Error("should only affect one row but affected", rowsAff)
	}
}

func testLinksSliceUpdateAll(t *testing.T) {
	t.Parallel()

	if len(linkAllColumns) == len(linkPrimaryKeyColumns) {
		t.Skip("Skipping table with only primary key columns")
	}

	seed := randomize.NewSeed()
	var err error
	o := &Link{}
	if err = randomize.Struct(seed, o, linkDBTypes, true, linkColumnsWithDefault...); err != nil {
		t.Errorf("Unable to randomize Link struct: %s", err)
	}

	ctx := context.Background()
	tx := MustTx(boil.BeginTx(ctx, nil))
	defer func() { _ = tx.Rollback() }()
	if err = o.Insert(ctx, tx, boil.Infer()); err != nil {
		t.Error(err)
	}

	count, err := Links().Count(ctx, tx)
	if err != nil {
		t.Error(err)
	}

	if count != 1 {
		t.Error("want one record, got:", count)
	}

	if err = randomize.Struct(seed, o, linkDBTypes, true, linkPrimaryKeyColumns...); err != nil {
		t.Errorf("Unable to randomize Link struct: %s", err)
	}

	// Remove Primary keys and unique columns from what we plan to update
	var fields []string
	if strmangle.StringSliceMatch(linkAllColumns, linkPrimaryKeyColumns) {
		fields = linkAllColumns
	} else {
		fields = strmangle.SetComplement(
			linkAllColumns,
			linkPrimaryKeyColumns,
		)
	}

	value := reflect.Indirect(reflect.ValueOf(o))
	typ := reflect.TypeOf(o).Elem()
	n := typ.NumField()

	updateMap := M{}
	for _, col := range fields {
		for i := 0; i < n; i++ {
			f := typ.Field(i)
			if f.Tag.Get("boil") == col {
				updateMap[col] = value.Field(i).Interface()
			}
		}
	}

	slice := LinkSlice{o}
	if rowsAff, err := slice.UpdateAll(ctx, tx, updateMap); err != nil {
		t.Error(err)
	} else if rowsAff != 1 {
		t.Error("wanted one record updated but got", rowsAff)
	}
}

func testLinksUpsert(t *testing.T) {
	t.Parallel()

	if len(linkAllColumns) == len(linkPrimaryKeyColumns) {
		t.Skip("Skipping table with only primary key columns")
	}

	seed := randomize.NewSeed()
	var err error
	// Attempt the INSERT side of an UPSERT
	o := Link{}
	if err = randomize.Struct(seed, &o, linkDBTypes, true); err != nil {
		t.Errorf("Unable to randomize Link struct: %s", err)
	}

	ctx := context.Background()
	tx := MustTx(boil.BeginTx(ctx, nil))
	defer func() { _ = tx.Rollback() }()
	if err = o.Upsert(ctx, tx, false, nil, boil.Infer(), boil.Infer()); err != nil {
		t.Errorf("Unable to upsert Link: %s", err)
	}

	count, err := Links().Count(ctx, tx)
	if err != nil {
		t.Error(err)
	}
	if count != 1 {
		t.Error("want one record, got:", count)
	}

	// Attempt the UPDATE side of an UPSERT
	if err = randomize.Struct(seed, &o, linkDBTypes, false, linkPrimaryKeyColumns...); err != nil {
		t.Errorf("Unable to randomize Link struct: %s", err)
	}

	if err = o.Upsert(ctx, tx, true, nil, boil.Infer(), boil.Infer()); err != nil {
		t.Errorf("Unable to upsert Link: %s", err)
	}

	count, err = Links().Count(ctx, tx)
	if err != nil {
		t.Error(err)
	}
	if count != 1 {
		t.Error("want one record, got:", count)
	}
}
