//   Copyright 2019-2025 IPinfo library developers
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

use std::{collections::HashMap, num::NonZeroUsize, time::Duration};

use crate::{
    cache_key, is_bogon, Continent, CountryCurrency, CountryFlag,
    IpDetailsLite, IpError, CONTINENTS, COUNTRIES, CURRENCIES, EU, FLAGS,
    VERSION,
};

use lru::LruCache;

use reqwest::header::{
    HeaderMap, HeaderValue, ACCEPT, CONTENT_TYPE, USER_AGENT,
};

const COUNTRY_FLAG_URL: &str =
    "https://cdn.ipinfo.io/static/images/countries-flags/";
const BASE_URL: &str = "https://api.ipinfo.io/lite";
const BASE_URL_V6: &str = "https://v6.api.ipinfo.io/lite";

/// IpInfoLite structure configuration.
pub struct IpInfoLiteConfig {
    /// IPinfo access token.
    pub token: Option<String>,

    /// The timeout of HTTP requests. (default: 3 seconds)
    pub timeout: Duration,

    /// The size of the LRU cache. (default: 100 IPs)
    pub cache_size: usize,

    // Default mapping of country codes to country names
    pub defaut_countries: Option<HashMap<String, String>>,

    // Default list of EU countries
    pub default_eu: Option<Vec<String>>,

    // Default mapping of country codes to their respective flag emoji and unicode
    pub default_flags: Option<HashMap<String, CountryFlag>>,

    // Default mapping of currencies to their respective currency code and symbol
    pub default_currencies: Option<HashMap<String, CountryCurrency>>,

    // Default mapping of country codes to their respective continent code and name
    pub default_continents: Option<HashMap<String, Continent>>,
}

impl Default for IpInfoLiteConfig {
    fn default() -> Self {
        Self {
            token: None,
            timeout: Duration::from_secs(3),
            cache_size: 100,
            defaut_countries: None,
            default_eu: None,
            default_flags: None,
            default_currencies: None,
            default_continents: None,
        }
    }
}

/// IpInfoLite requests context structure.
pub struct IpInfoLite {
    token: Option<String>,
    client: reqwest::Client,
    cache: LruCache<String, IpDetailsLite>,
    countries: HashMap<String, String>,
    eu: Vec<String>,
    country_flags: HashMap<String, CountryFlag>,
    country_currencies: HashMap<String, CountryCurrency>,
    continents: HashMap<String, Continent>,
}

impl IpInfoLite {
    /// Construct a new IpInfoLite structure.
    ///
    /// # Examples
    ///
    /// ```
    /// use ipinfo::IpInfoLite;
    ///
    /// let ipinfo = IpInfoLite::new(Default::default()).expect("should construct");
    /// ```
    pub fn new(config: IpInfoLiteConfig) -> Result<Self, IpError> {
        let client =
            reqwest::Client::builder().timeout(config.timeout).build()?;

        let mut ipinfo_obj = Self {
            client,
            token: config.token,
            cache: LruCache::new(
                NonZeroUsize::new(config.cache_size).unwrap(),
            ),
            countries: HashMap::new(),
            eu: Vec::new(),
            country_flags: HashMap::new(),
            country_currencies: HashMap::new(),
            continents: HashMap::new(),
        };

        if config.defaut_countries.is_none() {
            ipinfo_obj.countries = COUNTRIES.clone();
        } else {
            ipinfo_obj.countries = config.defaut_countries.unwrap();
        }

        if config.default_eu.is_none() {
            ipinfo_obj.eu = EU.clone();
        } else {
            ipinfo_obj.eu = config.default_eu.unwrap();
        }

        if config.default_flags.is_none() {
            ipinfo_obj.country_flags = FLAGS.clone();
        } else {
            ipinfo_obj.country_flags = config.default_flags.unwrap();
        }

        if config.default_currencies.is_none() {
            ipinfo_obj.country_currencies = CURRENCIES.clone();
        } else {
            ipinfo_obj.country_currencies = config.default_currencies.unwrap();
        }

        if config.default_continents.is_none() {
            ipinfo_obj.continents = CONTINENTS.clone();
        } else {
            ipinfo_obj.continents = config.default_continents.unwrap();
        }

        Ok(ipinfo_obj)
    }

