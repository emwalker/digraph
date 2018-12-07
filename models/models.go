package models

import (
	"github.com/google/uuid"
)

// IsNamespaceable tags Link as implementing the Namespaceable interface.
func (Link) IsNamespaceable() {}

// IsResourceIdentifiable tags Link as implementing the ResourceIdentifiable interface.
func (Link) IsResourceIdentifiable() {}

// IsResourceIdentifiable tags Topic as implementing the ResourceIdentifiable interface.
func (Topic) IsResourceIdentifiable() {}

// IsNamespaceable tags Topic as implementing the Namespaceable interface.
func (Topic) IsNamespaceable() {}

// IsResourceIdentifiable tags Organization as implementing the ResourceIdentifiable interface.
func (Organization) IsResourceIdentifiable() {}

func NewAlert(typ AlertType, text string) *Alert {
	return &Alert{
		ID:   uuid.New().String(),
		Text: text,
		Type: typ,
	}
}
