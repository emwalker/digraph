use std::borrow::Cow;
use std::hash::Hasher;
use url;

use crate::prelude::*;
use crate::types::sha256_base64;

// There's an occasional issue in which a site will not respond or will respond in a way that the
// current Axum connection setup can't handle, causing an Axum thread to be tied up indefinitely.
// This is a workaround until a proper timeout can be put in place to deal with these cases.
const AVOID_REQUESTS: &[&str] = &["miamiherald.com"];

// Struct for urls that are saved as links to a repo.  This class is not a general purpose wrapper
// for urls, as the needs for keeping track of a page on a site differ from one site to another. For
// example, often you can strip off query parameters for a normalized link, but you must keep the
// "id" parameter in the case of a link to a page on Hacker News.
//
// Eventually this kind of site-level handling of links can be moved into configs that are stored
// in a repo or in the database, but for now we just hard-code the handling here.
#[derive(Clone, Debug)]
pub struct RepoUrl {
    pub host: String,
    pub input: String,
    pub normalized: String,
    pub path: String,
    pub sha256: String,
}

impl std::cmp::PartialEq for RepoUrl {
    fn eq(&self, other: &Self) -> bool {
        self.sha256 == other.sha256
    }
}

impl std::cmp::Eq for RepoUrl {}

impl std::cmp::Ord for RepoUrl {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.normalized.cmp(&other.normalized)
    }
}

impl std::cmp::PartialOrd for RepoUrl {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl std::fmt::Display for RepoUrl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "{}", self.normalized)
    }
}

impl std::hash::Hash for RepoUrl {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.sha256.hash(state);
    }
}

impl RepoUrl {
    pub fn parse(input: &str) -> Result<Self> {
        let (url, host) = parse_url(input)?;
        let input = input.to_string();
        let normalized = format!("{url}");
        let sha256_base64 = sha256_base64(&normalized);

        Ok(Self {
            input,
            normalized,
            host,
            path: url.path().to_string(),
            sha256: sha256_base64,
        })
    }

    pub fn is_valid_url(input: &str) -> bool {
        Self::parse(input).is_ok()
    }

    pub fn avoid_requests(&self) -> bool {
        AVOID_REQUESTS
            .iter()
            .any(|suffix| self.host.ends_with(suffix))
    }

    pub fn ends_with(&self, suffix: &str) -> bool {
        self.path.ends_with(suffix)
    }

    pub fn id(&self) -> Result<ExternalId> {
        ExternalId::try_from(&self.sha256)
    }

    pub fn is_pdf(&self) -> bool {
        self.ends_with(".pdf")
    }
}

type Pair<'r> = (Cow<'r, str>, Cow<'r, str>);

fn make_filter<'r>(host: &str) -> impl Fn(&Pair<'r>) -> bool + '_ {
    move |p: &Pair<'r>| match host {
        "abcnews.go.com" => p.0 == "id",
        "khpg.org" => p.0 == "id",
        "news.ycombinator.com" => p.0 == "id",
        "newscenter.sdsu.edu" => p.0 == "sid",
        "scholarworks.umass.edu" => p.0 == "article" || p.0 == "context",
        "www.baylor.edu" => p.0 == "action" || p.0 == "story",
        "www.c-span.org" => true,
        "www.dur.ac.uk" => p.0 == "itemno",
        "www.facebook.com" => p.0 == "__xts__[0]" || p.0 == "v",
        "www.greenbeltmd.gov" => p.0 == "id",
        "www.koreaherald.com" => p.0 == "ud",
        "www.lenr-forum.com" => p.0 == "pageNo",
        "www.nzherald.co.nz" => p.0 == "objectid",
        "www.sourcewatch.org" => p.0 == "title",
        "www.urbandictionary.com" => p.0 == "term",
        "www.youtube.com" => p.0 == "v" || p.0 == "t",
        _ => false,
    }
}

