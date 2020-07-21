package pageinfo_test

import (
	"testing"

	"github.com/emwalker/digraph/cmd/frontend/services/pageinfo"
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
		{
			name:         "Another New York Times article",
			inputURL:     "https://www.nytimes.com/2013/10/14/world/to-ousted-boss-arms-watchdog-was-seen-as-an-obstacle-in-iraq.html?_r=1&",
			canonicalURL: "https://www.nytimes.com/2013/10/14/world/to-ousted-boss-arms-watchdog-was-seen-as-an-obstacle-in-iraq.html",
		},
		{
			name:         "An article in the Atlantic",
			inputURL:     "https://www.theatlantic.com/ideas/archive/2019/05/coming-generation-war/588670/?te=1&nl=morning-briefing&emc=edit_NN_p_20190515&section=whatElse",
			canonicalURL: "https://www.theatlantic.com/ideas/archive/2019/05/coming-generation-war/588670/",
		},
		{
			name:         "A tweet",
			inputURL:     "https://twitter.com/Marco_Langbroek/status/1132389022519762945?ref_src=twsrc%5Etfw%7Ctwcamp%5Etweetembed%7Ctwterm%5E1132389022519762945&ref_url=https%3A%2F%2Fwww.geekwire.com%2F2019%2Fsightings-spacexs-starlink-satellites-spark-awe-astronomical-angst%2F",
			canonicalURL: "https://twitter.com/Marco_Langbroek/status/1132389022519762945",
		},
		{
			name:         "An article in Mother Jones",
			inputURL:     "https://www.motherjones.com/kevin-drum/2019/06/nancy-pelosi-is-smart-part-895/?fbclid=IwAR0O-JdeFKAu8wBjUKrEZeqpY9zINW2N09MwsA-OeJHjEX4JliXg_ybt92Q",
			canonicalURL: "https://www.motherjones.com/kevin-drum/2019/06/nancy-pelosi-is-smart-part-895/",
		},
		{
			name:         "An article in The Hill",
			inputURL:     "https://thehill.com/regulation/court-battles/445184-federal-judge-backs-trump-rules-emergency-declaration-on-wall-can?rnd=1559598882",
			canonicalURL: "https://thehill.com/regulation/court-battles/445184-federal-judge-backs-trump-rules-emergency-declaration-on-wall-can",
		},
		{
			name:         "A book on Amazon",
			inputURL:     "https://www.amazon.com/gp/offer-listing/0743228383/ref=tmm_hrd_used_olp_sr?ie=UTF8&condition=used&qid=&sr=",
			canonicalURL: "https://www.amazon.com/gp/offer-listing/0743228383/ref=tmm_hrd_used_olp_sr",
		},
		{
			name:         "An article in Scientific American",
			inputURL:     "https://www.scientificamerican.com/article/atheism-is-inconsistent-with-the-scientific-method-prizewinning-physicist-says/?redirect=1",
			canonicalURL: "https://www.scientificamerican.com/article/atheism-is-inconsistent-with-the-scientific-method-prizewinning-physicist-says/",
		},
		{
			name:         "An article in the Daily Beast",
			inputURL:     "https://www.thedailybeast.com/how-natural-news-became-a-conspiracy-hub-rivaling-infowars?someparam",
			canonicalURL: "https://www.thedailybeast.com/how-natural-news-became-a-conspiracy-hub-rivaling-infowars",
		},
		{
			name:         "An Urban Dictionary definition",
			inputURL:     "https://www.urbandictionary.com/define.php?term=Vote%20from%20the%20rooftops",
			canonicalURL: "https://www.urbandictionary.com/define.php?term=Vote+from+the+rooftops",
		},
		{
			name:         "A redirectedFrom parameter",
			inputURL:     "https://academic.oup.com/cid/advance-article-abstract/doi/10.1093/cid/ciaa436/5847659?redirectedFrom=fulltext",
			canonicalURL: "https://academic.oup.com/cid/advance-article-abstract/doi/10.1093/cid/ciaa436/5847659",
		},
		{
			name:         "An ABC News article",
			inputURL:     "https://abcnews.go.com/US/facebook-takes-proud-boys-american-guard-accounts-connected/story?cid=clicksource_4380645_2_heads_hero_live_twopack_hed&id=71286604",
			canonicalURL: "https://abcnews.go.com/US/facebook-takes-proud-boys-american-guard-accounts-connected/story?id=71286604",
		},
		{
			name:         "A link to a Facebook thread",
			inputURL:     "https://www.facebook.com/kristof/posts/10159885205317891?__xts__[0]=68.ARAVnkUTUgiRHe7PEE2SsJ8HPxpc50fo9LzoUWxjsgXHvmgtl-NE8VFhGrr4qBIZt7cxN9cvFsH8nVaD0IAVqLeyGe7KsnhjpJxJb8pio_yTPi6m0aKQr8SwTr1950y6fKrObouJIz5ai3L0XEqb0RcN7XnNtGyglUdu76Md2B5qCreEQMveNjWjaw2lNQEAYlSuU7uENm2F8fae1WBozYwBtzgYayLDzVJhZ_VJMsDq9qhaDDQVQ8v3ZxNpcLWJz2PlRPJ8lcd_QsctED82cujRarYxRMSyx0RwGUj-zvljdBuF-sPSdIKyFNo5GE3RBu_qSCL7TUQ",
			canonicalURL: "https://www.facebook.com/kristof/posts/10159885205317891?__xts__[0]=68.ARAVnkUTUgiRHe7PEE2SsJ8HPxpc50fo9LzoUWxjsgXHvmgtl-NE8VFhGrr4qBIZt7cxN9cvFsH8nVaD0IAVqLeyGe7KsnhjpJxJb8pio_yTPi6m0aKQr8SwTr1950y6fKrObouJIz5ai3L0XEqb0RcN7XnNtGyglUdu76Md2B5qCreEQMveNjWjaw2lNQEAYlSuU7uENm2F8fae1WBozYwBtzgYayLDzVJhZ_VJMsDq9qhaDDQVQ8v3ZxNpcLWJz2PlRPJ8lcd_QsctED82cujRarYxRMSyx0RwGUj-zvljdBuF-sPSdIKyFNo5GE3RBu_qSCL7TUQ",
		},
		{
			name:         "An article in the journal Cell",
			inputURL:     "https://www.cell.com/cell/pdf/S0092-8674(20)30567-5.pdf?_returnURL=https%3A%2F%2Flinkinghub.elsevier.com%2Fretrieve%2Fpii%2FS0092867420305675%3Fshowall%3Dtrue",
			canonicalURL: "https://www.cell.com/cell/pdf/S0092-8674(20)30567-5.pdf",
		},
		{
			name:         "An article in Huffington Post",
			inputURL:     "https://www.huffingtonpost.co.uk/entry/katie-hopkins-account-permanently-suspended_uk_5eece139c5b6e9623c8179bf?guce_referrer=aHR0cHM6Ly90LmNvL2x3bFREcTFrVEg_YW1wPTE&guce_referrer_sig=AQAAADtPeiajMvrnzZWDtw7yy-JiohkfzxDxV7FaOXDaDuB_gDWIvwG7SMs0qsUS959IQ8OC0rehnZAZGF6OOnA3TRwCJayLF9QcUZp0EMlJ0aP4Gjt_4ce0v9M5wy5qV5via8b17RMJt-K2_zpJX2jOn_sh3RBeZfxDDQxXMDAJZN19&guccounter=2",
			canonicalURL: "https://www.huffingtonpost.co.uk/entry/katie-hopkins-account-permanently-suspended_uk_5eece139c5b6e9623c8179bf",
		},
		{
			name:         "An NPR article",
			inputURL:     "https://www.npr.org/2020/06/20/881148365/geoffrey-berman-u-s-attorney-who-prosecuted-trump-allies-says-he-wont-quit?utm_source=npr_newsletter&utm_medium=email&utm_content=20200619&utm_term=4628612&utm_campaign=breaking-news&utm_id=2400537&orgid=309",
			canonicalURL: "https://www.npr.org/2020/06/20/881148365/geoffrey-berman-u-s-attorney-who-prosecuted-trump-allies-says-he-wont-quit",
		},
		{
			name:         "An NBC News article",
			inputURL:     "https://www.nbcnewyork.com/news/local/u-s-attorney-in-manhattan-who-oversaw-key-cases-of-trump-allies-abruptly-resigns/2474381/?__twitter_impression=true&_osource=SocialFlowTwt_NYBrand&amp=",
			canonicalURL: "https://www.nbcnewyork.com/news/local/u-s-attorney-in-manhattan-who-oversaw-key-cases-of-trump-allies-abruptly-resigns/2474381/",
		},
		{
			name:         "A note from the CDC",
			inputURL:     "https://www.cdc.gov/mmwr/volumes/69/wr/mm6924a2.htm?s_cid=mm6924a2_w",
			canonicalURL: "https://www.cdc.gov/mmwr/volumes/69/wr/mm6924a2.htm",
		},
		{
			name:         "A URL with a glcid parameter",
			inputURL:     "https://www.southafrica.net/gl/en/travel/article/the-face-of-an-uprising-the-hector-pieterson-museum-soweto?gclid=CjwKCAjw_-D3BRBIEiwAjVMy7AR_6Aaw2XjWkV36MEbdlqaCY2sHqXCc655NWoMZ8NtDtI30KF1UCxoCF0IQAvD_BwE",
			canonicalURL: "https://www.southafrica.net/gl/en/travel/article/the-face-of-an-uprising-the-hector-pieterson-museum-soweto",
		},
		{
			name:         "A URL from Durham University",
			inputURL:     "https://www.dur.ac.uk/research/news/item/?itemno=42260",
			canonicalURL: "https://www.dur.ac.uk/research/news/item/?itemno=42260",
		},
	}

	for _, testCase := range testCases {
		t.Run(testCase.name, func(t *testing.T) {
			url, err := pageinfo.NormalizeURL(testCase.inputURL)
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
	var url *pageinfo.URL
	var err error

	if url, err = pageinfo.NormalizeURL("http://some.url.com"); err != nil {
		t.Fatal(err)
	}

	if url.Sha1 != "85cdd80985b9fef9ec0bc1d1ab2aeb7bd4efef86" {
		t.Fatalf("Unexpected SHA1: %s", url.Sha1)
	}
}

func TestIsURL(t *testing.T) {
	testCases := []struct {
		name  string
		input string
		isURL bool
	}{
		{
			name:  "A basic case",
			input: "http://some.url.com",
			isURL: true,
		},
		{
			name:  "When unusual case is used",
			input: "HTTP://some.url.com",
			isURL: true,
		},
		{
			name:  "A title with a colon",
			input: "Sarecycline: a narrow spectrum tetracycline for the treatment of moderate-to-severe acne vulgaris",
			isURL: false,
		},
	}

	for _, testCase := range testCases {
		t.Run(testCase.name, func(t *testing.T) {
			actual := pageinfo.IsURL(testCase.input)
			if actual != testCase.isURL {
				t.Fatalf("Unexpected result: %t, actual result: %t", testCase.isURL, actual)
			}
		})
	}
}