    /// looks up IpDetailsLite for a single IP Address
    ///
    /// # Example
    ///
    /// ```no_run
    /// use ipinfo::IpInfoLite;
    ///
    ///  #[tokio::main]
    /// async fn main() {
    ///     let mut ipinfo = IpInfoLite::new(Default::default()).expect("should construct");
    ///     let res = ipinfo.lookup("8.8.8.8").await.expect("should run");
    /// }
    /// ```
    pub async fn lookup(
        &mut self,
        ip: &str,
    ) -> Result<IpDetailsLite, IpError> {
        self._lookup(ip, BASE_URL).await
    }

    /// looks up IPDetailsLite of your own v4 IP
    ///
    /// # Example
    ///
    /// ```no_run
    /// use ipinfo::IpInfoLite;
    ///
    ///  #[tokio::main]
    /// async fn main() {
    ///     let mut ipinfo = IpInfoLite::new(Default::default()).expect("should construct");
    ///     let res = ipinfo.lookup_self_v4().await.expect("should run");
    /// }
    /// ```
    pub async fn lookup_self_v4(&mut self) -> Result<IpDetailsLite, IpError> {
        self._lookup("me", BASE_URL).await
    }

    /// looks up IPDetailsLite of your own v6 IP
    ///
    /// # Example
    ///
    /// ```no_run
    /// use ipinfo::IpInfoLite;
    ///
    ///  #[tokio::main]
    /// async fn main() {
    ///     let mut ipinfo = IpInfoLite::new(Default::default()).expect("should construct");
    ///     let res = ipinfo.lookup_self_v6().await.expect("should run");
    /// }
    /// ```
    pub async fn lookup_self_v6(&mut self) -> Result<IpDetailsLite, IpError> {
        self._lookup("me", BASE_URL_V6).await
    }

    async fn _lookup(
        &mut self,
        ip: &str,
        base_url: &str,
    ) -> Result<IpDetailsLite, IpError> {
        if is_bogon(ip) {
            return Ok(IpDetailsLite {
                ip: ip.to_string(),
                bogon: Some(true),
                ..Default::default() // fill remaining with default values
            });
        }

        // Check for cache hit
        let cached_detail = self.cache.get(&cache_key(ip));

        if let Some(cached_detail) = cached_detail {
            return Ok(cached_detail.clone());
        }

        // lookup in case of a cache miss
        let response = self
            .client
            .get(format!("{base_url}/{ip}"))
            .headers(Self::construct_headers())
            .bearer_auth(self.token.as_deref().unwrap_or_default())
            .send()
            .await?;

        // Check if we exhausted our request quota
        if let reqwest::StatusCode::TOO_MANY_REQUESTS = response.status() {
            return Err(err!(RateLimitExceededError));
        }

        // Acquire response
        let raw_resp = response.error_for_status()?.text().await?;

        // Parse the response
        let resp: serde_json::Value = serde_json::from_str(&raw_resp)?;

        // Return if an error occurred
        if let Some(e) = resp["error"].as_str() {
            return Err(err!(IpRequestError, e));
        }

        // Parse the results and add additional country details
        let mut details: IpDetailsLite = serde_json::from_str(&raw_resp)?;
        self.populate_static_details(&mut details);

        // update cache
        self.cache.put(cache_key(ip), details.clone());
        Ok(details)
    }

    // Add country details and EU status to response
    fn populate_static_details(&self, details: &mut IpDetailsLite) {
        if !&details.country_code.is_empty() {
            let country_name =
                self.countries.get(&details.country_code).unwrap();
            details.country_name = country_name.to_owned();
            details.is_eu = self.eu.contains(&details.country_code);
            let country_flag =
                self.country_flags.get(&details.country_code).unwrap();
            details.country_flag = country_flag.to_owned();
            let file_ext = ".svg";
            details.country_flag_url = COUNTRY_FLAG_URL.to_string()
                + &details.country_code
                + file_ext;
            let country_currency =
                self.country_currencies.get(&details.country_code).unwrap();
            details.country_currency = country_currency.to_owned();
            let continent =
                self.continents.get(&details.country_code).unwrap();
            details.continent = continent.to_owned();
        }
    }

