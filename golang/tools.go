//go:build tools
// +build tools

package tools

import (
	_ "github.com/99designs/gqlgen"
	_ "github.com/99designs/gqlgen/codegen/config"
	_ "github.com/99designs/gqlgen/internal/imports"
	_ "github.com/vektah/dataloaden"
)
