package services_test

import (
	"testing"

	"github.com/emwalker/digraph/cmd/frontend/services"
)

func TestNormalizeURL(t *testing.T) {
	testCases := []struct {
		name         string
		inputURL     string
		canonicalURL string
	}{
		{
			name:         "A basic case",
			inputURL:     "http://some.url.com",
			canonicalURL: "http://some.url.com",
		},
		{
			name:         "Y Combinator comment section",
			inputURL:     "https://news.ycombinator.com/item?id=18504300",
			canonicalURL: "https://news.ycombinator.com/item?id=18504300",
		},
		{
			name:         "A bugfix",
			inputURL:     "https://quaderno.io/stripe-vat-subscriptions/",
			canonicalURL: "https://quaderno.io/stripe-vat-subscriptions/",
		},
		{
			name:         "A New York Times article",
			inputURL:     "https://www.nytimes.com/2019/04/12/world/canada/foreign-election-interference-social-media.html?partner=rss&emc=rss",
			canonicalURL: "https://www.nytimes.com/2019/04/12/world/canada/foreign-election-interference-social-media.html",
		},
		{
			name:         "An article from the Independent",
			inputURL:     "https://www.independent.co.uk/news/world/middle-east/saudi-arabia-born-babies-streets-abortion-marriage-wedlock-a8867571.html?utm_source=reddit.com",
			canonicalURL: "https://www.independent.co.uk/news/world/middle-east/saudi-arabia-born-babies-streets-abortion-marriage-wedlock-a8867571.html",
		},
		{
			name:         "An article from Reuters",
			inputURL:     "https://www.reuters.com/article/france-electricity-solarpower/sunny-spell-boosts-french-solar-generation-to-record-level-idUSL8N21U58M?utm_source=reddit.com",
			canonicalURL: "https://www.reuters.com/article/france-electricity-solarpower/sunny-spell-boosts-french-solar-generation-to-record-level-idUSL8N21U58M",
		},
		{
			name:         "A Business Insider article",
			inputURL:     "https://www.businessinsider.com/gnss-hacking-spoofing-jamming-russians-screwing-with-gps-2019-4?utm_source=reddit.com",
			canonicalURL: "https://www.businessinsider.com/gnss-hacking-spoofing-jamming-russians-screwing-with-gps-2019-4",
		},
		{
			name:         "A YouTube video",
			inputURL:     "https://www.youtube.com/watch?v=Wx_2SVm9Jgo&list=PLJ8cMiYb3G5eYGt47YpJcNhILyYLmV-tW&index=3&t=0s",
			canonicalURL: "https://www.youtube.com/watch?v=Wx_2SVm9Jgo",
		},
		{
			name:         "A BuzzFeed article",
			inputURL:     "https://www.buzzfeed.com/craigsilverman/fever-swamp-election?utm_term=.ug4NRgEQDe#.lszgG6PJZr",
			canonicalURL: "https://www.buzzfeed.com/craigsilverman/fever-swamp-election",
		},
		{
			name:         "A Gmail link",
			inputURL:     "https://mail.google.com/mail/u/0/#inbox",
			canonicalURL: "https://mail.google.com/mail/u/0/#inbox",
		},
		{
			name:         "A link with several utm fields",
			inputURL:     "https://apnews.com/e087076881f3449fa603e4434d164ac9?utm_campaign=Bundle&utm_medium=referral&utm_source=Bundle&",
			canonicalURL: "https://apnews.com/e087076881f3449fa603e4434d164ac9",
		},
		{
			name:         "An article from the Guardian",
			inputURL:     "https://www.theguardian.com/money/2019/apr/17/who-owns-england-thousand-secret-landowners-author?CMP=Share_AndroidApp_WhatsApp",
			canonicalURL: "https://www.theguardian.com/money/2019/apr/17/who-owns-england-thousand-secret-landowners-author",
		},
		{
			name:         "An article with an rss parameter",
			inputURL:     "https://www.ajicjournal.org/article/S0196-6553(19)30151-8/fulltext?rss=yes",
			canonicalURL: "https://www.ajicjournal.org/article/S0196-6553(19)30151-8/fulltext",
		},
		{
			name:         "An Indie Hackers article",
			inputURL:     "https://www.indiehackers.com/interview/d2c4d6f8fa?utm_source=Indie+Hackers+Newsletter&utm_campaign=indie-hackers-newsletter-20190417&utm_medium=email",
			canonicalURL: "https://www.indiehackers.com/interview/d2c4d6f8fa",
		},
		{
			name:         "A Vice article",
			inputURL:     "https://news.vice.com/en_us/article/43jw79/how-pro-trump-grifters-used-medium-to-smear-pete-buttigieg?utm_medium=vicenewsfacebook&fbclid=IwAR1RQ7CVhzbpnkThsJiYS2P_xHvVF93y_Z92wUZgWiPu9sId0x3yQBjeA2Q&utm_source=reddit.com",
			canonicalURL: "https://news.vice.com/en_us/article/43jw79/how-pro-trump-grifters-used-medium-to-smear-pete-buttigieg",
		},
		{
			name:         "A Slate article",
			inputURL:     "https://slate.com/news-and-politics/2019/05/william-barr-donald-trump-calm-defense.html?via=homepage_taps_top",
			canonicalURL: "https://slate.com/news-and-politics/2019/05/william-barr-donald-trump-calm-defense.html",
		},
		{
			name:         "A dictionary.com definition",
			inputURL:     "https://www.dictionary.com/browse/temporize?s=t",
			canonicalURL: "https://www.dictionary.com/browse/temporize",
		},
		{
			name:         "A CNN article",
			inputURL:     "https://www.cnn.com/2019/05/04/tech/trump-social-media-twitter-facebook/index.html?utm_source=feedburner&utm_medium=feed&utm_campaign=Feed%3A+rss%2Fcnn_latest+%28RSS%3A+CNN+-+Most+Recent%29",
			canonicalURL: "https://www.cnn.com/2019/05/04/tech/trump-social-media-twitter-facebook/index.html",
		},
		{
			name:         "A Washington Post article",
			inputURL:     "https://www.washingtonpost.com/national/health-science/microbes-called-extremophiles-might-combat-superbugs-biowarfare-agents/2019/05/03/6e0277f4-6b81-11e9-8f44-e8d8bb1df986_story.html?utm_term=.0aa16c680491",
			canonicalURL: "https://www.washingtonpost.com/national/health-science/microbes-called-extremophiles-might-combat-superbugs-biowarfare-agents/2019/05/03/6e0277f4-6b81-11e9-8f44-e8d8bb1df986_story.html",
		},
	}

	for _, testCase := range testCases {
		t.Run(testCase.name, func(t *testing.T) {
			url, err := services.NormalizeURL(testCase.inputURL)
			if err != nil {
				t.Fatal(err)
			}

			if url.CanonicalURL != testCase.canonicalURL {
				t.Fatalf("Unexpected url: %s, expected: %s", url.CanonicalURL, testCase.canonicalURL)
			}
		})
	}
}

func TestSha1Value(t *testing.T) {
	var url *services.URL
	var err error

	if url, err = services.NormalizeURL("http://some.url.com"); err != nil {
		t.Fatal(err)
	}

	if url.Sha1 != "85cdd80985b9fef9ec0bc1d1ab2aeb7bd4efef86" {
		t.Fatalf("Unexpected SHA1: %s", url.Sha1)
	}
}
