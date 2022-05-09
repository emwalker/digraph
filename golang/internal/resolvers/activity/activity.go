package activity

import (
	"bytes"
	"fmt"
	"strings"
	"time"

	"github.com/emwalker/digraph/golang/internal/models"
	"github.com/emwalker/digraph/golang/internal/text"
)

// Topic holds essential information about a topic.
type Topic struct {
	Name string
	ID   string
}

// User holds essential information about a user.
type User struct {
	Name string
}

// Link holds essential information about a link.
type Link struct {
	Title string
	URL   string
}

// UpsertLink holds essential information about a log entry in which a link was upserted.
type UpsertLink struct {
	CreatedAt time.Time
	User      User
	Link      Link
	Topics    []Topic
}

// MakeEdges takes a slice of UpsertLink structs and returns a []*models.ActivityLineItemEdge.
func MakeEdges(lineItems []UpsertLink) ([]*models.ActivityLineItemEdge, error) {
	edges := make([]*models.ActivityLineItemEdge, len(lineItems))

	for i, item := range lineItems {
		node := upsertLinkNode(item)
		edges[i] = &models.ActivityLineItemEdge{Node: &node}
	}

	return edges, nil
}

// EscapeTitle escapes a title that will be used in a markdown reference.
func EscapeTitle(str string) string {
	str = strings.Replace(str, "[", "\\[", -1)
	str = strings.Replace(str, "]", "\\]", -1)
	return str
}

// EscapeURL escapes a URL that will be used in a markdown reference.
func EscapeURL(str string) string {
	str = strings.Replace(str, "(", "%%28", -1)
	str = strings.Replace(str, ")", "%%29", -1)
	return str
}

func makeRef(title, url string) string {
	return fmt.Sprintf("[%s](%s)", EscapeTitle(text.Squash(title)), EscapeURL(url))
}

func upsertLinkNode(ul UpsertLink) models.ActivityLineItem {
	var desc bytes.Buffer

	desc.WriteString(fmt.Sprintf("%s added %s", ul.User.Name, makeRef(ul.Link.Title, ul.Link.URL)))

	if len(ul.Topics) > 0 {
		refs := make([]string, len(ul.Topics))

		for i, topic := range ul.Topics {
			refs[i] = makeRef(topic.Name, fmt.Sprintf("/wiki/topics/%s", topic.ID))
		}

		var topicRefs string

		if len(ul.Topics) > 2 {
			idx := len(refs) - 1
			topicRefs = fmt.Sprintf("%s and %s", strings.Join(refs[:idx], ", "), refs[idx])
		} else if len(ul.Topics) > 1 {
			topicRefs = strings.Join(refs, " and ")
		} else {
			topicRefs = refs[0]
		}

		desc.WriteString(fmt.Sprintf(" and tagged it with %s", topicRefs))
	}

	return models.ActivityLineItem{
		Description: desc.String(),
		CreatedAt:   ul.CreatedAt.Format(time.RFC3339),
	}
}
