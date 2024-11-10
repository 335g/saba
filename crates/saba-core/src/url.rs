use alloc::{
    string::{String, ToString},
    vec::Vec,
};
use core::result::Result;

use crate::error::Error;

#[derive(Debug, Clone, PartialEq)]
pub struct Url {
    host: String,
    port: String,
    path: Option<String>,
    searchpart: Option<String>,
}

impl TryFrom<String> for Url {
    type Error = Error;

    fn try_from(url: String) -> Result<Self, Self::Error> {
        if !url.starts_with("http://") {
            return Err(Error::InvalidUrl(url.clone()));
        }

        let host = extract_host(&url);
        let port = extract_port(&url);
        let path = extract_path(&url);
        let searchpart = extract_searchpart(&url);

        Ok(Self {
            host,
            port,
            path,
            searchpart,
        })
    }
}

impl Url {
    pub fn host(&self) -> &str {
        &self.host
    }

    pub fn port(&self) -> &str {
        &self.port
    }

    pub fn path(&self) -> Option<&str> {
        self.path.as_ref().map(|s| s.as_ref())
    }

    pub fn searchpart(&self) -> Option<&str> {
        self.searchpart.as_ref().map(|s| s.as_ref())
    }

    pub fn is_http(&self) -> bool {
        true
    }
}

fn url_parts(url: &str) -> Vec<&str> {
    url.trim_start_matches("http://").splitn(2, '/').collect()
}

fn extract_host(url: &str) -> String {
    let parts = url_parts(url);

    parts[0]
        .find(':')
        .map(|index| parts[0][..index].to_string())
        .unwrap_or(parts[0].to_string())
}

fn extract_port(url: &str) -> String {
    let parts = url_parts(url);

    parts[0]
        .find(':')
        .map(|index| parts[0][index + 1..].to_string())
        .unwrap_or("80".to_string())
}

fn extract_path(url: &str) -> Option<String> {
    let parts = url_parts(url);

    parts
        .get(1)
        .map(|x| x.split('?').next().expect("is path").to_string())
}

fn extract_searchpart(url: &str) -> Option<String> {
    let parts = url_parts(url);

    parts.get(1).and_then(|x| {
        x.split('?')
            .collect::<Vec<_>>()
            .get(1)
            .map(|s| s.to_string())
    })
}
