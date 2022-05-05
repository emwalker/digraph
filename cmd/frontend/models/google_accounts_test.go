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

func testGoogleAccounts(t *testing.T) {
	t.Parallel()

	query := GoogleAccounts()

	if query.Query == nil {
		t.Error("expected a query, got nothing")
	}
}

func testGoogleAccountsDelete(t *testing.T) {
	t.Parallel()

	seed := randomize.NewSeed()
	var err error
	o := &GoogleAccount{}
	if err = randomize.Struct(seed, o, googleAccountDBTypes, true, googleAccountColumnsWithDefault...); err != nil {
		t.Errorf("Unable to randomize GoogleAccount struct: %s", err)
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

	count, err := GoogleAccounts().Count(ctx, tx)
	if err != nil {
		t.Error(err)
	}

	if count != 0 {
		t.Error("want zero records, got:", count)
	}
}

func testGoogleAccountsQueryDeleteAll(t *testing.T) {
	t.Parallel()

	seed := randomize.NewSeed()
	var err error
	o := &GoogleAccount{}
	if err = randomize.Struct(seed, o, googleAccountDBTypes, true, googleAccountColumnsWithDefault...); err != nil {
		t.Errorf("Unable to randomize GoogleAccount struct: %s", err)
	}

	ctx := context.Background()
	tx := MustTx(boil.BeginTx(ctx, nil))
	defer func() { _ = tx.Rollback() }()
	if err = o.Insert(ctx, tx, boil.Infer()); err != nil {
		t.Error(err)
	}

	if rowsAff, err := GoogleAccounts().DeleteAll(ctx, tx); err != nil {
		t.Error(err)
	} else if rowsAff != 1 {
		t.Error("should only have deleted one row, but affected:", rowsAff)
	}

	count, err := GoogleAccounts().Count(ctx, tx)
	if err != nil {
		t.Error(err)
	}

	if count != 0 {
		t.Error("want zero records, got:", count)
	}
}

func testGoogleAccountsSliceDeleteAll(t *testing.T) {
	t.Parallel()

	seed := randomize.NewSeed()
	var err error
	o := &GoogleAccount{}
	if err = randomize.Struct(seed, o, googleAccountDBTypes, true, googleAccountColumnsWithDefault...); err != nil {
		t.Errorf("Unable to randomize GoogleAccount struct: %s", err)
	}

	ctx := context.Background()
	tx := MustTx(boil.BeginTx(ctx, nil))
	defer func() { _ = tx.Rollback() }()
	if err = o.Insert(ctx, tx, boil.Infer()); err != nil {
		t.Error(err)
	}

	slice := GoogleAccountSlice{o}

	if rowsAff, err := slice.DeleteAll(ctx, tx); err != nil {
		t.Error(err)
	} else if rowsAff != 1 {
		t.Error("should only have deleted one row, but affected:", rowsAff)
	}

	count, err := GoogleAccounts().Count(ctx, tx)
	if err != nil {
		t.Error(err)
	}

	if count != 0 {
		t.Error("want zero records, got:", count)
	}
}

func testGoogleAccountsExists(t *testing.T) {
	t.Parallel()

	seed := randomize.NewSeed()
	var err error
	o := &GoogleAccount{}
	if err = randomize.Struct(seed, o, googleAccountDBTypes, true, googleAccountColumnsWithDefault...); err != nil {
		t.Errorf("Unable to randomize GoogleAccount struct: %s", err)
	}

	ctx := context.Background()
	tx := MustTx(boil.BeginTx(ctx, nil))
	defer func() { _ = tx.Rollback() }()
	if err = o.Insert(ctx, tx, boil.Infer()); err != nil {
		t.Error(err)
	}

	e, err := GoogleAccountExists(ctx, tx, o.ID)
	if err != nil {
		t.Errorf("Unable to check if GoogleAccount exists: %s", err)
	}
	if !e {
		t.Errorf("Expected GoogleAccountExists to return true, but got false.")
	}
}

func testGoogleAccountsFind(t *testing.T) {
	t.Parallel()

	seed := randomize.NewSeed()
	var err error
	o := &GoogleAccount{}
	if err = randomize.Struct(seed, o, googleAccountDBTypes, true, googleAccountColumnsWithDefault...); err != nil {
		t.Errorf("Unable to randomize GoogleAccount struct: %s", err)
	}

	ctx := context.Background()
	tx := MustTx(boil.BeginTx(ctx, nil))
	defer func() { _ = tx.Rollback() }()
	if err = o.Insert(ctx, tx, boil.Infer()); err != nil {
		t.Error(err)
	}

	googleAccountFound, err := FindGoogleAccount(ctx, tx, o.ID)
	if err != nil {
		t.Error(err)
	}

	if googleAccountFound == nil {
		t.Error("want a record, got nil")
	}
}

func testGoogleAccountsBind(t *testing.T) {
	t.Parallel()

	seed := randomize.NewSeed()
	var err error
	o := &GoogleAccount{}
	if err = randomize.Struct(seed, o, googleAccountDBTypes, true, googleAccountColumnsWithDefault...); err != nil {
		t.Errorf("Unable to randomize GoogleAccount struct: %s", err)
	}

	ctx := context.Background()
	tx := MustTx(boil.BeginTx(ctx, nil))
	defer func() { _ = tx.Rollback() }()
	if err = o.Insert(ctx, tx, boil.Infer()); err != nil {
		t.Error(err)
	}

	if err = GoogleAccounts().Bind(ctx, tx, o); err != nil {
		t.Error(err)
	}
}

func testGoogleAccountsOne(t *testing.T) {
	t.Parallel()

	seed := randomize.NewSeed()
	var err error
	o := &GoogleAccount{}
	if err = randomize.Struct(seed, o, googleAccountDBTypes, true, googleAccountColumnsWithDefault...); err != nil {
		t.Errorf("Unable to randomize GoogleAccount struct: %s", err)
	}

	ctx := context.Background()
	tx := MustTx(boil.BeginTx(ctx, nil))
	defer func() { _ = tx.Rollback() }()
	if err = o.Insert(ctx, tx, boil.Infer()); err != nil {
		t.Error(err)
	}

	if x, err := GoogleAccounts().One(ctx, tx); err != nil {
		t.Error(err)
	} else if x == nil {
		t.Error("expected to get a non nil record")
	}
}

func testGoogleAccountsAll(t *testing.T) {
	t.Parallel()

	seed := randomize.NewSeed()
	var err error
	googleAccountOne := &GoogleAccount{}
	googleAccountTwo := &GoogleAccount{}
	if err = randomize.Struct(seed, googleAccountOne, googleAccountDBTypes, false, googleAccountColumnsWithDefault...); err != nil {
		t.Errorf("Unable to randomize GoogleAccount struct: %s", err)
	}
	if err = randomize.Struct(seed, googleAccountTwo, googleAccountDBTypes, false, googleAccountColumnsWithDefault...); err != nil {
		t.Errorf("Unable to randomize GoogleAccount struct: %s", err)
	}

	ctx := context.Background()
	tx := MustTx(boil.BeginTx(ctx, nil))
	defer func() { _ = tx.Rollback() }()
	if err = googleAccountOne.Insert(ctx, tx, boil.Infer()); err != nil {
		t.Error(err)
	}
	if err = googleAccountTwo.Insert(ctx, tx, boil.Infer()); err != nil {
		t.Error(err)
	}

	slice, err := GoogleAccounts().All(ctx, tx)
	if err != nil {
		t.Error(err)
	}

	if len(slice) != 2 {
		t.Error("want 2 records, got:", len(slice))
	}
}

func testGoogleAccountsCount(t *testing.T) {
	t.Parallel()

	var err error
	seed := randomize.NewSeed()
	googleAccountOne := &GoogleAccount{}
	googleAccountTwo := &GoogleAccount{}
	if err = randomize.Struct(seed, googleAccountOne, googleAccountDBTypes, false, googleAccountColumnsWithDefault...); err != nil {
		t.Errorf("Unable to randomize GoogleAccount struct: %s", err)
	}
	if err = randomize.Struct(seed, googleAccountTwo, googleAccountDBTypes, false, googleAccountColumnsWithDefault...); err != nil {
		t.Errorf("Unable to randomize GoogleAccount struct: %s", err)
	}

	ctx := context.Background()
	tx := MustTx(boil.BeginTx(ctx, nil))
	defer func() { _ = tx.Rollback() }()
	if err = googleAccountOne.Insert(ctx, tx, boil.Infer()); err != nil {
		t.Error(err)
	}
	if err = googleAccountTwo.Insert(ctx, tx, boil.Infer()); err != nil {
		t.Error(err)
	}

	count, err := GoogleAccounts().Count(ctx, tx)
	if err != nil {
		t.Error(err)
	}

	if count != 2 {
		t.Error("want 2 records, got:", count)
	}
}

func testGoogleAccountsInsert(t *testing.T) {
	t.Parallel()

	seed := randomize.NewSeed()
	var err error
	o := &GoogleAccount{}
	if err = randomize.Struct(seed, o, googleAccountDBTypes, true, googleAccountColumnsWithDefault...); err != nil {
		t.Errorf("Unable to randomize GoogleAccount struct: %s", err)
	}

	ctx := context.Background()
	tx := MustTx(boil.BeginTx(ctx, nil))
	defer func() { _ = tx.Rollback() }()
	if err = o.Insert(ctx, tx, boil.Infer()); err != nil {
		t.Error(err)
	}

	count, err := GoogleAccounts().Count(ctx, tx)
	if err != nil {
		t.Error(err)
	}

	if count != 1 {
		t.Error("want one record, got:", count)
	}
}

func testGoogleAccountsInsertWhitelist(t *testing.T) {
	t.Parallel()

	seed := randomize.NewSeed()
	var err error
	o := &GoogleAccount{}
	if err = randomize.Struct(seed, o, googleAccountDBTypes, true); err != nil {
		t.Errorf("Unable to randomize GoogleAccount struct: %s", err)
	}

	ctx := context.Background()
	tx := MustTx(boil.BeginTx(ctx, nil))
	defer func() { _ = tx.Rollback() }()
	if err = o.Insert(ctx, tx, boil.Whitelist(googleAccountColumnsWithoutDefault...)); err != nil {
		t.Error(err)
	}

	count, err := GoogleAccounts().Count(ctx, tx)
	if err != nil {
		t.Error(err)
	}

	if count != 1 {
		t.Error("want one record, got:", count)
	}
}

func testGoogleAccountToOneUserUsingUser(t *testing.T) {
	ctx := context.Background()
	tx := MustTx(boil.BeginTx(ctx, nil))
	defer func() { _ = tx.Rollback() }()

	var local GoogleAccount
	var foreign User

	seed := randomize.NewSeed()
	if err := randomize.Struct(seed, &local, googleAccountDBTypes, false, googleAccountColumnsWithDefault...); err != nil {
		t.Errorf("Unable to randomize GoogleAccount struct: %s", err)
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

	slice := GoogleAccountSlice{&local}
	if err = local.L.LoadUser(ctx, tx, false, (*[]*GoogleAccount)(&slice), nil); err != nil {
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

func testGoogleAccountToOneSetOpUserUsingUser(t *testing.T) {
	var err error

	ctx := context.Background()
	tx := MustTx(boil.BeginTx(ctx, nil))
	defer func() { _ = tx.Rollback() }()

	var a GoogleAccount
	var b, c User

	seed := randomize.NewSeed()
	if err = randomize.Struct(seed, &a, googleAccountDBTypes, false, strmangle.SetComplement(googleAccountPrimaryKeyColumns, googleAccountColumnsWithoutDefault)...); err != nil {
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

		if x.R.GoogleAccounts[0] != &a {
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

func testGoogleAccountsReload(t *testing.T) {
	t.Parallel()

	seed := randomize.NewSeed()
	var err error
	o := &GoogleAccount{}
	if err = randomize.Struct(seed, o, googleAccountDBTypes, true, googleAccountColumnsWithDefault...); err != nil {
		t.Errorf("Unable to randomize GoogleAccount struct: %s", err)
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

func testGoogleAccountsReloadAll(t *testing.T) {
	t.Parallel()

	seed := randomize.NewSeed()
	var err error
	o := &GoogleAccount{}
	if err = randomize.Struct(seed, o, googleAccountDBTypes, true, googleAccountColumnsWithDefault...); err != nil {
		t.Errorf("Unable to randomize GoogleAccount struct: %s", err)
	}

	ctx := context.Background()
	tx := MustTx(boil.BeginTx(ctx, nil))
	defer func() { _ = tx.Rollback() }()
	if err = o.Insert(ctx, tx, boil.Infer()); err != nil {
		t.Error(err)
	}

	slice := GoogleAccountSlice{o}

	if err = slice.ReloadAll(ctx, tx); err != nil {
		t.Error(err)
	}
}

func testGoogleAccountsSelect(t *testing.T) {
	t.Parallel()

	seed := randomize.NewSeed()
	var err error
	o := &GoogleAccount{}
	if err = randomize.Struct(seed, o, googleAccountDBTypes, true, googleAccountColumnsWithDefault...); err != nil {
		t.Errorf("Unable to randomize GoogleAccount struct: %s", err)
	}

	ctx := context.Background()
	tx := MustTx(boil.BeginTx(ctx, nil))
	defer func() { _ = tx.Rollback() }()
	if err = o.Insert(ctx, tx, boil.Infer()); err != nil {
		t.Error(err)
	}

	slice, err := GoogleAccounts().All(ctx, tx)
	if err != nil {
		t.Error(err)
	}

	if len(slice) != 1 {
		t.Error("want one record, got:", len(slice))
	}
}

var (
	googleAccountDBTypes = map[string]string{`ID`: `uuid`, `UserID`: `uuid`, `ProfileID`: `character varying`, `Name`: `character varying`, `PrimaryEmail`: `USER-DEFINED`, `AvatarURL`: `character varying`}
	_                    = bytes.MinRead
)

func testGoogleAccountsUpdate(t *testing.T) {
	t.Parallel()

	if 0 == len(googleAccountPrimaryKeyColumns) {
		t.Skip("Skipping table with no primary key columns")
	}
	if len(googleAccountAllColumns) == len(googleAccountPrimaryKeyColumns) {
		t.Skip("Skipping table with only primary key columns")
	}

	seed := randomize.NewSeed()
	var err error
	o := &GoogleAccount{}
	if err = randomize.Struct(seed, o, googleAccountDBTypes, true, googleAccountColumnsWithDefault...); err != nil {
		t.Errorf("Unable to randomize GoogleAccount struct: %s", err)
	}

	ctx := context.Background()
	tx := MustTx(boil.BeginTx(ctx, nil))
	defer func() { _ = tx.Rollback() }()
	if err = o.Insert(ctx, tx, boil.Infer()); err != nil {
		t.Error(err)
	}

	count, err := GoogleAccounts().Count(ctx, tx)
	if err != nil {
		t.Error(err)
	}

	if count != 1 {
		t.Error("want one record, got:", count)
	}

	if err = randomize.Struct(seed, o, googleAccountDBTypes, true, googleAccountPrimaryKeyColumns...); err != nil {
		t.Errorf("Unable to randomize GoogleAccount struct: %s", err)
	}

	if rowsAff, err := o.Update(ctx, tx, boil.Infer()); err != nil {
		t.Error(err)
	} else if rowsAff != 1 {
		t.Error("should only affect one row but affected", rowsAff)
	}
}

func testGoogleAccountsSliceUpdateAll(t *testing.T) {
	t.Parallel()

	if len(googleAccountAllColumns) == len(googleAccountPrimaryKeyColumns) {
		t.Skip("Skipping table with only primary key columns")
	}

	seed := randomize.NewSeed()
	var err error
	o := &GoogleAccount{}
	if err = randomize.Struct(seed, o, googleAccountDBTypes, true, googleAccountColumnsWithDefault...); err != nil {
		t.Errorf("Unable to randomize GoogleAccount struct: %s", err)
	}

	ctx := context.Background()
	tx := MustTx(boil.BeginTx(ctx, nil))
	defer func() { _ = tx.Rollback() }()
	if err = o.Insert(ctx, tx, boil.Infer()); err != nil {
		t.Error(err)
	}

	count, err := GoogleAccounts().Count(ctx, tx)
	if err != nil {
		t.Error(err)
	}

	if count != 1 {
		t.Error("want one record, got:", count)
	}

	if err = randomize.Struct(seed, o, googleAccountDBTypes, true, googleAccountPrimaryKeyColumns...); err != nil {
		t.Errorf("Unable to randomize GoogleAccount struct: %s", err)
	}

	// Remove Primary keys and unique columns from what we plan to update
	var fields []string
	if strmangle.StringSliceMatch(googleAccountAllColumns, googleAccountPrimaryKeyColumns) {
		fields = googleAccountAllColumns
	} else {
		fields = strmangle.SetComplement(
			googleAccountAllColumns,
			googleAccountPrimaryKeyColumns,
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

	slice := GoogleAccountSlice{o}
	if rowsAff, err := slice.UpdateAll(ctx, tx, updateMap); err != nil {
		t.Error(err)
	} else if rowsAff != 1 {
		t.Error("wanted one record updated but got", rowsAff)
	}
}

func testGoogleAccountsUpsert(t *testing.T) {
	t.Parallel()

	if len(googleAccountAllColumns) == len(googleAccountPrimaryKeyColumns) {
		t.Skip("Skipping table with only primary key columns")
	}

	seed := randomize.NewSeed()
	var err error
	// Attempt the INSERT side of an UPSERT
	o := GoogleAccount{}
	if err = randomize.Struct(seed, &o, googleAccountDBTypes, true); err != nil {
		t.Errorf("Unable to randomize GoogleAccount struct: %s", err)
	}

	ctx := context.Background()
	tx := MustTx(boil.BeginTx(ctx, nil))
	defer func() { _ = tx.Rollback() }()
	if err = o.Upsert(ctx, tx, false, nil, boil.Infer(), boil.Infer()); err != nil {
		t.Errorf("Unable to upsert GoogleAccount: %s", err)
	}

	count, err := GoogleAccounts().Count(ctx, tx)
	if err != nil {
		t.Error(err)
	}
	if count != 1 {
		t.Error("want one record, got:", count)
	}

	// Attempt the UPDATE side of an UPSERT
	if err = randomize.Struct(seed, &o, googleAccountDBTypes, false, googleAccountPrimaryKeyColumns...); err != nil {
		t.Errorf("Unable to randomize GoogleAccount struct: %s", err)
	}

	if err = o.Upsert(ctx, tx, true, nil, boil.Infer(), boil.Infer()); err != nil {
		t.Errorf("Unable to upsert GoogleAccount: %s", err)
	}

	count, err = GoogleAccounts().Count(ctx, tx)
	if err != nil {
		t.Error(err)
	}
	if count != 1 {
		t.Error("want one record, got:", count)
	}
}
