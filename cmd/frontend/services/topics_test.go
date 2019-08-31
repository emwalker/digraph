package services_test

import (
	"testing"
	"time"

	"github.com/emwalker/digraph/cmd/frontend/models"
	"github.com/emwalker/digraph/cmd/frontend/services"
	"github.com/volatiletech/null"
)

func TestNormalizeName(t *testing.T) {
	testCases := []struct {
		description    string
		inputName      string
		normalizedName string
		isValid        bool
	}{
		{
			description:    "An valid topic",
			inputName:      "   Agricultural   revolution ",
			normalizedName: "Agricultural revolution",
			isValid:        true,
		},
		{
			description:    "An empty topic",
			inputName:      "   ",
			normalizedName: "",
			isValid:        false,
		},
		{
			description:    "A link",
			inputName:      "http://www.google.com/",
			normalizedName: "http://www.google.com/",
			isValid:        false,
		},
		{
			description:    "An FTP url",
			inputName:      "ftp://ftp.google.com/",
			normalizedName: "ftp://ftp.google.com/",
			isValid:        false,
		},
	}

	for _, testCase := range testCases {
		t.Run(testCase.description, func(t *testing.T) {
			normalizedName, ok := services.NormalizeTopicName(testCase.inputName)
			if ok != testCase.isValid {
				t.Fatalf("Expected name to be considered valid:%t, got valid:%t", testCase.isValid, ok)
			}

			if normalizedName != testCase.normalizedName {
				t.Fatalf("Expected normalized name: %s, got: %s", testCase.normalizedName, normalizedName)
			}
		})
	}
}

func TestDisplayName(t *testing.T) {
	nullStartsAt, _ := time.Parse(time.RFC3339, "2020-10-02T15:00:00Z")
	startsAt := null.NewTime(nullStartsAt, true)

	testData := []struct {
		displayName string
		name        string
		timeline    *models.TopicTimeline
		synonyms    *models.SynonymList
	}{
		{
			name:     "When there is no timeline",
			timeline: nil,
			synonyms: &models.SynonymList{
				Values: []models.Synonym{{"en", "Gnusto"}},
			},
			displayName: "Gnusto",
		},
		{
			name: "When there is a timeline with a format of NONE",
			timeline: &models.TopicTimeline{
				StartsAt:     startsAt,
				PrefixFormat: string(models.TimelinePrefixFormatNone),
			},
			synonyms: &models.SynonymList{
				Values: []models.Synonym{{"en", "Gnusto"}},
			},
			displayName: "Gnusto",
		},
		{
			name: "When there is a timeline with a format of START_YEAR",
			timeline: &models.TopicTimeline{
				StartsAt:     startsAt,
				PrefixFormat: string(models.TimelinePrefixFormatStartYear),
			},
			synonyms: &models.SynonymList{
				Values: []models.Synonym{{"en", "Gnusto"}},
			},
			displayName: "2020 Gnusto",
		},
		{
			name: "When there is a timeline with a format of START_YEAR_MONTH",
			timeline: &models.TopicTimeline{
				StartsAt:     startsAt,
				PrefixFormat: string(models.TimelinePrefixFormatStartYearMonth),
			},
			synonyms: &models.SynonymList{
				Values: []models.Synonym{{"en", "Gnusto"}},
			},
			displayName: "2020-10 Gnusto",
		},
	}

	for _, td := range testData {
		t.Run(td.name, func(t *testing.T) {
			name, err := services.DisplayName(td.timeline, td.synonyms, "en")
			if err != nil {
				t.Fatal(err)
			}

			if name != td.displayName {
				t.Fatalf("Expected %s, got %s", td.displayName, name)
			}
		})
	}
}