    /// Construct API request headers.
    fn construct_headers() -> HeaderMap {
        let mut headers = HeaderMap::new();
        headers.insert(
            USER_AGENT,
            HeaderValue::from_str(&format!("IPinfoClient/Rust/{VERSION}"))
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
    use crate::IpErrorKind::HTTPClientError;
    use std::env;

    fn get_ipinfo_client() -> IpInfoLite {
        IpInfoLite::new(IpInfoLiteConfig {
            token: Some(env::var("IPINFO_TOKEN").unwrap().to_string()),
            timeout: Duration::from_secs(3),
            cache_size: 100,
            ..Default::default()
        })
        .expect("should construct")
    }

    #[test]
    fn ipinfo_config_defaults_reasonable() {
        let ipinfo_config = IpInfoLiteConfig::default();

        assert_eq!(ipinfo_config.timeout, Duration::from_secs(3));
        assert_eq!(ipinfo_config.cache_size, 100);
    }

    #[test]
    fn request_headers_are_canonical() {
        let headers = IpInfoLite::construct_headers();

        assert_eq!(
            headers[USER_AGENT],
            format!("IPinfoClient/Rust/{}", VERSION)
        );
        assert_eq!(headers[CONTENT_TYPE], "application/json");
        assert_eq!(headers[ACCEPT], "application/json");
    }

    #[tokio::test]
    async fn lookup_no_token() {
        let mut ipinfo =
            IpInfoLite::new(Default::default()).expect("should construct");

        assert_eq!(
            ipinfo.lookup("8.8.8.8").await.err().unwrap().kind(),
            HTTPClientError
        );
    }

    #[tokio::test]
    async fn lookup_single_ip() {
        let mut ipinfo = get_ipinfo_client();

        let details = ipinfo.lookup("8.8.8.8").await.expect("should lookup");

        assert_eq!(details.ip, "8.8.8.8");
        assert_eq!(details.country_code, "US");
        assert_eq!(details.country, "United States");
        assert_eq!(details.country_name, "United States");
        assert_eq!(details.is_eu, false);
        assert_eq!(details.country_flag.emoji, "🇺🇸");
        assert_eq!(details.country_flag.unicode, "U+1F1FA U+1F1F8");
        assert_eq!(
            details.country_flag_url,
            "https://cdn.ipinfo.io/static/images/countries-flags/US.svg"
        );
        assert_eq!(details.country_currency.code, "USD");
        assert_eq!(details.country_currency.symbol, "$");

        assert_eq!(details.asn, "AS15169");
        assert_eq!(details.as_name, "Google LLC");
        assert_eq!(details.as_domain, "google.com");

        assert_eq!(details.continent.code, "NA");
        assert_eq!(details.continent.name, "North America");
    }

    #[tokio::test]
    async fn lookup_single_ip_v6() {
        let mut ipinfo = get_ipinfo_client();

        let details = ipinfo
            .lookup("2001:4860:4860::8888")
            .await
            .expect("should lookup");

        assert_eq!(details.ip, "2001:4860:4860::8888");
        assert_eq!(details.country_code, "US");
        assert_eq!(details.country, "United States");
        assert_eq!(details.country_name, "United States");
        assert_eq!(details.is_eu, false);
        assert_eq!(details.country_flag.emoji, "🇺🇸");
        assert_eq!(details.country_flag.unicode, "U+1F1FA U+1F1F8");
        assert_eq!(
            details.country_flag_url,
            "https://cdn.ipinfo.io/static/images/countries-flags/US.svg"
        );
        assert_eq!(details.country_currency.code, "USD");
        assert_eq!(details.country_currency.symbol, "$");

        assert_eq!(details.asn, "AS15169");
        assert_eq!(details.as_name, "Google LLC");
        assert_eq!(details.as_domain, "google.com");

        assert_eq!(details.continent.code, "NA");
        assert_eq!(details.continent.name, "North America");
    }

    #[tokio::test]
    async fn lookup_self_v4() {
        let mut ipinfo = get_ipinfo_client();

        let details = ipinfo.lookup_self_v4().await.expect("should lookup");

        // We can't know the values since they depend on the host IP
        assert_ne!(details.ip, "");
        assert_ne!(details.country_code, "");
        assert_ne!(details.country, "");
        assert_ne!(details.country_name, "");

        assert_ne!(details.country_flag.emoji, "");
        assert_ne!(details.country_flag.unicode, "");
        assert_ne!(details.country_flag_url, "");
        assert_ne!(details.country_currency.code, "");
        assert_ne!(details.country_currency.symbol, "");

        assert_ne!(details.asn, "");
        assert_ne!(details.as_name, "");
        assert_ne!(details.as_domain, "");

        assert_ne!(details.continent.code, "");
        assert_ne!(details.continent.name, "");
    }
}
