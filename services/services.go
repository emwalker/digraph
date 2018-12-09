package services

import (
	"github.com/emwalker/digraph/models"
	"github.com/volatiletech/sqlboiler/boil"
)

type Connection struct {
	Exec  boil.ContextExecutor
	Actor *models.User
}