fn parse_url(input: &str) -> Result<(url::Url, String)> {
    let url = url::Url::parse(input)?;

    if !url.has_host() {
        return Err(Error::UrlParse(format!("invalid url: {input}")));
    }

    let host = url
        .host_str()
        .ok_or(Error::UrlParse(format!("no host: {input}")))?;
    let filter = make_filter(host);
    let query: Vec<(_, _)> = url.query_pairs().filter(filter).collect();

    let mut url2 = url.clone();
    url2.set_query(None);

    for pair in query {
        url2.query_pairs_mut()
            .append_pair(&pair.0.to_string()[..], &pair.1.to_string()[..]);
    }

    match host {
        "mail.google.com" => {}
        _ => url2.set_fragment(None),
    }

    Ok((url2, host.to_owned()))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse(input: &str) -> RepoUrl {
        RepoUrl::parse(input).unwrap()
    }

    #[test]
    fn sha256() {
        // The sha1 digest is based on the normalized url
        let url = parse("http://www.google.com");
        assert_eq!(url.normalized, "http://www.google.com/");
        assert_eq!(url.sha256, "3QFK9e1rONkTDj9Gb4UORtIblRGZ1ToY7ynuk0FhTq8");

        let url = parse("http://some.url.com");
        assert_eq!(url.normalized, "http://some.url.com/");
        assert_eq!(url.sha256, "xUJjzElMux2Ebv6Fym7OQCIduMu1j0Dy8mWQrobMJbg");
    }

    #[test]
    fn partial_eq() {
        assert_eq!(
            parse("http://www.google.com"),
            parse("http://www.google.com/")
        );
    }

    #[test]
    fn is_valid() {
        assert!(RepoUrl::is_valid_url("https://www.google.com"));
        assert!(!RepoUrl::is_valid_url("Some name"));
        assert!(!RepoUrl::is_valid_url("aaas:"));
    }

    #[test]
    fn simple_case() {
        let url = parse("http://www.google.com");
        assert_eq!(url.normalized, "http://www.google.com/");
    }

    #[test]
    fn url_params() {
        let url = parse("https://www.reuters.com/some-title?utm_source=reddit.com");
        assert_eq!(url.normalized, "https://www.reuters.com/some-title");
    }

    #[test]
    fn ends_with() {
        let url = parse("https://www.dni.gov//Prelimary-Assessment-UAP-20210625.pdf?q=something");
        assert!(url.ends_with(".pdf"));

        let url = parse("https://www.dni.gov//Prelimary-Assessment-UAP-20210625.html?q=something");
        assert!(!url.ends_with(".pdf"));
    }

    #[test]
    fn is_pdf() {
        let url = parse("https://www.dni.gov//Prelimary-Assessment-UAP-20210625.pdf?q=something");
        assert!(url.is_pdf());

        let url = parse("https://www.dni.gov//Prelimary-Assessment-UAP-20210625.html?q=something");
        assert!(!url.is_pdf());
    }

    #[test]
    fn avoid_requests() {
        let url = parse("https://www.miamiherald.com/news/local/environment/climate-change/article303949086.html");
        assert!(url.avoid_requests());
    }

    #[test]
    fn hacker_news_article() {
        // The id parameter is preserved
        let url = parse("https://news.ycombinator.com/item?id=18504300");
        assert_eq!(
            url.normalized,
            "https://news.ycombinator.com/item?id=18504300"
        );
    }

    #[test]
    fn youtube_video() {
        // The id parameter is preserved
        let url = parse("https://www.youtube.com/watch?v=Wx_2SVm9Jgo&list=PLJ8cMiYb3G5eYGt47YpJcNhILyYLmV-tW&index=3&t=0s");
        assert_eq!(
            url.normalized,
            "https://www.youtube.com/watch?v=Wx_2SVm9Jgo&t=0s"
        );
    }

    #[test]
    fn bugfix() {
        let url = parse("https://quaderno.io/stripe-vat-subscriptions/");
        assert_eq!(
            url.normalized,
            "https://quaderno.io/stripe-vat-subscriptions/"
        );
    }

    #[test]
    fn nytimes_link() {
        let url = parse("https://www.nytimes.com/2019/04/12/world/canada/foreign-election-interference-social-media.html?partner=rss&emc=rss");
        assert_eq!(
            url.normalized,
            "https://www.nytimes.com/2019/04/12/world/canada/foreign-election-interference-social-media.html"
        );
    }

    #[test]
    fn link_with_an_anchor() {
        let url = parse("https://www.buzzfeed.com/craigsilverman/fever-swamp-election?utm_term=.ug4NRgEQDe#.lszgG6PJZr");
        assert_eq!(
            url.normalized,
            "https://www.buzzfeed.com/craigsilverman/fever-swamp-election"
        );
    }

    #[test]
    fn gmail_link() {
        let url = parse("https://mail.google.com/mail/u/0/#inbox");
        assert_eq!(url.normalized, "https://mail.google.com/mail/u/0/#inbox");
    }

    #[test]
    fn urban_dictionary() {
        let url =
            parse("https://www.urbandictionary.com/define.php?term=Vote%20from%20the%20rooftops");
        assert_eq!(
            url.normalized,
            "https://www.urbandictionary.com/define.php?term=Vote+from+the+rooftops"
        );
    }

    #[test]
    fn abcnews_link() {
        let url = parse("https://abcnews.go.com/US/facebook-takes-proud-boys-american-guard-accounts-connected/story?cid=clicksource_4380645_2_heads_hero_live_twopack_hed&id=71286604");
        assert_eq!(
            url.normalized,
            "https://abcnews.go.com/US/facebook-takes-proud-boys-american-guard-accounts-connected/story?id=71286604"
        );
    }

    #[test]
    fn facebook_link() {
        let url = parse("https://www.facebook.com/kristof/posts/10159885205317891?__xts__[0]=68.ARAVnkUTUgiRHe7PEE2SsJ8HPxpc50fo9LzoUWxjsgXHvmgtl-NE8VFhGrr4qBIZt7cxN9cvFsH8nVaD0IAVqLeyGe7KsnhjpJxJb8pio_yTPi6m0aKQr8SwTr1950y6fKrObouJIz5ai3L0XEqb0RcN7XnNtGyglUdu76Md2B5qCreEQMveNjWjaw2lNQEAYlSuU7uENm2F8fae1WBozYwBtzgYayLDzVJhZ_VJMsDq9qhaDDQVQ8v3ZxNpcLWJz2PlRPJ8lcd_QsctED82cujRarYxRMSyx0RwGUj-zvljdBuF-sPSdIKyFNo5GE3RBu_qSCL7TUQ");
        assert_eq!(
            url.normalized,
            "https://www.facebook.com/kristof/posts/10159885205317891?__xts__%5B0%5D=68.ARAVnkUTUgiRHe7PEE2SsJ8HPxpc50fo9LzoUWxjsgXHvmgtl-NE8VFhGrr4qBIZt7cxN9cvFsH8nVaD0IAVqLeyGe7KsnhjpJxJb8pio_yTPi6m0aKQr8SwTr1950y6fKrObouJIz5ai3L0XEqb0RcN7XnNtGyglUdu76Md2B5qCreEQMveNjWjaw2lNQEAYlSuU7uENm2F8fae1WBozYwBtzgYayLDzVJhZ_VJMsDq9qhaDDQVQ8v3ZxNpcLWJz2PlRPJ8lcd_QsctED82cujRarYxRMSyx0RwGUj-zvljdBuF-sPSdIKyFNo5GE3RBu_qSCL7TUQ"
        );
    }

    #[test]
    fn cell_journal_link() {
        let url = parse("https://www.cell.com/cell/pdf/S0092-8674(20)30567-5.pdf?_returnURL=https%3A%2F%2Flinkinghub.elsevier.com%2Fretrieve%2Fpii%2FS0092867420305675%3Fshowall%3Dtrue");
        assert_eq!(
            url.normalized,
            "https://www.cell.com/cell/pdf/S0092-8674(20)30567-5.pdf"
        );
    }

    #[test]
    fn durham_university_link() {
        let url = parse("https://www.dur.ac.uk/research/news/item/?itemno=42260");
        assert_eq!(
            url.normalized,
            "https://www.dur.ac.uk/research/news/item/?itemno=42260"
        );
    }

    #[test]
    fn baylor_university_link() {
        let url =
            parse("https://www.baylor.edu/mediacommunications/news.php?action=story&story=219716");
        assert_eq!(
            url.normalized,
            "https://www.baylor.edu/mediacommunications/news.php?action=story&story=219716"
        );
    }

    #[test]
    fn umass_link() {
        let url = parse("https://scholarworks.umass.edu/cgi/viewcontent.cgi?article=1004&context=eng_faculty_pubs");
        assert_eq!(
            url.normalized,
            "https://scholarworks.umass.edu/cgi/viewcontent.cgi?article=1004&context=eng_faculty_pubs"
        );
    }

    #[test]
    fn san_diego_university_link() {
        let url = parse("https://newscenter.sdsu.edu/sdsu_newscenter/news_story.aspx?sid=78119");
        assert_eq!(
            url.normalized,
            "https://newscenter.sdsu.edu/sdsu_newscenter/news_story.aspx?sid=78119"
        );
    }

    #[test]
    fn human_rights_ukraine_link() {
        let url =
            parse("http://khpg.org/en/index.php?id=1597789267&fbclid=IwAR0TszHB5J_sEHt6ZRBnzMWuVCH_Ec2t8stvpSs8vt32sUnZYkVOlRXYnoY");
        assert_eq!(url.normalized, "http://khpg.org/en/index.php?id=1597789267");
    }

    #[test]
    fn greenbelt_link() {
        let url = parse("https://www.greenbeltmd.gov/home/showdocument?id=2656");
        assert_eq!(
            url.normalized,
            "https://www.greenbeltmd.gov/home/showdocument?id=2656"
        );
    }

    #[test]
    fn nz_herald_link() {
        let url =
            parse("https://www.nzherald.co.nz/world/news/article.cfm?c_id=2&objectid=12358821");
        assert_eq!(
            url.normalized,
            "https://www.nzherald.co.nz/world/news/article.cfm?objectid=12358821"
        );
    }

    #[test]
    fn facebook_video() {
        let url = parse("https://www.facebook.com/watch/?v=781916212211423");
        assert_eq!(
            url.normalized,
            "https://www.facebook.com/watch/?v=781916212211423"
        );
    }

    #[test]
    fn source_watch_page() {
        let url = parse("https://www.sourcewatch.org/index.php?title=Honest_Elections_Project");
        assert_eq!(
            url.normalized,
            "https://www.sourcewatch.org/index.php?title=Honest_Elections_Project"
        );
    }

    #[test]
    fn lenr_forum() {
        let url = parse("https://www.lenr-forum.com/forum/thread/6017-mizuno-replication-and-materials-only/?pageNo=2");
        assert_eq!(
            url.normalized,
            "https://www.lenr-forum.com/forum/thread/6017-mizuno-replication-and-materials-only/?pageNo=2"
        );
    }

    #[test]
    fn koreaherald_link() {
        let url = parse("http://www.koreaherald.com/view.php?ud=20210316000213");
        assert_eq!(
            url.normalized,
            "http://www.koreaherald.com/view.php?ud=20210316000213"
        );
    }

    #[test]
    fn cspan_video() {
        let url = parse("https://www.c-span.org/video/?c5004713/user-clip-mcgahn-quotes");
        assert_eq!(
            url.normalized,
            "https://www.c-span.org/video/?c5004713%2Fuser-clip-mcgahn-quotes="
        );
    }
}
