//   Copyright 2019 IPinfo library developers
//
//   Licensed under the Apache License, Version 2.0 (the "License");
//   you may not use this file except in compliance with the License.
//   You may obtain a copy of the License at
//
//       http://www.apache.org/licenses/LICENSE-2.0
//
//   Unless required by applicable law or agreed to in writing, software
//   distributed under the License is distributed on an "AS IS" BASIS,
//   WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//   See the License for the specific language governing permissions and
//   limitations under the License.

use std::time::Duration;

use crate::{
    VERSION,
    IpError,
    IpDetails,
};

use serde_json::json;
use lru::LruCache;

use reqwest::header::{
    HeaderMap,
    HeaderValue,
    USER_AGENT,
    CONTENT_TYPE,
    ACCEPT,
};

/// IpInfo structure configuration.
pub struct IpInfoConfig {
    /// IPinfo access token.
    pub token: Option<String>,

    /// The timeout of HTTP requests.
    pub timeout: Duration,

    /// The size of the LRU cache.
    pub cache_size: usize,
}

impl Default for IpInfoConfig {
    fn default() -> Self {
        Self {
            token: None,
            timeout: Duration::from_secs(3),
            cache_size: 100,
        }
    }
}

/// IPinfo requests context structure.
pub struct IpInfo {
    token: Option<String>,
    client: reqwest::Client,
    cache: LruCache<String, IpDetails>,
}

impl IpInfo {
    /// Construct a new IpInfo structure.
    ///
    /// # Examples
    ///
    /// ```
    /// use ipinfo::IpInfo;
    ///
    /// let ipinfo = IpInfo::new(Default::default());
    /// ```
    pub fn new(config: IpInfoConfig) -> Result<Self, IpError> {
        let client = reqwest::Client::builder()
            .timeout(config.timeout)
            .build()?;

        Ok(Self {
            token: config.token,
            client: client,
            cache: LruCache::new(config.cache_size),
        })
    }

    /// Lookup a list of IP addresses.
    pub fn lookup(&mut self, ips: &[&str]) -> Result<Vec<IpDetails>, IpError> {
        let mut hits: Vec<IpDetails> = vec![];
        let mut misses: Vec<&str> = vec![];

        // Check for cache hits
        ips.iter().for_each(|x| {
            if let Some(detail) = self.cache.get(&x.to_string()) {
                hits.push(detail.clone());
            } else {
                misses.push(*x);
            }
        });

        // Lookup cache misses
        let response = self.client.post(
            &format!("https://ipinfo.io/batch?token={}", self.token.as_ref().unwrap_or(&"".to_owned())))
            .headers(Self::construct_headers())
            .json(&json!(misses))
            .send()?;

        match response.status() {
            reqwest::StatusCode::TOO_MANY_REQUESTS => Err(err!(RateLimitExceededError)),
            _ => {
                let resp: Vec<IpDetails> = response.error_for_status()?.json()?;
                resp.iter().for_each(|x| { self.cache.put(x.ip.clone(), x.clone()); });
                Ok(resp)
            },
        }
    }

    /// Construct API request headers.
    fn construct_headers() -> HeaderMap {
        let mut headers = HeaderMap::new();
        headers.insert(USER_AGENT, HeaderValue::from_str(&format!("IPinfoClient/Rust/{}", VERSION)).unwrap());
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        headers.insert(ACCEPT, HeaderValue::from_static("application/json"));
        headers
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ipinfo_config_defaults_reasonable() {
        let ipinfo_config = IpInfoConfig::default();

        assert_eq!(ipinfo_config.timeout, Duration::from_secs(3));
        assert_eq!(ipinfo_config.cache_size, 100);
    }

    #[test]
    fn request_headers_are_canonical() {
        let headers = IpInfo::construct_headers();

        assert_eq!(headers[USER_AGENT], format!("IPinfoClient/Rust/{}", VERSION));
        assert_eq!(headers[CONTENT_TYPE], "application/json");
        assert_eq!(headers[ACCEPT], "application/json");
    }
}
