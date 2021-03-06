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

use std::{collections::HashMap, time::Duration};

use crate::{IpDetails, IpError, VERSION};

use lru::LruCache;
use serde_json::json;

use reqwest::header::{
    HeaderMap, HeaderValue, ACCEPT, CONTENT_TYPE, USER_AGENT,
};

/// IpInfo structure configuration.
pub struct IpInfoConfig {
    /// IPinfo access token.
    pub token: Option<String>,

    /// The timeout of HTTP requests. (default: 3 seconds)
    pub timeout: Duration,

    /// The size of the LRU cache. (default: 100 IPs)
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
    url: String,
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
    /// let ipinfo = IpInfo::new(Default::default()).expect("should construct");
    /// ```
    pub fn new(config: IpInfoConfig) -> Result<Self, IpError> {
        let client =
            reqwest::Client::builder().timeout(config.timeout).build()?;

        let url = "https://ipinfo.io".to_owned();

        Ok(Self {
            url,
            client,
            token: config.token,
            cache: LruCache::new(config.cache_size),
        })
    }

    /// Lookup a list of one or more IP addresses.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ipinfo::IpInfo;
    ///
    /// let mut ipinfo = IpInfo::new(Default::default()).expect("should construct");
    /// let res = ipinfo.lookup(&["8.8.8.8"]).expect("should run");
    /// ```
    pub fn lookup(
        &mut self,
        ips: &[&str],
    ) -> Result<HashMap<String, IpDetails>, IpError> {
        let mut hits: Vec<IpDetails> = vec![];
        let mut misses: Vec<&str> = vec![];

        // Check for cache hits
        ips.iter()
            .for_each(|x| match self.cache.get(&x.to_string()) {
                Some(detail) => hits.push(detail.clone()),
                None => misses.push(*x),
            });

        // Lookup cache misses
        let response = self
            .client
            .post(&format!("{}/batch", self.url))
            .headers(Self::construct_headers())
            .bearer_auth(&self.token.as_ref().unwrap_or(&"".to_string()))
            .json(&json!(misses))
            .send()?;

        // Check if we exhausted our request quota
        if let reqwest::StatusCode::TOO_MANY_REQUESTS = response.status() {
            return Err(err!(RateLimitExceededError));
        }

        // Acquire response
        let raw_resp = response.error_for_status()?.text()?;

        // Parse the response
        let resp: serde_json::Value = serde_json::from_str(&raw_resp)?;

        // Return if an error occurred
        if let Some(e) = resp["error"].as_str() {
            return Err(err!(IpRequestError, e));
        }

        // Parse the results
        let mut details: HashMap<String, IpDetails> =
            serde_json::from_str(&raw_resp)?;

        // Update cache
        details.iter().for_each(|x| {
            self.cache.put(x.0.clone(), x.1.clone());
        });

        // Add cache hits to the result
        hits.iter().for_each(|x| {
            details.insert(x.ip.clone(), x.clone());
        });

        Ok(details)
    }

    /// Construct API request headers.
    fn construct_headers() -> HeaderMap {
        let mut headers = HeaderMap::new();
        headers.insert(
            USER_AGENT,
            HeaderValue::from_str(&format!("IPinfoClient/Rust/{}", VERSION))
                .unwrap(),
        );
        headers.insert(
            CONTENT_TYPE,
            HeaderValue::from_static("application/json"),
        );
        headers.insert(ACCEPT, HeaderValue::from_static("application/json"));
        headers
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::IpErrorKind;
    use std::env;

    fn get_ipinfo_client() -> IpInfo {
        return IpInfo::new(IpInfoConfig{
            token: Some(env::var("IPINFO_TOKEN").unwrap().to_string()),
            timeout: Duration::from_secs(3),
            cache_size: 100,
        }).expect("should construct");
    }

    #[test]
    fn ipinfo_config_defaults_reasonable() {
        let ipinfo_config = IpInfoConfig::default();

        assert_eq!(ipinfo_config.timeout, Duration::from_secs(3));
        assert_eq!(ipinfo_config.cache_size, 100);
    }

    #[test]
    fn request_headers_are_canonical() {
        let headers = IpInfo::construct_headers();

        assert_eq!(
            headers[USER_AGENT],
            format!("IPinfoClient/Rust/{}", VERSION)
        );
        assert_eq!(headers[CONTENT_TYPE], "application/json");
        assert_eq!(headers[ACCEPT], "application/json");
    }

    #[test]
    fn request_single_ip() {
        let mut ipinfo = get_ipinfo_client();

        let details = ipinfo
            .lookup(&["66.87.125.72"])
            .expect("should lookup");

        assert!(details.contains_key("66.87.125.72"));
        assert_eq!(details.len(), 1);
    }

    #[test]
    fn request_single_ip_no_token() {
        let mut ipinfo =
            IpInfo::new(Default::default()).expect("should construct");

        assert_eq!(
            ipinfo.lookup(&["8.8.8.8"]).err().unwrap().kind(),
            IpErrorKind::IpRequestError
        );
    }

    #[test]
    fn request_multiple_ip() {
        let mut ipinfo = get_ipinfo_client();

        let details = ipinfo
            .lookup(&["8.8.8.8", "4.2.2.4"])
            .expect("should lookup");

        // Assert successful lookup
        assert!(details.contains_key("8.8.8.8"));
        assert!(details.contains_key("4.2.2.4"));

        // Assert 8.8.8.8
        let ip8 = &details["8.8.8.8"];
        assert_eq!(ip8.ip, "8.8.8.8");
        assert_eq!(ip8.hostname, Some("dns.google".to_owned()));
        assert_eq!(ip8.city, "Mountain View");
        assert_eq!(ip8.region, "California");
        assert_eq!(ip8.country, "US");
        assert_eq!(ip8.loc, "37.4056,-122.0775");
        assert_eq!(ip8.postal, Some("94043".to_owned()));
        assert_eq!(ip8.timezone, Some("America/Los_Angeles".to_owned()));

        // Assert 4.2.2.4
        let ip4 = &details["4.2.2.4"];
        assert_eq!(ip4.ip, "4.2.2.4");
        assert_eq!(ip4.hostname, Some("d.resolvers.level3.net".to_owned()));
        assert_eq!(ip4.city, "Monroe");
        assert_eq!(ip4.region, "Louisiana");
        assert_eq!(ip4.country, "US");
        assert_eq!(ip4.loc, "32.5530,-92.0422");
        assert_eq!(ip4.postal, Some("71203".to_owned()));
        assert_eq!(ip4.timezone, Some("America/Chicago".to_owned()));
    }

    #[test]
    fn request_cache_miss_and_hit() {
        let mut ipinfo = get_ipinfo_client();

        // Populate the cache with 8.8.8.8
        let details = ipinfo.lookup(&["8.8.8.8"]).expect("should lookup");

        // Assert 1 result
        assert!(details.contains_key("8.8.8.8"));
        assert_eq!(details.len(), 1);

        // Should have a cache hit for 8.8.8.8 and query for 4.2.2.4
        let details = ipinfo
            .lookup(&["4.2.2.4", "8.8.8.8"])
            .expect("should lookup");

        // Assert 2 results
        assert!(details.contains_key("8.8.8.8"));
        assert!(details.contains_key("4.2.2.4"));
        assert_eq!(details.len(), 2);
    }
}
