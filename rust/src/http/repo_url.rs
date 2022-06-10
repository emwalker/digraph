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
        Some("news.ycombinator.com") => p.0 == "id",
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
            sha1,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
    fn test_sha1() {
        // The sha1 digest is based on the normalized url
        let url = Url::parse("http://www.google.com").unwrap();
        assert_eq!(url.normalized, "http://www.google.com");
        assert_eq!(url.sha1, "738ddf35b3a85a7a6ba7b232bd3d5f1e4d284ad1");

        let url = Url::parse("http://some.url.com").unwrap();
        assert_eq!(url.normalized, "http://some.url.com");
        assert_eq!(url.sha1, "85cdd80985b9fef9ec0bc1d1ab2aeb7bd4efef86");
    }
}
