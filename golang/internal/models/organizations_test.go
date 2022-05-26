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

func testOrganizations(t *testing.T) {
	t.Parallel()

	query := Organizations()

	if query.Query == nil {
		t.Error("expected a query, got nothing")
	}
}

func testOrganizationsDelete(t *testing.T) {
	t.Parallel()

	seed := randomize.NewSeed()
	var err error
	o := &Organization{}
	if err = randomize.Struct(seed, o, organizationDBTypes, true, organizationColumnsWithDefault...); err != nil {
		t.Errorf("Unable to randomize Organization struct: %s", err)
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

	count, err := Organizations().Count(ctx, tx)
	if err != nil {
		t.Error(err)
	}

	if count != 0 {
		t.Error("want zero records, got:", count)
	}
}

func testOrganizationsQueryDeleteAll(t *testing.T) {
	t.Parallel()

	seed := randomize.NewSeed()
	var err error
	o := &Organization{}
	if err = randomize.Struct(seed, o, organizationDBTypes, true, organizationColumnsWithDefault...); err != nil {
		t.Errorf("Unable to randomize Organization struct: %s", err)
	}

	ctx := context.Background()
	tx := MustTx(boil.BeginTx(ctx, nil))
	defer func() { _ = tx.Rollback() }()
	if err = o.Insert(ctx, tx, boil.Infer()); err != nil {
		t.Error(err)
	}

	if rowsAff, err := Organizations().DeleteAll(ctx, tx); err != nil {
		t.Error(err)
	} else if rowsAff != 1 {
		t.Error("should only have deleted one row, but affected:", rowsAff)
	}

	count, err := Organizations().Count(ctx, tx)
	if err != nil {
		t.Error(err)
	}

	if count != 0 {
		t.Error("want zero records, got:", count)
	}
}

func testOrganizationsSliceDeleteAll(t *testing.T) {
	t.Parallel()

	seed := randomize.NewSeed()
	var err error
	o := &Organization{}
	if err = randomize.Struct(seed, o, organizationDBTypes, true, organizationColumnsWithDefault...); err != nil {
		t.Errorf("Unable to randomize Organization struct: %s", err)
	}

	ctx := context.Background()
	tx := MustTx(boil.BeginTx(ctx, nil))
	defer func() { _ = tx.Rollback() }()
	if err = o.Insert(ctx, tx, boil.Infer()); err != nil {
		t.Error(err)
	}

	slice := OrganizationSlice{o}

	if rowsAff, err := slice.DeleteAll(ctx, tx); err != nil {
		t.Error(err)
	} else if rowsAff != 1 {
		t.Error("should only have deleted one row, but affected:", rowsAff)
	}

	count, err := Organizations().Count(ctx, tx)
	if err != nil {
		t.Error(err)
	}

	if count != 0 {
		t.Error("want zero records, got:", count)
	}
}

func testOrganizationsExists(t *testing.T) {
	t.Parallel()

	seed := randomize.NewSeed()
	var err error
	o := &Organization{}
	if err = randomize.Struct(seed, o, organizationDBTypes, true, organizationColumnsWithDefault...); err != nil {
		t.Errorf("Unable to randomize Organization struct: %s", err)
	}

	ctx := context.Background()
	tx := MustTx(boil.BeginTx(ctx, nil))
	defer func() { _ = tx.Rollback() }()
	if err = o.Insert(ctx, tx, boil.Infer()); err != nil {
		t.Error(err)
	}

	e, err := OrganizationExists(ctx, tx, o.ID)
	if err != nil {
		t.Errorf("Unable to check if Organization exists: %s", err)
	}
	if !e {
		t.Errorf("Expected OrganizationExists to return true, but got false.")
	}
}

func testOrganizationsFind(t *testing.T) {
	t.Parallel()

	seed := randomize.NewSeed()
	var err error
	o := &Organization{}
	if err = randomize.Struct(seed, o, organizationDBTypes, true, organizationColumnsWithDefault...); err != nil {
		t.Errorf("Unable to randomize Organization struct: %s", err)
	}

	ctx := context.Background()
	tx := MustTx(boil.BeginTx(ctx, nil))
	defer func() { _ = tx.Rollback() }()
	if err = o.Insert(ctx, tx, boil.Infer()); err != nil {
		t.Error(err)
	}

	organizationFound, err := FindOrganization(ctx, tx, o.ID)
	if err != nil {
		t.Error(err)
	}

	if organizationFound == nil {
		t.Error("want a record, got nil")
	}
}

func testOrganizationsBind(t *testing.T) {
	t.Parallel()

	seed := randomize.NewSeed()
	var err error
	o := &Organization{}
	if err = randomize.Struct(seed, o, organizationDBTypes, true, organizationColumnsWithDefault...); err != nil {
		t.Errorf("Unable to randomize Organization struct: %s", err)
	}

	ctx := context.Background()
	tx := MustTx(boil.BeginTx(ctx, nil))
	defer func() { _ = tx.Rollback() }()
	if err = o.Insert(ctx, tx, boil.Infer()); err != nil {
		t.Error(err)
	}

	if err = Organizations().Bind(ctx, tx, o); err != nil {
		t.Error(err)
	}
}

func testOrganizationsOne(t *testing.T) {
	t.Parallel()

	seed := randomize.NewSeed()
	var err error
	o := &Organization{}
	if err = randomize.Struct(seed, o, organizationDBTypes, true, organizationColumnsWithDefault...); err != nil {
		t.Errorf("Unable to randomize Organization struct: %s", err)
	}

	ctx := context.Background()
	tx := MustTx(boil.BeginTx(ctx, nil))
	defer func() { _ = tx.Rollback() }()
	if err = o.Insert(ctx, tx, boil.Infer()); err != nil {
		t.Error(err)
	}

	if x, err := Organizations().One(ctx, tx); err != nil {
		t.Error(err)
	} else if x == nil {
		t.Error("expected to get a non nil record")
	}
}

func testOrganizationsAll(t *testing.T) {
	t.Parallel()

	seed := randomize.NewSeed()
	var err error
	organizationOne := &Organization{}
	organizationTwo := &Organization{}
	if err = randomize.Struct(seed, organizationOne, organizationDBTypes, false, organizationColumnsWithDefault...); err != nil {
		t.Errorf("Unable to randomize Organization struct: %s", err)
	}
	if err = randomize.Struct(seed, organizationTwo, organizationDBTypes, false, organizationColumnsWithDefault...); err != nil {
		t.Errorf("Unable to randomize Organization struct: %s", err)
	}

	ctx := context.Background()
	tx := MustTx(boil.BeginTx(ctx, nil))
	defer func() { _ = tx.Rollback() }()
	if err = organizationOne.Insert(ctx, tx, boil.Infer()); err != nil {
		t.Error(err)
	}
	if err = organizationTwo.Insert(ctx, tx, boil.Infer()); err != nil {
		t.Error(err)
	}

	slice, err := Organizations().All(ctx, tx)
	if err != nil {
		t.Error(err)
	}

	if len(slice) != 2 {
		t.Error("want 2 records, got:", len(slice))
	}
}

func testOrganizationsCount(t *testing.T) {
	t.Parallel()

	var err error
	seed := randomize.NewSeed()
	organizationOne := &Organization{}
	organizationTwo := &Organization{}
	if err = randomize.Struct(seed, organizationOne, organizationDBTypes, false, organizationColumnsWithDefault...); err != nil {
		t.Errorf("Unable to randomize Organization struct: %s", err)
	}
	if err = randomize.Struct(seed, organizationTwo, organizationDBTypes, false, organizationColumnsWithDefault...); err != nil {
		t.Errorf("Unable to randomize Organization struct: %s", err)
	}

	ctx := context.Background()
	tx := MustTx(boil.BeginTx(ctx, nil))
	defer func() { _ = tx.Rollback() }()
	if err = organizationOne.Insert(ctx, tx, boil.Infer()); err != nil {
		t.Error(err)
	}
	if err = organizationTwo.Insert(ctx, tx, boil.Infer()); err != nil {
		t.Error(err)
	}

	count, err := Organizations().Count(ctx, tx)
	if err != nil {
		t.Error(err)
	}

	if count != 2 {
		t.Error("want 2 records, got:", count)
	}
}

func testOrganizationsInsert(t *testing.T) {
	t.Parallel()

	seed := randomize.NewSeed()
	var err error
	o := &Organization{}
	if err = randomize.Struct(seed, o, organizationDBTypes, true, organizationColumnsWithDefault...); err != nil {
		t.Errorf("Unable to randomize Organization struct: %s", err)
	}

	ctx := context.Background()
	tx := MustTx(boil.BeginTx(ctx, nil))
	defer func() { _ = tx.Rollback() }()
	if err = o.Insert(ctx, tx, boil.Infer()); err != nil {
		t.Error(err)
	}

	count, err := Organizations().Count(ctx, tx)
	if err != nil {
		t.Error(err)
	}

	if count != 1 {
		t.Error("want one record, got:", count)
	}
}

func testOrganizationsInsertWhitelist(t *testing.T) {
	t.Parallel()

	seed := randomize.NewSeed()
	var err error
	o := &Organization{}
	if err = randomize.Struct(seed, o, organizationDBTypes, true); err != nil {
		t.Errorf("Unable to randomize Organization struct: %s", err)
	}

	ctx := context.Background()
	tx := MustTx(boil.BeginTx(ctx, nil))
	defer func() { _ = tx.Rollback() }()
	if err = o.Insert(ctx, tx, boil.Whitelist(organizationColumnsWithoutDefault...)); err != nil {
		t.Error(err)
	}

	count, err := Organizations().Count(ctx, tx)
	if err != nil {
		t.Error(err)
	}

	if count != 1 {
		t.Error("want one record, got:", count)
	}
}

func testOrganizationToManyLinks(t *testing.T) {
	var err error
	ctx := context.Background()
	tx := MustTx(boil.BeginTx(ctx, nil))
	defer func() { _ = tx.Rollback() }()

	var a Organization
	var b, c Link

	seed := randomize.NewSeed()
	if err = randomize.Struct(seed, &a, organizationDBTypes, true, organizationColumnsWithDefault...); err != nil {
		t.Errorf("Unable to randomize Organization struct: %s", err)
	}

	if err := a.Insert(ctx, tx, boil.Infer()); err != nil {
		t.Fatal(err)
	}

	if err = randomize.Struct(seed, &b, linkDBTypes, false, linkColumnsWithDefault...); err != nil {
		t.Fatal(err)
	}
	if err = randomize.Struct(seed, &c, linkDBTypes, false, linkColumnsWithDefault...); err != nil {
		t.Fatal(err)
	}

	b.OrganizationID = a.ID
	c.OrganizationID = a.ID

	if err = b.Insert(ctx, tx, boil.Infer()); err != nil {
		t.Fatal(err)
	}
	if err = c.Insert(ctx, tx, boil.Infer()); err != nil {
		t.Fatal(err)
	}

	check, err := a.Links().All(ctx, tx)
	if err != nil {
		t.Fatal(err)
	}

	bFound, cFound := false, false
	for _, v := range check {
		if v.OrganizationID == b.OrganizationID {
			bFound = true
		}
		if v.OrganizationID == c.OrganizationID {
			cFound = true
		}
	}

	if !bFound {
		t.Error("expected to find b")
	}
	if !cFound {
		t.Error("expected to find c")
	}

	slice := OrganizationSlice{&a}
	if err = a.L.LoadLinks(ctx, tx, false, (*[]*Organization)(&slice), nil); err != nil {
		t.Fatal(err)
	}
	if got := len(a.R.Links); got != 2 {
		t.Error("number of eager loaded records wrong, got:", got)
	}

	a.R.Links = nil
	if err = a.L.LoadLinks(ctx, tx, true, &a, nil); err != nil {
		t.Fatal(err)
	}
	if got := len(a.R.Links); got != 2 {
		t.Error("number of eager loaded records wrong, got:", got)
	}

	if t.Failed() {
		t.Logf("%#v", check)
	}
}

func testOrganizationToManyOrganizationMembers(t *testing.T) {
	var err error
	ctx := context.Background()
	tx := MustTx(boil.BeginTx(ctx, nil))
	defer func() { _ = tx.Rollback() }()

	var a Organization
	var b, c OrganizationMember

	seed := randomize.NewSeed()
	if err = randomize.Struct(seed, &a, organizationDBTypes, true, organizationColumnsWithDefault...); err != nil {
		t.Errorf("Unable to randomize Organization struct: %s", err)
	}

	if err := a.Insert(ctx, tx, boil.Infer()); err != nil {
		t.Fatal(err)
	}

	if err = randomize.Struct(seed, &b, organizationMemberDBTypes, false, organizationMemberColumnsWithDefault...); err != nil {
		t.Fatal(err)
	}
	if err = randomize.Struct(seed, &c, organizationMemberDBTypes, false, organizationMemberColumnsWithDefault...); err != nil {
		t.Fatal(err)
	}

	b.OrganizationID = a.ID
	c.OrganizationID = a.ID

	if err = b.Insert(ctx, tx, boil.Infer()); err != nil {
		t.Fatal(err)
	}
	if err = c.Insert(ctx, tx, boil.Infer()); err != nil {
		t.Fatal(err)
	}

	check, err := a.OrganizationMembers().All(ctx, tx)
	if err != nil {
		t.Fatal(err)
	}

	bFound, cFound := false, false
	for _, v := range check {
		if v.OrganizationID == b.OrganizationID {
			bFound = true
		}
		if v.OrganizationID == c.OrganizationID {
			cFound = true
		}
	}

	if !bFound {
		t.Error("expected to find b")
	}
	if !cFound {
		t.Error("expected to find c")
	}

	slice := OrganizationSlice{&a}
	if err = a.L.LoadOrganizationMembers(ctx, tx, false, (*[]*Organization)(&slice), nil); err != nil {
		t.Fatal(err)
	}
	if got := len(a.R.OrganizationMembers); got != 2 {
		t.Error("number of eager loaded records wrong, got:", got)
	}

	a.R.OrganizationMembers = nil
	if err = a.L.LoadOrganizationMembers(ctx, tx, true, &a, nil); err != nil {
		t.Fatal(err)
	}
	if got := len(a.R.OrganizationMembers); got != 2 {
		t.Error("number of eager loaded records wrong, got:", got)
	}

	if t.Failed() {
		t.Logf("%#v", check)
	}
}

func testOrganizationToManyRepositories(t *testing.T) {
	var err error
	ctx := context.Background()
	tx := MustTx(boil.BeginTx(ctx, nil))
	defer func() { _ = tx.Rollback() }()

	var a Organization
	var b, c Repository

	seed := randomize.NewSeed()
	if err = randomize.Struct(seed, &a, organizationDBTypes, true, organizationColumnsWithDefault...); err != nil {
		t.Errorf("Unable to randomize Organization struct: %s", err)
	}

	if err := a.Insert(ctx, tx, boil.Infer()); err != nil {
		t.Fatal(err)
	}

	if err = randomize.Struct(seed, &b, repositoryDBTypes, false, repositoryColumnsWithDefault...); err != nil {
		t.Fatal(err)
	}
	if err = randomize.Struct(seed, &c, repositoryDBTypes, false, repositoryColumnsWithDefault...); err != nil {
		t.Fatal(err)
	}

	b.OrganizationID = a.ID
	c.OrganizationID = a.ID

	if err = b.Insert(ctx, tx, boil.Infer()); err != nil {
		t.Fatal(err)
	}
	if err = c.Insert(ctx, tx, boil.Infer()); err != nil {
		t.Fatal(err)
	}

	check, err := a.Repositories().All(ctx, tx)
	if err != nil {
		t.Fatal(err)
	}

	bFound, cFound := false, false
	for _, v := range check {
		if v.OrganizationID == b.OrganizationID {
			bFound = true
		}
		if v.OrganizationID == c.OrganizationID {
			cFound = true
		}
	}

	if !bFound {
		t.Error("expected to find b")
	}
	if !cFound {
		t.Error("expected to find c")
	}

	slice := OrganizationSlice{&a}
	if err = a.L.LoadRepositories(ctx, tx, false, (*[]*Organization)(&slice), nil); err != nil {
		t.Fatal(err)
	}
	if got := len(a.R.Repositories); got != 2 {
		t.Error("number of eager loaded records wrong, got:", got)
	}

	a.R.Repositories = nil
	if err = a.L.LoadRepositories(ctx, tx, true, &a, nil); err != nil {
		t.Fatal(err)
	}
	if got := len(a.R.Repositories); got != 2 {
		t.Error("number of eager loaded records wrong, got:", got)
	}

	if t.Failed() {
		t.Logf("%#v", check)
	}
}

func testOrganizationToManyTopics(t *testing.T) {
	var err error
	ctx := context.Background()
	tx := MustTx(boil.BeginTx(ctx, nil))
	defer func() { _ = tx.Rollback() }()

	var a Organization
	var b, c Topic

	seed := randomize.NewSeed()
	if err = randomize.Struct(seed, &a, organizationDBTypes, true, organizationColumnsWithDefault...); err != nil {
		t.Errorf("Unable to randomize Organization struct: %s", err)
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

	b.OrganizationID = a.ID
	c.OrganizationID = a.ID

	if err = b.Insert(ctx, tx, boil.Infer()); err != nil {
		t.Fatal(err)
	}
	if err = c.Insert(ctx, tx, boil.Infer()); err != nil {
		t.Fatal(err)
	}

	check, err := a.Topics().All(ctx, tx)
	if err != nil {
		t.Fatal(err)
	}

	bFound, cFound := false, false
	for _, v := range check {
		if v.OrganizationID == b.OrganizationID {
			bFound = true
		}
		if v.OrganizationID == c.OrganizationID {
			cFound = true
		}
	}

	if !bFound {
		t.Error("expected to find b")
	}
	if !cFound {
		t.Error("expected to find c")
	}

	slice := OrganizationSlice{&a}
	if err = a.L.LoadTopics(ctx, tx, false, (*[]*Organization)(&slice), nil); err != nil {
		t.Fatal(err)
	}
	if got := len(a.R.Topics); got != 2 {
		t.Error("number of eager loaded records wrong, got:", got)
	}

	a.R.Topics = nil
	if err = a.L.LoadTopics(ctx, tx, true, &a, nil); err != nil {
		t.Fatal(err)
	}
	if got := len(a.R.Topics); got != 2 {
		t.Error("number of eager loaded records wrong, got:", got)
	}

	if t.Failed() {
		t.Logf("%#v", check)
	}
}

func testOrganizationToManyUserLinks(t *testing.T) {
	var err error
	ctx := context.Background()
	tx := MustTx(boil.BeginTx(ctx, nil))
	defer func() { _ = tx.Rollback() }()

	var a Organization
	var b, c UserLink

	seed := randomize.NewSeed()
	if err = randomize.Struct(seed, &a, organizationDBTypes, true, organizationColumnsWithDefault...); err != nil {
		t.Errorf("Unable to randomize Organization struct: %s", err)
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

	b.OrganizationID = a.ID
	c.OrganizationID = a.ID

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
		if v.OrganizationID == b.OrganizationID {
			bFound = true
		}
		if v.OrganizationID == c.OrganizationID {
			cFound = true
		}
	}

	if !bFound {
		t.Error("expected to find b")
	}
	if !cFound {
		t.Error("expected to find c")
	}

	slice := OrganizationSlice{&a}
	if err = a.L.LoadUserLinks(ctx, tx, false, (*[]*Organization)(&slice), nil); err != nil {
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

func testOrganizationToManyAddOpLinks(t *testing.T) {
	var err error

	ctx := context.Background()
	tx := MustTx(boil.BeginTx(ctx, nil))
	defer func() { _ = tx.Rollback() }()

	var a Organization
	var b, c, d, e Link

	seed := randomize.NewSeed()
	if err = randomize.Struct(seed, &a, organizationDBTypes, false, strmangle.SetComplement(organizationPrimaryKeyColumns, organizationColumnsWithoutDefault)...); err != nil {
		t.Fatal(err)
	}
	foreigners := []*Link{&b, &c, &d, &e}
	for _, x := range foreigners {
		if err = randomize.Struct(seed, x, linkDBTypes, false, strmangle.SetComplement(linkPrimaryKeyColumns, linkColumnsWithoutDefault)...); err != nil {
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

	foreignersSplitByInsertion := [][]*Link{
		{&b, &c},
		{&d, &e},
	}

	for i, x := range foreignersSplitByInsertion {
		err = a.AddLinks(ctx, tx, i != 0, x...)
		if err != nil {
			t.Fatal(err)
		}

		first := x[0]
		second := x[1]

		if a.ID != first.OrganizationID {
			t.Error("foreign key was wrong value", a.ID, first.OrganizationID)
		}
		if a.ID != second.OrganizationID {
			t.Error("foreign key was wrong value", a.ID, second.OrganizationID)
		}

		if first.R.Organization != &a {
			t.Error("relationship was not added properly to the foreign slice")
		}
		if second.R.Organization != &a {
			t.Error("relationship was not added properly to the foreign slice")
		}

		if a.R.Links[i*2] != first {
			t.Error("relationship struct slice not set to correct value")
		}
		if a.R.Links[i*2+1] != second {
			t.Error("relationship struct slice not set to correct value")
		}

		count, err := a.Links().Count(ctx, tx)
		if err != nil {
			t.Fatal(err)
		}
		if want := int64((i + 1) * 2); count != want {
			t.Error("want", want, "got", count)
		}
	}
}
func testOrganizationToManyAddOpOrganizationMembers(t *testing.T) {
	var err error

	ctx := context.Background()
	tx := MustTx(boil.BeginTx(ctx, nil))
	defer func() { _ = tx.Rollback() }()

	var a Organization
	var b, c, d, e OrganizationMember

	seed := randomize.NewSeed()
	if err = randomize.Struct(seed, &a, organizationDBTypes, false, strmangle.SetComplement(organizationPrimaryKeyColumns, organizationColumnsWithoutDefault)...); err != nil {
		t.Fatal(err)
	}
	foreigners := []*OrganizationMember{&b, &c, &d, &e}
	for _, x := range foreigners {
		if err = randomize.Struct(seed, x, organizationMemberDBTypes, false, strmangle.SetComplement(organizationMemberPrimaryKeyColumns, organizationMemberColumnsWithoutDefault)...); err != nil {
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

	foreignersSplitByInsertion := [][]*OrganizationMember{
		{&b, &c},
		{&d, &e},
	}

	for i, x := range foreignersSplitByInsertion {
		err = a.AddOrganizationMembers(ctx, tx, i != 0, x...)
		if err != nil {
			t.Fatal(err)
		}

		first := x[0]
		second := x[1]

		if a.ID != first.OrganizationID {
			t.Error("foreign key was wrong value", a.ID, first.OrganizationID)
		}
		if a.ID != second.OrganizationID {
			t.Error("foreign key was wrong value", a.ID, second.OrganizationID)
		}

		if first.R.Organization != &a {
			t.Error("relationship was not added properly to the foreign slice")
		}
		if second.R.Organization != &a {
			t.Error("relationship was not added properly to the foreign slice")
		}

		if a.R.OrganizationMembers[i*2] != first {
			t.Error("relationship struct slice not set to correct value")
		}
		if a.R.OrganizationMembers[i*2+1] != second {
			t.Error("relationship struct slice not set to correct value")
		}

		count, err := a.OrganizationMembers().Count(ctx, tx)
		if err != nil {
			t.Fatal(err)
		}
		if want := int64((i + 1) * 2); count != want {
			t.Error("want", want, "got", count)
		}
	}
}
func testOrganizationToManyAddOpRepositories(t *testing.T) {
	var err error

	ctx := context.Background()
	tx := MustTx(boil.BeginTx(ctx, nil))
	defer func() { _ = tx.Rollback() }()

	var a Organization
	var b, c, d, e Repository

	seed := randomize.NewSeed()
	if err = randomize.Struct(seed, &a, organizationDBTypes, false, strmangle.SetComplement(organizationPrimaryKeyColumns, organizationColumnsWithoutDefault)...); err != nil {
		t.Fatal(err)
	}
	foreigners := []*Repository{&b, &c, &d, &e}
	for _, x := range foreigners {
		if err = randomize.Struct(seed, x, repositoryDBTypes, false, strmangle.SetComplement(repositoryPrimaryKeyColumns, repositoryColumnsWithoutDefault)...); err != nil {
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

	foreignersSplitByInsertion := [][]*Repository{
		{&b, &c},
		{&d, &e},
	}

	for i, x := range foreignersSplitByInsertion {
		err = a.AddRepositories(ctx, tx, i != 0, x...)
		if err != nil {
			t.Fatal(err)
		}

		first := x[0]
		second := x[1]

		if a.ID != first.OrganizationID {
			t.Error("foreign key was wrong value", a.ID, first.OrganizationID)
		}
		if a.ID != second.OrganizationID {
			t.Error("foreign key was wrong value", a.ID, second.OrganizationID)
		}

		if first.R.Organization != &a {
			t.Error("relationship was not added properly to the foreign slice")
		}
		if second.R.Organization != &a {
			t.Error("relationship was not added properly to the foreign slice")
		}

		if a.R.Repositories[i*2] != first {
			t.Error("relationship struct slice not set to correct value")
		}
		if a.R.Repositories[i*2+1] != second {
			t.Error("relationship struct slice not set to correct value")
		}

		count, err := a.Repositories().Count(ctx, tx)
		if err != nil {
			t.Fatal(err)
		}
		if want := int64((i + 1) * 2); count != want {
			t.Error("want", want, "got", count)
		}
	}
}
func testOrganizationToManyAddOpTopics(t *testing.T) {
	var err error

	ctx := context.Background()
	tx := MustTx(boil.BeginTx(ctx, nil))
	defer func() { _ = tx.Rollback() }()

	var a Organization
	var b, c, d, e Topic

	seed := randomize.NewSeed()
	if err = randomize.Struct(seed, &a, organizationDBTypes, false, strmangle.SetComplement(organizationPrimaryKeyColumns, organizationColumnsWithoutDefault)...); err != nil {
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
		err = a.AddTopics(ctx, tx, i != 0, x...)
		if err != nil {
			t.Fatal(err)
		}

		first := x[0]
		second := x[1]

		if a.ID != first.OrganizationID {
			t.Error("foreign key was wrong value", a.ID, first.OrganizationID)
		}
		if a.ID != second.OrganizationID {
			t.Error("foreign key was wrong value", a.ID, second.OrganizationID)
		}

		if first.R.Organization != &a {
			t.Error("relationship was not added properly to the foreign slice")
		}
		if second.R.Organization != &a {
			t.Error("relationship was not added properly to the foreign slice")
		}

		if a.R.Topics[i*2] != first {
			t.Error("relationship struct slice not set to correct value")
		}
		if a.R.Topics[i*2+1] != second {
			t.Error("relationship struct slice not set to correct value")
		}

		count, err := a.Topics().Count(ctx, tx)
		if err != nil {
			t.Fatal(err)
		}
		if want := int64((i + 1) * 2); count != want {
			t.Error("want", want, "got", count)
		}
	}
}
func testOrganizationToManyAddOpUserLinks(t *testing.T) {
	var err error

	ctx := context.Background()
	tx := MustTx(boil.BeginTx(ctx, nil))
	defer func() { _ = tx.Rollback() }()

	var a Organization
	var b, c, d, e UserLink

	seed := randomize.NewSeed()
	if err = randomize.Struct(seed, &a, organizationDBTypes, false, strmangle.SetComplement(organizationPrimaryKeyColumns, organizationColumnsWithoutDefault)...); err != nil {
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

		if a.ID != first.OrganizationID {
			t.Error("foreign key was wrong value", a.ID, first.OrganizationID)
		}
		if a.ID != second.OrganizationID {
			t.Error("foreign key was wrong value", a.ID, second.OrganizationID)
		}

		if first.R.Organization != &a {
			t.Error("relationship was not added properly to the foreign slice")
		}
		if second.R.Organization != &a {
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

func testOrganizationsReload(t *testing.T) {
	t.Parallel()

	seed := randomize.NewSeed()
	var err error
	o := &Organization{}
	if err = randomize.Struct(seed, o, organizationDBTypes, true, organizationColumnsWithDefault...); err != nil {
		t.Errorf("Unable to randomize Organization struct: %s", err)
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

func testOrganizationsReloadAll(t *testing.T) {
	t.Parallel()

	seed := randomize.NewSeed()
	var err error
	o := &Organization{}
	if err = randomize.Struct(seed, o, organizationDBTypes, true, organizationColumnsWithDefault...); err != nil {
		t.Errorf("Unable to randomize Organization struct: %s", err)
	}

	ctx := context.Background()
	tx := MustTx(boil.BeginTx(ctx, nil))
	defer func() { _ = tx.Rollback() }()
	if err = o.Insert(ctx, tx, boil.Infer()); err != nil {
		t.Error(err)
	}

	slice := OrganizationSlice{o}

	if err = slice.ReloadAll(ctx, tx); err != nil {
		t.Error(err)
	}
}

func testOrganizationsSelect(t *testing.T) {
	t.Parallel()

	seed := randomize.NewSeed()
	var err error
	o := &Organization{}
	if err = randomize.Struct(seed, o, organizationDBTypes, true, organizationColumnsWithDefault...); err != nil {
		t.Errorf("Unable to randomize Organization struct: %s", err)
	}

	ctx := context.Background()
	tx := MustTx(boil.BeginTx(ctx, nil))
	defer func() { _ = tx.Rollback() }()
	if err = o.Insert(ctx, tx, boil.Infer()); err != nil {
		t.Error(err)
	}

	slice, err := Organizations().All(ctx, tx)
	if err != nil {
		t.Error(err)
	}

	if len(slice) != 1 {
		t.Error("want one record, got:", len(slice))
	}
}

var (
	organizationDBTypes = map[string]string{`ID`: `uuid`, `Name`: `character varying`, `CreatedAt`: `timestamp with time zone`, `UpdatedAt`: `timestamp with time zone`, `Login`: `character varying`, `Description`: `character varying`, `Public`: `boolean`, `System`: `boolean`}
	_                   = bytes.MinRead
)

func testOrganizationsUpdate(t *testing.T) {
	t.Parallel()

	if 0 == len(organizationPrimaryKeyColumns) {
		t.Skip("Skipping table with no primary key columns")
	}
	if len(organizationAllColumns) == len(organizationPrimaryKeyColumns) {
		t.Skip("Skipping table with only primary key columns")
	}

	seed := randomize.NewSeed()
	var err error
	o := &Organization{}
	if err = randomize.Struct(seed, o, organizationDBTypes, true, organizationColumnsWithDefault...); err != nil {
		t.Errorf("Unable to randomize Organization struct: %s", err)
	}

	ctx := context.Background()
	tx := MustTx(boil.BeginTx(ctx, nil))
	defer func() { _ = tx.Rollback() }()
	if err = o.Insert(ctx, tx, boil.Infer()); err != nil {
		t.Error(err)
	}

	count, err := Organizations().Count(ctx, tx)
	if err != nil {
		t.Error(err)
	}

	if count != 1 {
		t.Error("want one record, got:", count)
	}

	if err = randomize.Struct(seed, o, organizationDBTypes, true, organizationPrimaryKeyColumns...); err != nil {
		t.Errorf("Unable to randomize Organization struct: %s", err)
	}

	if rowsAff, err := o.Update(ctx, tx, boil.Infer()); err != nil {
		t.Error(err)
	} else if rowsAff != 1 {
		t.Error("should only affect one row but affected", rowsAff)
	}
}

func testOrganizationsSliceUpdateAll(t *testing.T) {
	t.Parallel()

	if len(organizationAllColumns) == len(organizationPrimaryKeyColumns) {
		t.Skip("Skipping table with only primary key columns")
	}

	seed := randomize.NewSeed()
	var err error
	o := &Organization{}
	if err = randomize.Struct(seed, o, organizationDBTypes, true, organizationColumnsWithDefault...); err != nil {
		t.Errorf("Unable to randomize Organization struct: %s", err)
	}

	ctx := context.Background()
	tx := MustTx(boil.BeginTx(ctx, nil))
	defer func() { _ = tx.Rollback() }()
	if err = o.Insert(ctx, tx, boil.Infer()); err != nil {
		t.Error(err)
	}

	count, err := Organizations().Count(ctx, tx)
	if err != nil {
		t.Error(err)
	}

	if count != 1 {
		t.Error("want one record, got:", count)
	}

	if err = randomize.Struct(seed, o, organizationDBTypes, true, organizationPrimaryKeyColumns...); err != nil {
		t.Errorf("Unable to randomize Organization struct: %s", err)
	}

	// Remove Primary keys and unique columns from what we plan to update
	var fields []string
	if strmangle.StringSliceMatch(organizationAllColumns, organizationPrimaryKeyColumns) {
		fields = organizationAllColumns
	} else {
		fields = strmangle.SetComplement(
			organizationAllColumns,
			organizationPrimaryKeyColumns,
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

	slice := OrganizationSlice{o}
	if rowsAff, err := slice.UpdateAll(ctx, tx, updateMap); err != nil {
		t.Error(err)
	} else if rowsAff != 1 {
		t.Error("wanted one record updated but got", rowsAff)
	}
}

func testOrganizationsUpsert(t *testing.T) {
	t.Parallel()

	if len(organizationAllColumns) == len(organizationPrimaryKeyColumns) {
		t.Skip("Skipping table with only primary key columns")
	}

	seed := randomize.NewSeed()
	var err error
	// Attempt the INSERT side of an UPSERT
	o := Organization{}
	if err = randomize.Struct(seed, &o, organizationDBTypes, true); err != nil {
		t.Errorf("Unable to randomize Organization struct: %s", err)
	}

	ctx := context.Background()
	tx := MustTx(boil.BeginTx(ctx, nil))
	defer func() { _ = tx.Rollback() }()
	if err = o.Upsert(ctx, tx, false, nil, boil.Infer(), boil.Infer()); err != nil {
		t.Errorf("Unable to upsert Organization: %s", err)
	}

	count, err := Organizations().Count(ctx, tx)
	if err != nil {
		t.Error(err)
	}
	if count != 1 {
		t.Error("want one record, got:", count)
	}

	// Attempt the UPDATE side of an UPSERT
	if err = randomize.Struct(seed, &o, organizationDBTypes, false, organizationPrimaryKeyColumns...); err != nil {
		t.Errorf("Unable to randomize Organization struct: %s", err)
	}

	if err = o.Upsert(ctx, tx, true, nil, boil.Infer(), boil.Infer()); err != nil {
		t.Errorf("Unable to upsert Organization: %s", err)
	}

	count, err = Organizations().Count(ctx, tx)
	if err != nil {
		t.Error(err)
	}
	if count != 1 {
		t.Error("want one record, got:", count)
	}
}
