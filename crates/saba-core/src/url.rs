use alloc::{
    collections::BTreeMap,
    string::{String, ToString},
};
use core::{num::ParseIntError, result::Result};

use crate::error::Error;

#[derive(Debug, Clone, PartialEq)]
pub struct Url {
    host: String,
    port: u16,
    path: Option<String>,
    searchpart: BTreeMap<String, String>,
}

impl TryFrom<&str> for Url {
    type Error = Error;

    fn try_from(url: &str) -> Result<Self, Self::Error> {
        if !url.starts_with("http://") {
            return Err(Error::InvalidScheme(url.to_string()));
        }

        let url = url.trim_start_matches("http://");

        let host = extract_host(url);
        let port = extract_port(url)?;
        let path = extract_path(url);
        let searchpart = extract_searchpart(url);

        Ok(Self {
            host,
            port,
            path,
            searchpart,
        })
    }
}

impl TryFrom<String> for Url {
    type Error = Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::try_from(value.as_str())
    }
}

impl Url {
    pub fn host(&self) -> &str {
        &self.host
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub fn path(&self) -> Option<&str> {
        self.path.as_ref().map(|s| s.as_ref())
    }

    pub fn searchpart(&self) -> &BTreeMap<String, String> {
        &self.searchpart
    }

    pub fn is_http(&self) -> bool {
        true
    }
}

fn pre_url_parts(url: &str) -> impl Iterator<Item = &str> {
    url.split('/')
        .next()
        .expect("at least 1 element")
        .splitn(2, ':')
}

fn post_url_parts(url: &str) -> Option<impl Iterator<Item = &str>> {
    url.split_once('/').map(|(_, s)| s.splitn(2, '?'))
}

fn extract_host(url: &str) -> String {
    pre_url_parts(url)
        .next()
        .expect("at least have 1 element")
        .to_string()
}

fn extract_port(url: &str) -> Result<u16, ParseIntError> {
    pre_url_parts(url).nth(1).unwrap_or("80").parse()
}

fn extract_path(url: &str) -> Option<String> {
    post_url_parts(url)
        .and_then(|mut s| s.next())
        .map(|s| s.to_string())
}

fn extract_searchpart(url: &str) -> BTreeMap<String, String> {
    post_url_parts(url)
        .and_then(|mut x| x.nth(1))
        .map(|s| {
            s.split('&')
                .map(|s| {
                    let mut kv = s.splitn(2, '=');
                    let k = kv.next().unwrap_or_default();
                    let v = kv.next().unwrap_or_default();
                    (k.to_string(), v.to_string())
                })
                .collect::<BTreeMap<_, _>>()
        })
        .unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_url_host() {
        let url = "http://example.com";
        let url = Url::try_from(url);

        if let Ok(url) = url {
            assert_eq!(url.host(), "example.com");
            assert_eq!(url.port(), 80);
            assert_eq!(url.path(), None);
            assert_eq!(*url.searchpart(), BTreeMap::default());
        } else {
            panic!();
        }
    }

    #[test]
    fn test_url_host_port() {
        let url = "http://example.com:8888";
        let url = Url::try_from(url);

        if let Ok(url) = url {
            assert_eq!(url.host(), "example.com");
            assert_eq!(url.port(), 8888);
            assert_eq!(url.path(), None);
            assert_eq!(*url.searchpart(), BTreeMap::default());
        } else {
            panic!();
        }
    }

    #[test]
    fn test_url_host_port_path() {
        let url = "http://example.com:8888/index.html";
        let url = Url::try_from(url);

        if let Ok(url) = url {
            assert_eq!(url.host(), "example.com");
            assert_eq!(url.port(), 8888);
            assert_eq!(url.path(), Some("index.html"));
            assert_eq!(*url.searchpart(), BTreeMap::default());
        } else {
            panic!();
        }
    }

    #[test]
    fn test_url_host_path() {
        let url = "http://example.com/index.html";
        let url = Url::try_from(url);

        if let Ok(url) = url {
            assert_eq!(url.host(), "example.com");
            assert_eq!(url.port(), 80);
            assert_eq!(url.path(), Some("index.html"));
            assert_eq!(*url.searchpart(), BTreeMap::default());
        } else {
            panic!();
        }
    }

    #[test]
    fn test_url_host_port_path_searchpoint() {
        let url = "http://example.com:8888/index.html?a=123&b=456";
        let url = Url::try_from(url);

        if let Ok(url) = url {
            assert_eq!(url.host(), "example.com");
            assert_eq!(url.port(), 8888);
            assert_eq!(url.path(), Some("index.html"));

            let mut other = BTreeMap::default();
            other.insert("a".to_string(), "123".to_string());
            other.insert("b".to_string(), "456".to_string());
            assert_eq!(*url.searchpart(), other);
        } else {
            panic!();
        }
    }

    #[test]
    fn test_no_scheme() {
        let url = "example.com";
        let url = Url::try_from(url);

        assert!(url.is_err());
    }

    #[test]
    fn test_unsupported_scheme() {
        let url = "https://example.com:8888/index.html";
        let url = Url::try_from(url);

        assert!(url.is_err());
    }
}
