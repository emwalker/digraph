use sha1::{Digest, Sha1};
use std::borrow::Cow;
use url;

use crate::prelude::*;

// Struct for urls that are saved as links to a repo.  This class is not a general purpose wrapper
// for urls, as the needs for keeping track of a page on a site differ from one site to another. For
// example, often you can strip off query parameters for a normalized link, but you must keep the
// "id" parameter in the case of a link to a page on Hacker News.
//
// Eventually this kind of site-level handling of links can be moved into configs that are stored
// in a repo or in the database, but for now we just hard-code the handling here.
#[derive(Clone, Debug)]
pub struct Url {
    pub input: String,
    pub normalized: String,
    pub path: String,
    pub sha1: String,
}

impl std::fmt::Display for Url {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "{}", self.normalized)
    }
}

type Pair<'r> = (Cow<'r, str>, Cow<'r, str>);

fn sha1_digest(normalized: &[u8]) -> String {
    let hash = Sha1::digest(normalized);
    format!("{:x}", hash)
}

fn make_filter<'r>(host: Option<&str>) -> impl Fn(&Pair<'r>) -> bool + '_ {
    move |p: &Pair<'r>| match host {
        Some("abcnews.go.com") => p.0 == "id",
        Some("khpg.org") => p.0 == "id",
        Some("news.ycombinator.com") => p.0 == "id",
        Some("newscenter.sdsu.edu") => p.0 == "sid",
        Some("scholarworks.umass.edu") => p.0 == "article" || p.0 == "context",
        Some("www.baylor.edu") => p.0 == "action" || p.0 == "story",
        Some("www.c-span.org") => true,
        Some("www.dur.ac.uk") => p.0 == "itemno",
        Some("www.facebook.com") => p.0 == "__xts__[0]" || p.0 == "v",
        Some("www.greenbeltmd.gov") => p.0 == "id",
        Some("www.koreaherald.com") => p.0 == "ud",
        Some("www.lenr-forum.com") => p.0 == "pageNo",
        Some("www.nzherald.co.nz") => p.0 == "objectid",
        Some("www.sourcewatch.org") => p.0 == "title",
        Some("www.urbandictionary.com") => p.0 == "term",
        Some("www.youtube.com") => p.0 == "v" || p.0 == "t",
        _ => false,
    }
}

fn parse_url(input: &str) -> Result<url::Url> {
    let url = url::Url::parse(input)?;
    let host = url.host_str();
    let filter = make_filter(host);
    let query: Vec<(_, _)> = url.query_pairs().filter(filter).collect();

    let mut url2 = url.clone();
    url2.set_query(None);

    for pair in query {
        url2.query_pairs_mut()
            .append_pair(&pair.0.to_string()[..], &pair.1.to_string()[..]);
    }

    match host {
        Some("mail.google.com") => {}
        _ => url2.set_fragment(None),
    }

    Ok(url2)
}

impl Url {
    pub fn parse(input: &str) -> Result<Self> {
        let url = parse_url(input)?;
        let input = input.to_string();

        // Strip off the path if it is just "/" in order to avoid duplicate urls, since this is what
        // has been done for a long time.
        let normalized = match url.path() {
            "/" => {
                let link = format!("{}", url);
                link.get(0..link.len() - 1).unwrap_or_default().to_string()
            }
            _ => format!("{}", url),
        };

        let sha1 = sha1_digest(normalized.as_bytes());

        Ok(Self {
            input,
            normalized,
            path: url.path().to_string(),
            sha1,
        })
    }

    pub fn is_valid_url(input: &str) -> bool {
        Self::parse(input).is_ok()
    }

    pub fn ends_with(&self, suffix: &str) -> bool {
        self.path.ends_with(suffix)
    }

