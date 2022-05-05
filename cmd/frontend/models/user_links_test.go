// Code generated by SQLBoiler 4.10.2 (https://github.com/volatiletech/sqlboiler). DO NOT EDIT.
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

func testUserLinks(t *testing.T) {
	t.Parallel()

	query := UserLinks()

	if query.Query == nil {
		t.Error("expected a query, got nothing")
	}
}

func testUserLinksDelete(t *testing.T) {
	t.Parallel()

	seed := randomize.NewSeed()
	var err error
	o := &UserLink{}
	if err = randomize.Struct(seed, o, userLinkDBTypes, true, userLinkColumnsWithDefault...); err != nil {
		t.Errorf("Unable to randomize UserLink struct: %s", err)
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

	count, err := UserLinks().Count(ctx, tx)
	if err != nil {
		t.Error(err)
	}

	if count != 0 {
		t.Error("want zero records, got:", count)
	}
}

func testUserLinksQueryDeleteAll(t *testing.T) {
	t.Parallel()

	seed := randomize.NewSeed()
	var err error
	o := &UserLink{}
	if err = randomize.Struct(seed, o, userLinkDBTypes, true, userLinkColumnsWithDefault...); err != nil {
		t.Errorf("Unable to randomize UserLink struct: %s", err)
	}

	ctx := context.Background()
	tx := MustTx(boil.BeginTx(ctx, nil))
	defer func() { _ = tx.Rollback() }()
	if err = o.Insert(ctx, tx, boil.Infer()); err != nil {
		t.Error(err)
	}

	if rowsAff, err := UserLinks().DeleteAll(ctx, tx); err != nil {
		t.Error(err)
	} else if rowsAff != 1 {
		t.Error("should only have deleted one row, but affected:", rowsAff)
	}

	count, err := UserLinks().Count(ctx, tx)
	if err != nil {
		t.Error(err)
	}

	if count != 0 {
		t.Error("want zero records, got:", count)
	}
}

func testUserLinksSliceDeleteAll(t *testing.T) {
	t.Parallel()

	seed := randomize.NewSeed()
	var err error
	o := &UserLink{}
	if err = randomize.Struct(seed, o, userLinkDBTypes, true, userLinkColumnsWithDefault...); err != nil {
		t.Errorf("Unable to randomize UserLink struct: %s", err)
	}

	ctx := context.Background()
	tx := MustTx(boil.BeginTx(ctx, nil))
	defer func() { _ = tx.Rollback() }()
	if err = o.Insert(ctx, tx, boil.Infer()); err != nil {
		t.Error(err)
	}

	slice := UserLinkSlice{o}

	if rowsAff, err := slice.DeleteAll(ctx, tx); err != nil {
		t.Error(err)
	} else if rowsAff != 1 {
		t.Error("should only have deleted one row, but affected:", rowsAff)
	}

	count, err := UserLinks().Count(ctx, tx)
	if err != nil {
		t.Error(err)
	}

	if count != 0 {
		t.Error("want zero records, got:", count)
	}
}

func testUserLinksExists(t *testing.T) {
	t.Parallel()

	seed := randomize.NewSeed()
	var err error
	o := &UserLink{}
	if err = randomize.Struct(seed, o, userLinkDBTypes, true, userLinkColumnsWithDefault...); err != nil {
		t.Errorf("Unable to randomize UserLink struct: %s", err)
	}

	ctx := context.Background()
	tx := MustTx(boil.BeginTx(ctx, nil))
	defer func() { _ = tx.Rollback() }()
	if err = o.Insert(ctx, tx, boil.Infer()); err != nil {
		t.Error(err)
	}

	e, err := UserLinkExists(ctx, tx, o.ID)
	if err != nil {
		t.Errorf("Unable to check if UserLink exists: %s", err)
	}
	if !e {
		t.Errorf("Expected UserLinkExists to return true, but got false.")
	}
}

func testUserLinksFind(t *testing.T) {
	t.Parallel()

	seed := randomize.NewSeed()
	var err error
	o := &UserLink{}
	if err = randomize.Struct(seed, o, userLinkDBTypes, true, userLinkColumnsWithDefault...); err != nil {
		t.Errorf("Unable to randomize UserLink struct: %s", err)
	}

	ctx := context.Background()
	tx := MustTx(boil.BeginTx(ctx, nil))
	defer func() { _ = tx.Rollback() }()
	if err = o.Insert(ctx, tx, boil.Infer()); err != nil {
		t.Error(err)
	}

	userLinkFound, err := FindUserLink(ctx, tx, o.ID)
	if err != nil {
		t.Error(err)
	}

	if userLinkFound == nil {
		t.Error("want a record, got nil")
	}
}

func testUserLinksBind(t *testing.T) {
	t.Parallel()

	seed := randomize.NewSeed()
	var err error
	o := &UserLink{}
	if err = randomize.Struct(seed, o, userLinkDBTypes, true, userLinkColumnsWithDefault...); err != nil {
		t.Errorf("Unable to randomize UserLink struct: %s", err)
	}

	ctx := context.Background()
	tx := MustTx(boil.BeginTx(ctx, nil))
	defer func() { _ = tx.Rollback() }()
	if err = o.Insert(ctx, tx, boil.Infer()); err != nil {
		t.Error(err)
	}

	if err = UserLinks().Bind(ctx, tx, o); err != nil {
		t.Error(err)
	}
}

func testUserLinksOne(t *testing.T) {
	t.Parallel()

	seed := randomize.NewSeed()
	var err error
	o := &UserLink{}
	if err = randomize.Struct(seed, o, userLinkDBTypes, true, userLinkColumnsWithDefault...); err != nil {
		t.Errorf("Unable to randomize UserLink struct: %s", err)
	}

	ctx := context.Background()
	tx := MustTx(boil.BeginTx(ctx, nil))
	defer func() { _ = tx.Rollback() }()
	if err = o.Insert(ctx, tx, boil.Infer()); err != nil {
		t.Error(err)
	}

	if x, err := UserLinks().One(ctx, tx); err != nil {
		t.Error(err)
	} else if x == nil {
		t.Error("expected to get a non nil record")
	}
}

func testUserLinksAll(t *testing.T) {
	t.Parallel()

	seed := randomize.NewSeed()
	var err error
	userLinkOne := &UserLink{}
	userLinkTwo := &UserLink{}
	if err = randomize.Struct(seed, userLinkOne, userLinkDBTypes, false, userLinkColumnsWithDefault...); err != nil {
		t.Errorf("Unable to randomize UserLink struct: %s", err)
	}
	if err = randomize.Struct(seed, userLinkTwo, userLinkDBTypes, false, userLinkColumnsWithDefault...); err != nil {
		t.Errorf("Unable to randomize UserLink struct: %s", err)
	}

	ctx := context.Background()
	tx := MustTx(boil.BeginTx(ctx, nil))
	defer func() { _ = tx.Rollback() }()
	if err = userLinkOne.Insert(ctx, tx, boil.Infer()); err != nil {
		t.Error(err)
	}
	if err = userLinkTwo.Insert(ctx, tx, boil.Infer()); err != nil {
		t.Error(err)
	}

	slice, err := UserLinks().All(ctx, tx)
	if err != nil {
		t.Error(err)
	}

	if len(slice) != 2 {
		t.Error("want 2 records, got:", len(slice))
	}
}

func testUserLinksCount(t *testing.T) {
	t.Parallel()

	var err error
	seed := randomize.NewSeed()
	userLinkOne := &UserLink{}
	userLinkTwo := &UserLink{}
	if err = randomize.Struct(seed, userLinkOne, userLinkDBTypes, false, userLinkColumnsWithDefault...); err != nil {
		t.Errorf("Unable to randomize UserLink struct: %s", err)
	}
	if err = randomize.Struct(seed, userLinkTwo, userLinkDBTypes, false, userLinkColumnsWithDefault...); err != nil {
		t.Errorf("Unable to randomize UserLink struct: %s", err)
	}

	ctx := context.Background()
	tx := MustTx(boil.BeginTx(ctx, nil))
	defer func() { _ = tx.Rollback() }()
	if err = userLinkOne.Insert(ctx, tx, boil.Infer()); err != nil {
		t.Error(err)
	}
	if err = userLinkTwo.Insert(ctx, tx, boil.Infer()); err != nil {
		t.Error(err)
	}

	count, err := UserLinks().Count(ctx, tx)
	if err != nil {
		t.Error(err)
	}

	if count != 2 {
		t.Error("want 2 records, got:", count)
	}
}

func testUserLinksInsert(t *testing.T) {
	t.Parallel()

	seed := randomize.NewSeed()
	var err error
	o := &UserLink{}
	if err = randomize.Struct(seed, o, userLinkDBTypes, true, userLinkColumnsWithDefault...); err != nil {
		t.Errorf("Unable to randomize UserLink struct: %s", err)
	}

	ctx := context.Background()
	tx := MustTx(boil.BeginTx(ctx, nil))
	defer func() { _ = tx.Rollback() }()
	if err = o.Insert(ctx, tx, boil.Infer()); err != nil {
		t.Error(err)
	}

	count, err := UserLinks().Count(ctx, tx)
	if err != nil {
		t.Error(err)
	}

	if count != 1 {
		t.Error("want one record, got:", count)
	}
}

func testUserLinksInsertWhitelist(t *testing.T) {
	t.Parallel()

	seed := randomize.NewSeed()
	var err error
	o := &UserLink{}
	if err = randomize.Struct(seed, o, userLinkDBTypes, true); err != nil {
		t.Errorf("Unable to randomize UserLink struct: %s", err)
	}

	ctx := context.Background()
	tx := MustTx(boil.BeginTx(ctx, nil))
	defer func() { _ = tx.Rollback() }()
	if err = o.Insert(ctx, tx, boil.Whitelist(userLinkColumnsWithoutDefault...)); err != nil {
		t.Error(err)
	}

	count, err := UserLinks().Count(ctx, tx)
	if err != nil {
		t.Error(err)
	}

	if count != 1 {
		t.Error("want one record, got:", count)
	}
}

func testUserLinkToManyUserLinkTopics(t *testing.T) {
	var err error
	ctx := context.Background()
	tx := MustTx(boil.BeginTx(ctx, nil))
	defer func() { _ = tx.Rollback() }()

	var a UserLink
	var b, c UserLinkTopic

	seed := randomize.NewSeed()
	if err = randomize.Struct(seed, &a, userLinkDBTypes, true, userLinkColumnsWithDefault...); err != nil {
		t.Errorf("Unable to randomize UserLink struct: %s", err)
	}

	if err := a.Insert(ctx, tx, boil.Infer()); err != nil {
		t.Fatal(err)
	}

	if err = randomize.Struct(seed, &b, userLinkTopicDBTypes, false, userLinkTopicColumnsWithDefault...); err != nil {
		t.Fatal(err)
	}
	if err = randomize.Struct(seed, &c, userLinkTopicDBTypes, false, userLinkTopicColumnsWithDefault...); err != nil {
		t.Fatal(err)
	}

	b.UserLinkID = a.ID
	c.UserLinkID = a.ID

	if err = b.Insert(ctx, tx, boil.Infer()); err != nil {
		t.Fatal(err)
	}
	if err = c.Insert(ctx, tx, boil.Infer()); err != nil {
		t.Fatal(err)
	}

	check, err := a.UserLinkTopics().All(ctx, tx)
	if err != nil {
		t.Fatal(err)
	}

	bFound, cFound := false, false
	for _, v := range check {
		if v.UserLinkID == b.UserLinkID {
			bFound = true
		}
		if v.UserLinkID == c.UserLinkID {
			cFound = true
		}
	}

	if !bFound {
		t.Error("expected to find b")
	}
	if !cFound {
		t.Error("expected to find c")
	}

	slice := UserLinkSlice{&a}
	if err = a.L.LoadUserLinkTopics(ctx, tx, false, (*[]*UserLink)(&slice), nil); err != nil {
		t.Fatal(err)
	}
	if got := len(a.R.UserLinkTopics); got != 2 {
		t.Error("number of eager loaded records wrong, got:", got)
	}

	a.R.UserLinkTopics = nil
	if err = a.L.LoadUserLinkTopics(ctx, tx, true, &a, nil); err != nil {
		t.Fatal(err)
	}
	if got := len(a.R.UserLinkTopics); got != 2 {
		t.Error("number of eager loaded records wrong, got:", got)
	}

	if t.Failed() {
		t.Logf("%#v", check)
	}
}

func testUserLinkToManyAddOpUserLinkTopics(t *testing.T) {
	var err error

	ctx := context.Background()
	tx := MustTx(boil.BeginTx(ctx, nil))
	defer func() { _ = tx.Rollback() }()

	var a UserLink
	var b, c, d, e UserLinkTopic

	seed := randomize.NewSeed()
	if err = randomize.Struct(seed, &a, userLinkDBTypes, false, strmangle.SetComplement(userLinkPrimaryKeyColumns, userLinkColumnsWithoutDefault)...); err != nil {
		t.Fatal(err)
	}
	foreigners := []*UserLinkTopic{&b, &c, &d, &e}
	for _, x := range foreigners {
		if err = randomize.Struct(seed, x, userLinkTopicDBTypes, false, strmangle.SetComplement(userLinkTopicPrimaryKeyColumns, userLinkTopicColumnsWithoutDefault)...); err != nil {
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

	foreignersSplitByInsertion := [][]*UserLinkTopic{
		{&b, &c},
		{&d, &e},
	}

	for i, x := range foreignersSplitByInsertion {
		err = a.AddUserLinkTopics(ctx, tx, i != 0, x...)
		if err != nil {
			t.Fatal(err)
		}

		first := x[0]
		second := x[1]

		if a.ID != first.UserLinkID {
			t.Error("foreign key was wrong value", a.ID, first.UserLinkID)
		}
		if a.ID != second.UserLinkID {
			t.Error("foreign key was wrong value", a.ID, second.UserLinkID)
		}

		if first.R.UserLink != &a {
			t.Error("relationship was not added properly to the foreign slice")
		}
		if second.R.UserLink != &a {
			t.Error("relationship was not added properly to the foreign slice")
		}

		if a.R.UserLinkTopics[i*2] != first {
			t.Error("relationship struct slice not set to correct value")
		}
		if a.R.UserLinkTopics[i*2+1] != second {
			t.Error("relationship struct slice not set to correct value")
		}

		count, err := a.UserLinkTopics().Count(ctx, tx)
		if err != nil {
			t.Fatal(err)
		}
		if want := int64((i + 1) * 2); count != want {
			t.Error("want", want, "got", count)
		}
	}
}
func testUserLinkToOneLinkUsingLink(t *testing.T) {
	ctx := context.Background()
	tx := MustTx(boil.BeginTx(ctx, nil))
	defer func() { _ = tx.Rollback() }()

	var local UserLink
	var foreign Link

	seed := randomize.NewSeed()
	if err := randomize.Struct(seed, &local, userLinkDBTypes, false, userLinkColumnsWithDefault...); err != nil {
		t.Errorf("Unable to randomize UserLink struct: %s", err)
	}
	if err := randomize.Struct(seed, &foreign, linkDBTypes, false, linkColumnsWithDefault...); err != nil {
		t.Errorf("Unable to randomize Link struct: %s", err)
	}

	if err := foreign.Insert(ctx, tx, boil.Infer()); err != nil {
		t.Fatal(err)
	}

	local.LinkID = foreign.ID
	if err := local.Insert(ctx, tx, boil.Infer()); err != nil {
		t.Fatal(err)
	}

	check, err := local.Link().One(ctx, tx)
	if err != nil {
		t.Fatal(err)
	}

	if check.ID != foreign.ID {
		t.Errorf("want: %v, got %v", foreign.ID, check.ID)
	}

	slice := UserLinkSlice{&local}
	if err = local.L.LoadLink(ctx, tx, false, (*[]*UserLink)(&slice), nil); err != nil {
		t.Fatal(err)
	}
	if local.R.Link == nil {
		t.Error("struct should have been eager loaded")
	}

	local.R.Link = nil
	if err = local.L.LoadLink(ctx, tx, true, &local, nil); err != nil {
		t.Fatal(err)
	}
	if local.R.Link == nil {
		t.Error("struct should have been eager loaded")
	}
}

func testUserLinkToOneOrganizationUsingOrganization(t *testing.T) {
	ctx := context.Background()
	tx := MustTx(boil.BeginTx(ctx, nil))
	defer func() { _ = tx.Rollback() }()

	var local UserLink
	var foreign Organization

	seed := randomize.NewSeed()
	if err := randomize.Struct(seed, &local, userLinkDBTypes, false, userLinkColumnsWithDefault...); err != nil {
		t.Errorf("Unable to randomize UserLink struct: %s", err)
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

	slice := UserLinkSlice{&local}
	if err = local.L.LoadOrganization(ctx, tx, false, (*[]*UserLink)(&slice), nil); err != nil {
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

func testUserLinkToOneRepositoryUsingRepository(t *testing.T) {
	ctx := context.Background()
	tx := MustTx(boil.BeginTx(ctx, nil))
	defer func() { _ = tx.Rollback() }()

	var local UserLink
	var foreign Repository

	seed := randomize.NewSeed()
	if err := randomize.Struct(seed, &local, userLinkDBTypes, false, userLinkColumnsWithDefault...); err != nil {
		t.Errorf("Unable to randomize UserLink struct: %s", err)
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

	slice := UserLinkSlice{&local}
	if err = local.L.LoadRepository(ctx, tx, false, (*[]*UserLink)(&slice), nil); err != nil {
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

func testUserLinkToOneUserUsingUser(t *testing.T) {
	ctx := context.Background()
	tx := MustTx(boil.BeginTx(ctx, nil))
	defer func() { _ = tx.Rollback() }()

	var local UserLink
	var foreign User

	seed := randomize.NewSeed()
	if err := randomize.Struct(seed, &local, userLinkDBTypes, false, userLinkColumnsWithDefault...); err != nil {
		t.Errorf("Unable to randomize UserLink struct: %s", err)
	}
	if err := randomize.Struct(seed, &foreign, userDBTypes, false, userColumnsWithDefault...); err != nil {
		t.Errorf("Unable to randomize User struct: %s", err)
	}

	if err := foreign.Insert(ctx, tx, boil.Infer()); err != nil {
		t.Fatal(err)
	}

	local.UserID = foreign.ID
	if err := local.Insert(ctx, tx, boil.Infer()); err != nil {
		t.Fatal(err)
	}

	check, err := local.User().One(ctx, tx)
	if err != nil {
		t.Fatal(err)
	}

	if check.ID != foreign.ID {
		t.Errorf("want: %v, got %v", foreign.ID, check.ID)
	}

	slice := UserLinkSlice{&local}
	if err = local.L.LoadUser(ctx, tx, false, (*[]*UserLink)(&slice), nil); err != nil {
		t.Fatal(err)
	}
	if local.R.User == nil {
		t.Error("struct should have been eager loaded")
	}

	local.R.User = nil
	if err = local.L.LoadUser(ctx, tx, true, &local, nil); err != nil {
		t.Fatal(err)
	}
	if local.R.User == nil {
		t.Error("struct should have been eager loaded")
	}
}

func testUserLinkToOneSetOpLinkUsingLink(t *testing.T) {
	var err error

	ctx := context.Background()
	tx := MustTx(boil.BeginTx(ctx, nil))
	defer func() { _ = tx.Rollback() }()

	var a UserLink
	var b, c Link

	seed := randomize.NewSeed()
	if err = randomize.Struct(seed, &a, userLinkDBTypes, false, strmangle.SetComplement(userLinkPrimaryKeyColumns, userLinkColumnsWithoutDefault)...); err != nil {
		t.Fatal(err)
	}
	if err = randomize.Struct(seed, &b, linkDBTypes, false, strmangle.SetComplement(linkPrimaryKeyColumns, linkColumnsWithoutDefault)...); err != nil {
		t.Fatal(err)
	}
	if err = randomize.Struct(seed, &c, linkDBTypes, false, strmangle.SetComplement(linkPrimaryKeyColumns, linkColumnsWithoutDefault)...); err != nil {
		t.Fatal(err)
	}

	if err := a.Insert(ctx, tx, boil.Infer()); err != nil {
		t.Fatal(err)
	}
	if err = b.Insert(ctx, tx, boil.Infer()); err != nil {
		t.Fatal(err)
	}

	for i, x := range []*Link{&b, &c} {
		err = a.SetLink(ctx, tx, i != 0, x)
		if err != nil {
			t.Fatal(err)
		}

		if a.R.Link != x {
			t.Error("relationship struct not set to correct value")
		}

		if x.R.UserLinks[0] != &a {
			t.Error("failed to append to foreign relationship struct")
		}
		if a.LinkID != x.ID {
			t.Error("foreign key was wrong value", a.LinkID)
		}

		zero := reflect.Zero(reflect.TypeOf(a.LinkID))
		reflect.Indirect(reflect.ValueOf(&a.LinkID)).Set(zero)

		if err = a.Reload(ctx, tx); err != nil {
			t.Fatal("failed to reload", err)
		}

		if a.LinkID != x.ID {
			t.Error("foreign key was wrong value", a.LinkID, x.ID)
		}
	}
}
func testUserLinkToOneSetOpOrganizationUsingOrganization(t *testing.T) {
	var err error

	ctx := context.Background()
	tx := MustTx(boil.BeginTx(ctx, nil))
	defer func() { _ = tx.Rollback() }()

	var a UserLink
	var b, c Organization

	seed := randomize.NewSeed()
	if err = randomize.Struct(seed, &a, userLinkDBTypes, false, strmangle.SetComplement(userLinkPrimaryKeyColumns, userLinkColumnsWithoutDefault)...); err != nil {
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

		if x.R.UserLinks[0] != &a {
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
func testUserLinkToOneSetOpRepositoryUsingRepository(t *testing.T) {
	var err error

	ctx := context.Background()
	tx := MustTx(boil.BeginTx(ctx, nil))
	defer func() { _ = tx.Rollback() }()

	var a UserLink
	var b, c Repository

	seed := randomize.NewSeed()
	if err = randomize.Struct(seed, &a, userLinkDBTypes, false, strmangle.SetComplement(userLinkPrimaryKeyColumns, userLinkColumnsWithoutDefault)...); err != nil {
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

		if x.R.UserLinks[0] != &a {
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
func testUserLinkToOneSetOpUserUsingUser(t *testing.T) {
	var err error

	ctx := context.Background()
	tx := MustTx(boil.BeginTx(ctx, nil))
	defer func() { _ = tx.Rollback() }()

	var a UserLink
	var b, c User

	seed := randomize.NewSeed()
	if err = randomize.Struct(seed, &a, userLinkDBTypes, false, strmangle.SetComplement(userLinkPrimaryKeyColumns, userLinkColumnsWithoutDefault)...); err != nil {
		t.Fatal(err)
	}
	if err = randomize.Struct(seed, &b, userDBTypes, false, strmangle.SetComplement(userPrimaryKeyColumns, userColumnsWithoutDefault)...); err != nil {
		t.Fatal(err)
	}
	if err = randomize.Struct(seed, &c, userDBTypes, false, strmangle.SetComplement(userPrimaryKeyColumns, userColumnsWithoutDefault)...); err != nil {
		t.Fatal(err)
	}

	if err := a.Insert(ctx, tx, boil.Infer()); err != nil {
		t.Fatal(err)
	}
	if err = b.Insert(ctx, tx, boil.Infer()); err != nil {
		t.Fatal(err)
	}

	for i, x := range []*User{&b, &c} {
		err = a.SetUser(ctx, tx, i != 0, x)
		if err != nil {
			t.Fatal(err)
		}

		if a.R.User != x {
			t.Error("relationship struct not set to correct value")
		}

		if x.R.UserLinks[0] != &a {
			t.Error("failed to append to foreign relationship struct")
		}
		if a.UserID != x.ID {
			t.Error("foreign key was wrong value", a.UserID)
		}

		zero := reflect.Zero(reflect.TypeOf(a.UserID))
		reflect.Indirect(reflect.ValueOf(&a.UserID)).Set(zero)

		if err = a.Reload(ctx, tx); err != nil {
			t.Fatal("failed to reload", err)
		}

		if a.UserID != x.ID {
			t.Error("foreign key was wrong value", a.UserID, x.ID)
		}
	}
}

func testUserLinksReload(t *testing.T) {
	t.Parallel()

	seed := randomize.NewSeed()
	var err error
	o := &UserLink{}
	if err = randomize.Struct(seed, o, userLinkDBTypes, true, userLinkColumnsWithDefault...); err != nil {
		t.Errorf("Unable to randomize UserLink struct: %s", err)
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

func testUserLinksReloadAll(t *testing.T) {
	t.Parallel()

	seed := randomize.NewSeed()
	var err error
	o := &UserLink{}
	if err = randomize.Struct(seed, o, userLinkDBTypes, true, userLinkColumnsWithDefault...); err != nil {
		t.Errorf("Unable to randomize UserLink struct: %s", err)
	}

	ctx := context.Background()
	tx := MustTx(boil.BeginTx(ctx, nil))
	defer func() { _ = tx.Rollback() }()
	if err = o.Insert(ctx, tx, boil.Infer()); err != nil {
		t.Error(err)
	}

	slice := UserLinkSlice{o}

	if err = slice.ReloadAll(ctx, tx); err != nil {
		t.Error(err)
	}
}

func testUserLinksSelect(t *testing.T) {
	t.Parallel()

	seed := randomize.NewSeed()
	var err error
	o := &UserLink{}
	if err = randomize.Struct(seed, o, userLinkDBTypes, true, userLinkColumnsWithDefault...); err != nil {
		t.Errorf("Unable to randomize UserLink struct: %s", err)
	}

	ctx := context.Background()
	tx := MustTx(boil.BeginTx(ctx, nil))
	defer func() { _ = tx.Rollback() }()
	if err = o.Insert(ctx, tx, boil.Infer()); err != nil {
		t.Error(err)
	}

	slice, err := UserLinks().All(ctx, tx)
	if err != nil {
		t.Error(err)
	}

	if len(slice) != 1 {
		t.Error("want one record, got:", len(slice))
	}
}

var (
	userLinkDBTypes = map[string]string{`ID`: `uuid`, `OrganizationID`: `uuid`, `RepositoryID`: `uuid`, `UserID`: `uuid`, `LinkID`: `uuid`, `CreatedAt`: `timestamp with time zone`, `Action`: `enum.action('upsert_link','delete_link')`}
	_               = bytes.MinRead
)

func testUserLinksUpdate(t *testing.T) {
	t.Parallel()

	if 0 == len(userLinkPrimaryKeyColumns) {
		t.Skip("Skipping table with no primary key columns")
	}
	if len(userLinkAllColumns) == len(userLinkPrimaryKeyColumns) {
		t.Skip("Skipping table with only primary key columns")
	}

	seed := randomize.NewSeed()
	var err error
	o := &UserLink{}
	if err = randomize.Struct(seed, o, userLinkDBTypes, true, userLinkColumnsWithDefault...); err != nil {
		t.Errorf("Unable to randomize UserLink struct: %s", err)
	}

	ctx := context.Background()
	tx := MustTx(boil.BeginTx(ctx, nil))
	defer func() { _ = tx.Rollback() }()
	if err = o.Insert(ctx, tx, boil.Infer()); err != nil {
		t.Error(err)
	}

	count, err := UserLinks().Count(ctx, tx)
	if err != nil {
		t.Error(err)
	}

	if count != 1 {
		t.Error("want one record, got:", count)
	}

	if err = randomize.Struct(seed, o, userLinkDBTypes, true, userLinkPrimaryKeyColumns...); err != nil {
		t.Errorf("Unable to randomize UserLink struct: %s", err)
	}

	if rowsAff, err := o.Update(ctx, tx, boil.Infer()); err != nil {
		t.Error(err)
	} else if rowsAff != 1 {
		t.Error("should only affect one row but affected", rowsAff)
	}
}

func testUserLinksSliceUpdateAll(t *testing.T) {
	t.Parallel()

	if len(userLinkAllColumns) == len(userLinkPrimaryKeyColumns) {
		t.Skip("Skipping table with only primary key columns")
	}

	seed := randomize.NewSeed()
	var err error
	o := &UserLink{}
	if err = randomize.Struct(seed, o, userLinkDBTypes, true, userLinkColumnsWithDefault...); err != nil {
		t.Errorf("Unable to randomize UserLink struct: %s", err)
	}

	ctx := context.Background()
	tx := MustTx(boil.BeginTx(ctx, nil))
	defer func() { _ = tx.Rollback() }()
	if err = o.Insert(ctx, tx, boil.Infer()); err != nil {
		t.Error(err)
	}

	count, err := UserLinks().Count(ctx, tx)
	if err != nil {
		t.Error(err)
	}

	if count != 1 {
		t.Error("want one record, got:", count)
	}

	if err = randomize.Struct(seed, o, userLinkDBTypes, true, userLinkPrimaryKeyColumns...); err != nil {
		t.Errorf("Unable to randomize UserLink struct: %s", err)
	}

	// Remove Primary keys and unique columns from what we plan to update
	var fields []string
	if strmangle.StringSliceMatch(userLinkAllColumns, userLinkPrimaryKeyColumns) {
		fields = userLinkAllColumns
	} else {
		fields = strmangle.SetComplement(
			userLinkAllColumns,
			userLinkPrimaryKeyColumns,
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

	slice := UserLinkSlice{o}
	if rowsAff, err := slice.UpdateAll(ctx, tx, updateMap); err != nil {
		t.Error(err)
	} else if rowsAff != 1 {
		t.Error("wanted one record updated but got", rowsAff)
	}
}

func testUserLinksUpsert(t *testing.T) {
	t.Parallel()

	if len(userLinkAllColumns) == len(userLinkPrimaryKeyColumns) {
		t.Skip("Skipping table with only primary key columns")
	}

	seed := randomize.NewSeed()
	var err error
	// Attempt the INSERT side of an UPSERT
	o := UserLink{}
	if err = randomize.Struct(seed, &o, userLinkDBTypes, true); err != nil {
		t.Errorf("Unable to randomize UserLink struct: %s", err)
	}

	ctx := context.Background()
	tx := MustTx(boil.BeginTx(ctx, nil))
	defer func() { _ = tx.Rollback() }()
	if err = o.Upsert(ctx, tx, false, nil, boil.Infer(), boil.Infer()); err != nil {
		t.Errorf("Unable to upsert UserLink: %s", err)
	}

	count, err := UserLinks().Count(ctx, tx)
	if err != nil {
		t.Error(err)
	}
	if count != 1 {
		t.Error("want one record, got:", count)
	}

	// Attempt the UPDATE side of an UPSERT
	if err = randomize.Struct(seed, &o, userLinkDBTypes, false, userLinkPrimaryKeyColumns...); err != nil {
		t.Errorf("Unable to randomize UserLink struct: %s", err)
	}

	if err = o.Upsert(ctx, tx, true, nil, boil.Infer(), boil.Infer()); err != nil {
		t.Errorf("Unable to upsert UserLink: %s", err)
	}

	count, err = UserLinks().Count(ctx, tx)
	if err != nil {
		t.Error(err)
	}
	if count != 1 {
		t.Error("want one record, got:", count)
	}
}
