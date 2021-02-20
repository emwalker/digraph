// Code generated by SQLBoiler 3.7.1 (https://github.com/volatiletech/sqlboiler). DO NOT EDIT.
// This file is meant to be re-generated in place and/or deleted at any time.

package models

import (
	"bytes"
	"context"
	"reflect"
	"testing"

	"github.com/volatiletech/sqlboiler/boil"
	"github.com/volatiletech/sqlboiler/queries"
	"github.com/volatiletech/sqlboiler/randomize"
	"github.com/volatiletech/sqlboiler/strmangle"
)

var (
	// Relationships sometimes use the reflection helper queries.Equal/queries.Assign
	// so force a package dependency in case they don't.
	_ = queries.Equal
)

func testGithubAccounts(t *testing.T) {
	t.Parallel()

	query := GithubAccounts()

	if query.Query == nil {
		t.Error("expected a query, got nothing")
	}
}

func testGithubAccountsDelete(t *testing.T) {
	t.Parallel()

	seed := randomize.NewSeed()
	var err error
	o := &GithubAccount{}
	if err = randomize.Struct(seed, o, githubAccountDBTypes, true, githubAccountColumnsWithDefault...); err != nil {
		t.Errorf("Unable to randomize GithubAccount struct: %s", err)
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

	count, err := GithubAccounts().Count(ctx, tx)
	if err != nil {
		t.Error(err)
	}

	if count != 0 {
		t.Error("want zero records, got:", count)
	}
}

func testGithubAccountsQueryDeleteAll(t *testing.T) {
	t.Parallel()

	seed := randomize.NewSeed()
	var err error
	o := &GithubAccount{}
	if err = randomize.Struct(seed, o, githubAccountDBTypes, true, githubAccountColumnsWithDefault...); err != nil {
		t.Errorf("Unable to randomize GithubAccount struct: %s", err)
	}

	ctx := context.Background()
	tx := MustTx(boil.BeginTx(ctx, nil))
	defer func() { _ = tx.Rollback() }()
	if err = o.Insert(ctx, tx, boil.Infer()); err != nil {
		t.Error(err)
	}

	if rowsAff, err := GithubAccounts().DeleteAll(ctx, tx); err != nil {
		t.Error(err)
	} else if rowsAff != 1 {
		t.Error("should only have deleted one row, but affected:", rowsAff)
	}

	count, err := GithubAccounts().Count(ctx, tx)
	if err != nil {
		t.Error(err)
	}

	if count != 0 {
		t.Error("want zero records, got:", count)
	}
}

func testGithubAccountsSliceDeleteAll(t *testing.T) {
	t.Parallel()

	seed := randomize.NewSeed()
	var err error
	o := &GithubAccount{}
	if err = randomize.Struct(seed, o, githubAccountDBTypes, true, githubAccountColumnsWithDefault...); err != nil {
		t.Errorf("Unable to randomize GithubAccount struct: %s", err)
	}

	ctx := context.Background()
	tx := MustTx(boil.BeginTx(ctx, nil))
	defer func() { _ = tx.Rollback() }()
	if err = o.Insert(ctx, tx, boil.Infer()); err != nil {
		t.Error(err)
	}

	slice := GithubAccountSlice{o}

	if rowsAff, err := slice.DeleteAll(ctx, tx); err != nil {
		t.Error(err)
	} else if rowsAff != 1 {
		t.Error("should only have deleted one row, but affected:", rowsAff)
	}

	count, err := GithubAccounts().Count(ctx, tx)
	if err != nil {
		t.Error(err)
	}

	if count != 0 {
		t.Error("want zero records, got:", count)
	}
}

func testGithubAccountsExists(t *testing.T) {
	t.Parallel()

	seed := randomize.NewSeed()
	var err error
	o := &GithubAccount{}
	if err = randomize.Struct(seed, o, githubAccountDBTypes, true, githubAccountColumnsWithDefault...); err != nil {
		t.Errorf("Unable to randomize GithubAccount struct: %s", err)
	}

	ctx := context.Background()
	tx := MustTx(boil.BeginTx(ctx, nil))
	defer func() { _ = tx.Rollback() }()
	if err = o.Insert(ctx, tx, boil.Infer()); err != nil {
		t.Error(err)
	}

	e, err := GithubAccountExists(ctx, tx, o.ID)
	if err != nil {
		t.Errorf("Unable to check if GithubAccount exists: %s", err)
	}
	if !e {
		t.Errorf("Expected GithubAccountExists to return true, but got false.")
	}
}

func testGithubAccountsFind(t *testing.T) {
	t.Parallel()

	seed := randomize.NewSeed()
	var err error
	o := &GithubAccount{}
	if err = randomize.Struct(seed, o, githubAccountDBTypes, true, githubAccountColumnsWithDefault...); err != nil {
		t.Errorf("Unable to randomize GithubAccount struct: %s", err)
	}

	ctx := context.Background()
	tx := MustTx(boil.BeginTx(ctx, nil))
	defer func() { _ = tx.Rollback() }()
	if err = o.Insert(ctx, tx, boil.Infer()); err != nil {
		t.Error(err)
	}

	githubAccountFound, err := FindGithubAccount(ctx, tx, o.ID)
	if err != nil {
		t.Error(err)
	}

	if githubAccountFound == nil {
		t.Error("want a record, got nil")
	}
}

func testGithubAccountsBind(t *testing.T) {
	t.Parallel()

	seed := randomize.NewSeed()
	var err error
	o := &GithubAccount{}
	if err = randomize.Struct(seed, o, githubAccountDBTypes, true, githubAccountColumnsWithDefault...); err != nil {
		t.Errorf("Unable to randomize GithubAccount struct: %s", err)
	}

	ctx := context.Background()
	tx := MustTx(boil.BeginTx(ctx, nil))
	defer func() { _ = tx.Rollback() }()
	if err = o.Insert(ctx, tx, boil.Infer()); err != nil {
		t.Error(err)
	}

	if err = GithubAccounts().Bind(ctx, tx, o); err != nil {
		t.Error(err)
	}
}

func testGithubAccountsOne(t *testing.T) {
	t.Parallel()

	seed := randomize.NewSeed()
	var err error
	o := &GithubAccount{}
	if err = randomize.Struct(seed, o, githubAccountDBTypes, true, githubAccountColumnsWithDefault...); err != nil {
		t.Errorf("Unable to randomize GithubAccount struct: %s", err)
	}

	ctx := context.Background()
	tx := MustTx(boil.BeginTx(ctx, nil))
	defer func() { _ = tx.Rollback() }()
	if err = o.Insert(ctx, tx, boil.Infer()); err != nil {
		t.Error(err)
	}

	if x, err := GithubAccounts().One(ctx, tx); err != nil {
		t.Error(err)
	} else if x == nil {
		t.Error("expected to get a non nil record")
	}
}

func testGithubAccountsAll(t *testing.T) {
	t.Parallel()

	seed := randomize.NewSeed()
	var err error
	githubAccountOne := &GithubAccount{}
	githubAccountTwo := &GithubAccount{}
	if err = randomize.Struct(seed, githubAccountOne, githubAccountDBTypes, false, githubAccountColumnsWithDefault...); err != nil {
		t.Errorf("Unable to randomize GithubAccount struct: %s", err)
	}
	if err = randomize.Struct(seed, githubAccountTwo, githubAccountDBTypes, false, githubAccountColumnsWithDefault...); err != nil {
		t.Errorf("Unable to randomize GithubAccount struct: %s", err)
	}

	ctx := context.Background()
	tx := MustTx(boil.BeginTx(ctx, nil))
	defer func() { _ = tx.Rollback() }()
	if err = githubAccountOne.Insert(ctx, tx, boil.Infer()); err != nil {
		t.Error(err)
	}
	if err = githubAccountTwo.Insert(ctx, tx, boil.Infer()); err != nil {
		t.Error(err)
	}

	slice, err := GithubAccounts().All(ctx, tx)
	if err != nil {
		t.Error(err)
	}

	if len(slice) != 2 {
		t.Error("want 2 records, got:", len(slice))
	}
}

func testGithubAccountsCount(t *testing.T) {
	t.Parallel()

	var err error
	seed := randomize.NewSeed()
	githubAccountOne := &GithubAccount{}
	githubAccountTwo := &GithubAccount{}
	if err = randomize.Struct(seed, githubAccountOne, githubAccountDBTypes, false, githubAccountColumnsWithDefault...); err != nil {
		t.Errorf("Unable to randomize GithubAccount struct: %s", err)
	}
	if err = randomize.Struct(seed, githubAccountTwo, githubAccountDBTypes, false, githubAccountColumnsWithDefault...); err != nil {
		t.Errorf("Unable to randomize GithubAccount struct: %s", err)
	}

	ctx := context.Background()
	tx := MustTx(boil.BeginTx(ctx, nil))
	defer func() { _ = tx.Rollback() }()
	if err = githubAccountOne.Insert(ctx, tx, boil.Infer()); err != nil {
		t.Error(err)
	}
	if err = githubAccountTwo.Insert(ctx, tx, boil.Infer()); err != nil {
		t.Error(err)
	}

	count, err := GithubAccounts().Count(ctx, tx)
	if err != nil {
		t.Error(err)
	}

	if count != 2 {
		t.Error("want 2 records, got:", count)
	}
}

func testGithubAccountsInsert(t *testing.T) {
	t.Parallel()

	seed := randomize.NewSeed()
	var err error
	o := &GithubAccount{}
	if err = randomize.Struct(seed, o, githubAccountDBTypes, true, githubAccountColumnsWithDefault...); err != nil {
		t.Errorf("Unable to randomize GithubAccount struct: %s", err)
	}

	ctx := context.Background()
	tx := MustTx(boil.BeginTx(ctx, nil))
	defer func() { _ = tx.Rollback() }()
	if err = o.Insert(ctx, tx, boil.Infer()); err != nil {
		t.Error(err)
	}

	count, err := GithubAccounts().Count(ctx, tx)
	if err != nil {
		t.Error(err)
	}

	if count != 1 {
		t.Error("want one record, got:", count)
	}
}

func testGithubAccountsInsertWhitelist(t *testing.T) {
	t.Parallel()

	seed := randomize.NewSeed()
	var err error
	o := &GithubAccount{}
	if err = randomize.Struct(seed, o, githubAccountDBTypes, true); err != nil {
		t.Errorf("Unable to randomize GithubAccount struct: %s", err)
	}

	ctx := context.Background()
	tx := MustTx(boil.BeginTx(ctx, nil))
	defer func() { _ = tx.Rollback() }()
	if err = o.Insert(ctx, tx, boil.Whitelist(githubAccountColumnsWithoutDefault...)); err != nil {
		t.Error(err)
	}

	count, err := GithubAccounts().Count(ctx, tx)
	if err != nil {
		t.Error(err)
	}

	if count != 1 {
		t.Error("want one record, got:", count)
	}
}

func testGithubAccountToOneUserUsingUser(t *testing.T) {
	ctx := context.Background()
	tx := MustTx(boil.BeginTx(ctx, nil))
	defer func() { _ = tx.Rollback() }()

	var local GithubAccount
	var foreign User

	seed := randomize.NewSeed()
	if err := randomize.Struct(seed, &local, githubAccountDBTypes, false, githubAccountColumnsWithDefault...); err != nil {
		t.Errorf("Unable to randomize GithubAccount struct: %s", err)
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

	slice := GithubAccountSlice{&local}
	if err = local.L.LoadUser(ctx, tx, false, (*[]*GithubAccount)(&slice), nil); err != nil {
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

func testGithubAccountToOneSetOpUserUsingUser(t *testing.T) {
	var err error

	ctx := context.Background()
	tx := MustTx(boil.BeginTx(ctx, nil))
	defer func() { _ = tx.Rollback() }()

	var a GithubAccount
	var b, c User

	seed := randomize.NewSeed()
	if err = randomize.Struct(seed, &a, githubAccountDBTypes, false, strmangle.SetComplement(githubAccountPrimaryKeyColumns, githubAccountColumnsWithoutDefault)...); err != nil {
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

		if x.R.GithubAccounts[0] != &a {
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

func testGithubAccountsReload(t *testing.T) {
	t.Parallel()

	seed := randomize.NewSeed()
	var err error
	o := &GithubAccount{}
	if err = randomize.Struct(seed, o, githubAccountDBTypes, true, githubAccountColumnsWithDefault...); err != nil {
		t.Errorf("Unable to randomize GithubAccount struct: %s", err)
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

func testGithubAccountsReloadAll(t *testing.T) {
	t.Parallel()

	seed := randomize.NewSeed()
	var err error
	o := &GithubAccount{}
	if err = randomize.Struct(seed, o, githubAccountDBTypes, true, githubAccountColumnsWithDefault...); err != nil {
		t.Errorf("Unable to randomize GithubAccount struct: %s", err)
	}

	ctx := context.Background()
	tx := MustTx(boil.BeginTx(ctx, nil))
	defer func() { _ = tx.Rollback() }()
	if err = o.Insert(ctx, tx, boil.Infer()); err != nil {
		t.Error(err)
	}

	slice := GithubAccountSlice{o}

	if err = slice.ReloadAll(ctx, tx); err != nil {
		t.Error(err)
	}
}

func testGithubAccountsSelect(t *testing.T) {
	t.Parallel()

	seed := randomize.NewSeed()
	var err error
	o := &GithubAccount{}
	if err = randomize.Struct(seed, o, githubAccountDBTypes, true, githubAccountColumnsWithDefault...); err != nil {
		t.Errorf("Unable to randomize GithubAccount struct: %s", err)
	}

	ctx := context.Background()
	tx := MustTx(boil.BeginTx(ctx, nil))
	defer func() { _ = tx.Rollback() }()
	if err = o.Insert(ctx, tx, boil.Infer()); err != nil {
		t.Error(err)
	}

	slice, err := GithubAccounts().All(ctx, tx)
	if err != nil {
		t.Error(err)
	}

	if len(slice) != 1 {
		t.Error("want one record, got:", len(slice))
	}
}

var (
	githubAccountDBTypes = map[string]string{`ID`: `uuid`, `UserID`: `uuid`, `Username`: `character varying`, `Name`: `character varying`, `PrimaryEmail`: `USER-DEFINED`, `AvatarURL`: `character varying`}
	_                    = bytes.MinRead
)

func testGithubAccountsUpdate(t *testing.T) {
	t.Parallel()

	if 0 == len(githubAccountPrimaryKeyColumns) {
		t.Skip("Skipping table with no primary key columns")
	}
	if len(githubAccountAllColumns) == len(githubAccountPrimaryKeyColumns) {
		t.Skip("Skipping table with only primary key columns")
	}

	seed := randomize.NewSeed()
	var err error
	o := &GithubAccount{}
	if err = randomize.Struct(seed, o, githubAccountDBTypes, true, githubAccountColumnsWithDefault...); err != nil {
		t.Errorf("Unable to randomize GithubAccount struct: %s", err)
	}

	ctx := context.Background()
	tx := MustTx(boil.BeginTx(ctx, nil))
	defer func() { _ = tx.Rollback() }()
	if err = o.Insert(ctx, tx, boil.Infer()); err != nil {
		t.Error(err)
	}

	count, err := GithubAccounts().Count(ctx, tx)
	if err != nil {
		t.Error(err)
	}

	if count != 1 {
		t.Error("want one record, got:", count)
	}

	if err = randomize.Struct(seed, o, githubAccountDBTypes, true, githubAccountPrimaryKeyColumns...); err != nil {
		t.Errorf("Unable to randomize GithubAccount struct: %s", err)
	}

	if rowsAff, err := o.Update(ctx, tx, boil.Infer()); err != nil {
		t.Error(err)
	} else if rowsAff != 1 {
		t.Error("should only affect one row but affected", rowsAff)
	}
}

func testGithubAccountsSliceUpdateAll(t *testing.T) {
	t.Parallel()

	if len(githubAccountAllColumns) == len(githubAccountPrimaryKeyColumns) {
		t.Skip("Skipping table with only primary key columns")
	}

	seed := randomize.NewSeed()
	var err error
	o := &GithubAccount{}
	if err = randomize.Struct(seed, o, githubAccountDBTypes, true, githubAccountColumnsWithDefault...); err != nil {
		t.Errorf("Unable to randomize GithubAccount struct: %s", err)
	}

	ctx := context.Background()
	tx := MustTx(boil.BeginTx(ctx, nil))
	defer func() { _ = tx.Rollback() }()
	if err = o.Insert(ctx, tx, boil.Infer()); err != nil {
		t.Error(err)
	}

	count, err := GithubAccounts().Count(ctx, tx)
	if err != nil {
		t.Error(err)
	}

	if count != 1 {
		t.Error("want one record, got:", count)
	}

	if err = randomize.Struct(seed, o, githubAccountDBTypes, true, githubAccountPrimaryKeyColumns...); err != nil {
		t.Errorf("Unable to randomize GithubAccount struct: %s", err)
	}

	// Remove Primary keys and unique columns from what we plan to update
	var fields []string
	if strmangle.StringSliceMatch(githubAccountAllColumns, githubAccountPrimaryKeyColumns) {
		fields = githubAccountAllColumns
	} else {
		fields = strmangle.SetComplement(
			githubAccountAllColumns,
			githubAccountPrimaryKeyColumns,
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

	slice := GithubAccountSlice{o}
	if rowsAff, err := slice.UpdateAll(ctx, tx, updateMap); err != nil {
		t.Error(err)
	} else if rowsAff != 1 {
		t.Error("wanted one record updated but got", rowsAff)
	}
}

func testGithubAccountsUpsert(t *testing.T) {
	t.Parallel()

	if len(githubAccountAllColumns) == len(githubAccountPrimaryKeyColumns) {
		t.Skip("Skipping table with only primary key columns")
	}

	seed := randomize.NewSeed()
	var err error
	// Attempt the INSERT side of an UPSERT
	o := GithubAccount{}
	if err = randomize.Struct(seed, &o, githubAccountDBTypes, true); err != nil {
		t.Errorf("Unable to randomize GithubAccount struct: %s", err)
	}

	ctx := context.Background()
	tx := MustTx(boil.BeginTx(ctx, nil))
	defer func() { _ = tx.Rollback() }()
	if err = o.Upsert(ctx, tx, false, nil, boil.Infer(), boil.Infer()); err != nil {
		t.Errorf("Unable to upsert GithubAccount: %s", err)
	}

	count, err := GithubAccounts().Count(ctx, tx)
	if err != nil {
		t.Error(err)
	}
	if count != 1 {
		t.Error("want one record, got:", count)
	}

	// Attempt the UPDATE side of an UPSERT
	if err = randomize.Struct(seed, &o, githubAccountDBTypes, false, githubAccountPrimaryKeyColumns...); err != nil {
		t.Errorf("Unable to randomize GithubAccount struct: %s", err)
	}

	if err = o.Upsert(ctx, tx, true, nil, boil.Infer(), boil.Infer()); err != nil {
		t.Errorf("Unable to upsert GithubAccount: %s", err)
	}

	count, err = GithubAccounts().Count(ctx, tx)
	if err != nil {
		t.Error(err)
	}
	if count != 1 {
		t.Error("want one record, got:", count)
	}
}