    pub fn is_pdf(&self) -> bool {
        self.ends_with(".pdf")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sha1() {
        // The sha1 digest is based on the normalized url
        let url = Url::parse("http://www.google.com").unwrap();
        assert_eq!(url.normalized, "http://www.google.com");
        assert_eq!(url.sha1, "738ddf35b3a85a7a6ba7b232bd3d5f1e4d284ad1");

        let url = Url::parse("http://some.url.com").unwrap();
        assert_eq!(url.normalized, "http://some.url.com");
        assert_eq!(url.sha1, "85cdd80985b9fef9ec0bc1d1ab2aeb7bd4efef86");
    }

    #[test]
    fn test_is_valid() {
        assert!(Url::is_valid_url("https://www.google.com"));
        assert!(!Url::is_valid_url("Some name"));
    }

    #[test]
    fn test_simple_case() {
        let url = Url::parse("http://www.google.com").unwrap();
        assert_eq!(url.normalized, "http://www.google.com");
    }

    #[test]
    fn test_url_params() {
        let url = Url::parse("https://www.reuters.com/some-title?utm_source=reddit.com").unwrap();
        assert_eq!(url.normalized, "https://www.reuters.com/some-title");
    }

    #[test]
    fn test_ends_with() {
        let url = Url::parse("https://www.dni.gov//Prelimary-Assessment-UAP-20210625.pdf?q=something").unwrap();
        assert_eq!(url.ends_with(".pdf"), true);

        let url = Url::parse("https://www.dni.gov//Prelimary-Assessment-UAP-20210625.html?q=something").unwrap();
        assert_eq!(url.ends_with(".pdf"), false);
    }

    #[test]
    fn test_is_pdf() {
        let url = Url::parse("https://www.dni.gov//Prelimary-Assessment-UAP-20210625.pdf?q=something").unwrap();
        assert_eq!(url.is_pdf(), true);

        let url = Url::parse("https://www.dni.gov//Prelimary-Assessment-UAP-20210625.html?q=something").unwrap();
        assert_eq!(url.is_pdf(), false);
    }

    #[test]
    fn test_hacker_news_article() {
        // The id parameter is preserved
        let url = Url::parse("https://news.ycombinator.com/item?id=18504300").unwrap();
        assert_eq!(
            url.normalized,
            "https://news.ycombinator.com/item?id=18504300"
        );
    }

    #[test]
    fn test_youtube_video() {
        // The id parameter is preserved
        let url = Url::parse("https://www.youtube.com/watch?v=Wx_2SVm9Jgo&list=PLJ8cMiYb3G5eYGt47YpJcNhILyYLmV-tW&index=3&t=0s").unwrap();
        assert_eq!(
            url.normalized,
            "https://www.youtube.com/watch?v=Wx_2SVm9Jgo&t=0s"
        );
    }

    #[test]
    fn test_bugfix() {
        let url = Url::parse("https://quaderno.io/stripe-vat-subscriptions/").unwrap();
        assert_eq!(
            url.normalized,
            "https://quaderno.io/stripe-vat-subscriptions/"
        );
    }

    #[test]
    fn test_nytimes_link() {
        let url = Url::parse("https://www.nytimes.com/2019/04/12/world/canada/foreign-election-interference-social-media.html?partner=rss&emc=rss").unwrap();
        assert_eq!(
            url.normalized,
            "https://www.nytimes.com/2019/04/12/world/canada/foreign-election-interference-social-media.html"
        );
    }

    #[test]
    fn test_link_with_an_anchor() {
        let url = Url::parse("https://www.buzzfeed.com/craigsilverman/fever-swamp-election?utm_term=.ug4NRgEQDe#.lszgG6PJZr").unwrap();
        assert_eq!(
            url.normalized,
            "https://www.buzzfeed.com/craigsilverman/fever-swamp-election"
        );
    }

    #[test]
    fn test_gmail_link() {
        let url = Url::parse("https://mail.google.com/mail/u/0/#inbox").unwrap();
        assert_eq!(url.normalized, "https://mail.google.com/mail/u/0/#inbox");
    }

    #[test]
    fn test_urban_dictionary() {
        let url = Url::parse(
            "https://www.urbandictionary.com/define.php?term=Vote%20from%20the%20rooftops",
        )
        .unwrap();
        assert_eq!(
            url.normalized,
            "https://www.urbandictionary.com/define.php?term=Vote+from+the+rooftops"
        );
    }

    #[test]
    fn test_abcnews_link() {
        let url = Url::parse("https://abcnews.go.com/US/facebook-takes-proud-boys-american-guard-accounts-connected/story?cid=clicksource_4380645_2_heads_hero_live_twopack_hed&id=71286604").unwrap();
        assert_eq!(
            url.normalized,
            "https://abcnews.go.com/US/facebook-takes-proud-boys-american-guard-accounts-connected/story?id=71286604"
        );
    }

    #[test]
    fn test_facebook_link() {
        let url = Url::parse("https://www.facebook.com/kristof/posts/10159885205317891?__xts__[0]=68.ARAVnkUTUgiRHe7PEE2SsJ8HPxpc50fo9LzoUWxjsgXHvmgtl-NE8VFhGrr4qBIZt7cxN9cvFsH8nVaD0IAVqLeyGe7KsnhjpJxJb8pio_yTPi6m0aKQr8SwTr1950y6fKrObouJIz5ai3L0XEqb0RcN7XnNtGyglUdu76Md2B5qCreEQMveNjWjaw2lNQEAYlSuU7uENm2F8fae1WBozYwBtzgYayLDzVJhZ_VJMsDq9qhaDDQVQ8v3ZxNpcLWJz2PlRPJ8lcd_QsctED82cujRarYxRMSyx0RwGUj-zvljdBuF-sPSdIKyFNo5GE3RBu_qSCL7TUQ").unwrap();
        assert_eq!(
            url.normalized,
            "https://www.facebook.com/kristof/posts/10159885205317891?__xts__%5B0%5D=68.ARAVnkUTUgiRHe7PEE2SsJ8HPxpc50fo9LzoUWxjsgXHvmgtl-NE8VFhGrr4qBIZt7cxN9cvFsH8nVaD0IAVqLeyGe7KsnhjpJxJb8pio_yTPi6m0aKQr8SwTr1950y6fKrObouJIz5ai3L0XEqb0RcN7XnNtGyglUdu76Md2B5qCreEQMveNjWjaw2lNQEAYlSuU7uENm2F8fae1WBozYwBtzgYayLDzVJhZ_VJMsDq9qhaDDQVQ8v3ZxNpcLWJz2PlRPJ8lcd_QsctED82cujRarYxRMSyx0RwGUj-zvljdBuF-sPSdIKyFNo5GE3RBu_qSCL7TUQ"
        );
    }

    #[test]
    fn test_cell_journal_link() {
        let url = Url::parse("https://www.cell.com/cell/pdf/S0092-8674(20)30567-5.pdf?_returnURL=https%3A%2F%2Flinkinghub.elsevier.com%2Fretrieve%2Fpii%2FS0092867420305675%3Fshowall%3Dtrue").unwrap();
        assert_eq!(
            url.normalized,
            "https://www.cell.com/cell/pdf/S0092-8674(20)30567-5.pdf"
        );
    }

    #[test]
    fn test_durham_university_link() {
        let url = Url::parse("https://www.dur.ac.uk/research/news/item/?itemno=42260").unwrap();
        assert_eq!(
            url.normalized,
            "https://www.dur.ac.uk/research/news/item/?itemno=42260"
        );
    }

    #[test]
    fn test_baylor_university_link() {
        let url = Url::parse(
            "https://www.baylor.edu/mediacommunications/news.php?action=story&story=219716",
        )
        .unwrap();
        assert_eq!(
            url.normalized,
            "https://www.baylor.edu/mediacommunications/news.php?action=story&story=219716"
        );
    }

    #[test]
    fn test_umass_link() {
        let url = Url::parse("https://scholarworks.umass.edu/cgi/viewcontent.cgi?article=1004&context=eng_faculty_pubs").unwrap();
        assert_eq!(
            url.normalized,
            "https://scholarworks.umass.edu/cgi/viewcontent.cgi?article=1004&context=eng_faculty_pubs"
        );
    }

    #[test]
    fn test_san_diego_university_link() {
        let url =
            Url::parse("https://newscenter.sdsu.edu/sdsu_newscenter/news_story.aspx?sid=78119")
                .unwrap();
        assert_eq!(
            url.normalized,
            "https://newscenter.sdsu.edu/sdsu_newscenter/news_story.aspx?sid=78119"
        );
    }

    #[test]
    fn test_human_rights_ukraine_link() {
        let url =
            Url::parse("http://khpg.org/en/index.php?id=1597789267&fbclid=IwAR0TszHB5J_sEHt6ZRBnzMWuVCH_Ec2t8stvpSs8vt32sUnZYkVOlRXYnoY")
                .unwrap();
        assert_eq!(url.normalized, "http://khpg.org/en/index.php?id=1597789267");
    }

    #[test]
    fn test_greenbelt_link() {
        let url = Url::parse("https://www.greenbeltmd.gov/home/showdocument?id=2656").unwrap();
        assert_eq!(
            url.normalized,
            "https://www.greenbeltmd.gov/home/showdocument?id=2656"
        );
    }

    #[test]
    fn test_nz_herald_link() {
        let url = Url::parse(
            "https://www.nzherald.co.nz/world/news/article.cfm?c_id=2&objectid=12358821",
        )
        .unwrap();
        assert_eq!(
            url.normalized,
            "https://www.nzherald.co.nz/world/news/article.cfm?objectid=12358821"
        );
    }

    #[test]
    fn test_facebook_video() {
        let url = Url::parse("https://www.facebook.com/watch/?v=781916212211423").unwrap();
        assert_eq!(
            url.normalized,
            "https://www.facebook.com/watch/?v=781916212211423"
        );
    }

    #[test]
    fn test_source_watch_page() {
        let url =
            Url::parse("https://www.sourcewatch.org/index.php?title=Honest_Elections_Project")
                .unwrap();
        assert_eq!(
            url.normalized,
            "https://www.sourcewatch.org/index.php?title=Honest_Elections_Project"
        );
    }

    #[test]
    fn test_lenr_forum() {
        let url = Url::parse("https://www.lenr-forum.com/forum/thread/6017-mizuno-replication-and-materials-only/?pageNo=2").unwrap();
        assert_eq!(
            url.normalized,
            "https://www.lenr-forum.com/forum/thread/6017-mizuno-replication-and-materials-only/?pageNo=2"
        );
    }

    #[test]
    fn test_koreaherald_link() {
        let url = Url::parse("http://www.koreaherald.com/view.php?ud=20210316000213").unwrap();
        assert_eq!(
            url.normalized,
            "http://www.koreaherald.com/view.php?ud=20210316000213"
        );
    }

    #[test]
    fn test_cspan_video() {
        let url = Url::parse("https://www.c-span.org/video/?c5004713/user-clip-mcgahn-quotes").unwrap();
        assert_eq!(
            url.normalized,
            "https://www.c-span.org/video/?c5004713%2Fuser-clip-mcgahn-quotes="
        );
    }
}
