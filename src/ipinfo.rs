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

use std::{collections::HashMap, fs, num::NonZeroUsize, time::Duration};

use crate::{
    cache_key, is_bogon, Continent, CountryCurrency, CountryFlag, IpDetails,
    IpError, BATCH_MAX_SIZE, BATCH_REQ_TIMEOUT_DEFAULT, VERSION,
    COUNTRIES, EU, FLAGS, CONTINENTS, CURRENCIES,
};

use lru::LruCache;
use serde_json::json;

use reqwest::header::{
    HeaderMap, HeaderValue, ACCEPT, CONTENT_TYPE, USER_AGENT,
};

use tokio::time::timeout;

const COUNTRY_FLAG_URL : &str= "https://cdn.ipinfo.io/static/images/countries-flags/";
/// IpInfo structure configuration.
pub struct IpInfoConfig {
    /// IPinfo access token.
    pub token: Option<String>,

    /// The timeout of HTTP requests. (default: 3 seconds)
    pub timeout: Duration,

    /// The size of the LRU cache. (default: 100 IPs)
    pub cache_size: usize,

    /// The file path of `countries.json`
    pub countries_file_path: Option<String>,

    /// The file path of `eu.json`
    pub eu_file_path: Option<String>,

    /// The file path of `flags.json`
    pub country_flags_file_path: Option<String>,

    /// The file path of `currencies.json`
    pub country_currencies_file_path: Option<String>,

    /// The file path of `continents.json`
    pub continents_file_path: Option<String>,
}

impl Default for IpInfoConfig {
    fn default() -> Self {
        Self {
            token: None,
            timeout: Duration::from_secs(3),
            cache_size: 100,
            countries_file_path: None,
            eu_file_path: None,
            country_flags_file_path: None,
            country_currencies_file_path: None,
            continents_file_path: None,
        }
    }
}

/// IPinfo requests context structure.
pub struct IpInfo {
    url: String,
    token: Option<String>,
    client: reqwest::Client,
    cache: LruCache<String, IpDetails>,
    countries: HashMap<String, String>,
    eu: Vec<String>,
    country_flags: HashMap<String, CountryFlag>,
    country_currencies: HashMap<String, CountryCurrency>,
    continents: HashMap<String, Continent>,
}

pub struct BatchReqOpts {
    batch_size: u64,
    timeout_per_batch: Duration,
    timeout_total: Option<Duration>,
}

impl Default for BatchReqOpts {
    fn default() -> Self {
        Self {
            batch_size: BATCH_MAX_SIZE,
            timeout_per_batch: BATCH_REQ_TIMEOUT_DEFAULT,
            timeout_total: None,
        }
    }
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

        let mut ipinfo_obj = Self {
            url,
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

        if config.countries_file_path.is_none() {
            ipinfo_obj.countries = COUNTRIES.clone();
        } else {
            let t_file =
                fs::File::open(config.countries_file_path.as_ref().unwrap())
                    .expect("error opening file");
            ipinfo_obj.countries =
                serde_json::from_reader(t_file).expect("error parsing JSON!");
        }

        if config.eu_file_path.is_none() {
            ipinfo_obj.eu = EU.clone();
        } else {
            let t_file = fs::File::open(config.eu_file_path.as_ref().unwrap())
                .expect("error opening file");
            ipinfo_obj.eu =
                serde_json::from_reader(t_file).expect("error parsing JSON!");
        }

        if config.country_flags_file_path.is_none() {
            ipinfo_obj.country_flags = FLAGS.clone();
        } else {
            let t_file = fs::File::open(
                config.country_flags_file_path.as_ref().unwrap(),
            )
            .expect("error opening file");
            ipinfo_obj.country_flags =
                serde_json::from_reader(t_file).expect("error parsing JSON!");
        }

        if config.country_currencies_file_path.is_none() {
            ipinfo_obj.country_currencies = CURRENCIES.clone();
        } else {
            let t_file = fs::File::open(
                config.country_currencies_file_path.as_ref().unwrap(),
            )
            .expect("error opening file");
            ipinfo_obj.country_currencies =
                serde_json::from_reader(t_file).expect("error parsing JSON!");
        }

        if config.continents_file_path.is_none() {
            ipinfo_obj.continents = CONTINENTS.clone();
        } else {
            let t_file =
                fs::File::open(config.continents_file_path.as_ref().unwrap())
                    .expect("error opening file");
            ipinfo_obj.continents =
                serde_json::from_reader(t_file).expect("error parsing JSON!");
        }

        Ok(ipinfo_obj)
    }

    /// Lookup IPDetails for a list of one or more IP addresses.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ipinfo::{IpInfo, BatchReqOpts};
    /// #[tokio::main]
    /// async fn main() {
    ///     let mut ipinfo = IpInfo::new(Default::default()).expect("should construct");
    ///     let res = ipinfo.lookup_batch(&["8.8.8.8"], BatchReqOpts::default()).await.expect("should run");
    /// }
    /// ```
    pub async fn lookup_batch(
        &mut self,
        ips: &[&str],
        batch_config: BatchReqOpts,
    ) -> Result<HashMap<String, IpDetails>, IpError> {
        let mut hits: Vec<IpDetails> = vec![];
        let mut misses: Vec<&str> = vec![];
        let mut to_lookup: Vec<&str> = vec![];
        let mut details_bogon = vec![];
        let mut results: HashMap<String, IpDetails> = HashMap::new();

        // Check for cache hits
        ips.iter()
            .for_each(|x| match self.cache.get(&cache_key(x)) {
                Some(detail) => hits.push(detail.clone()),
                None => misses.push(*x),
            });

        // check for bogon ip addresses
        for ip_address in misses.iter() {
            match is_bogon(ip_address) {
                true => details_bogon.push(IpDetails {
                    ip: ip_address.to_string(),
                    bogon: Some(true),
                    ..Default::default()
                }),
                false => to_lookup.push(ip_address),
            }
        }

        let client = reqwest::Client::builder()
            .timeout(batch_config.timeout_per_batch)
            .build()?;
        for i in (0..to_lookup.len()).step_by(batch_config.batch_size as usize)
        {
            let mut end = i + batch_config.batch_size as usize;
            if end > to_lookup.len() {
                end = to_lookup.len();
            }

            let lookup_list = &to_lookup[i..end];

            if let Some(total_timeout) = batch_config.timeout_total {
                match timeout(
                    total_timeout,
                    self._lookup_batch(client.clone(), lookup_list),
                )
                .await
                {
                    Ok(result) => match result {
                        Ok(details) => results.extend(details),
                        Err(_) => return Err(err!(IpRequestError)),
                    },
                    Err(_) => return Err(err!(TimeOutError)),
                }
            } else {
                match self._lookup_batch(client.clone(), lookup_list).await {
                    Ok(result) => results.extend(result),
                    Err(_) => return Err(err!(IpRequestError)),
                }
            }
        }

        // Add country_name and EU status to response
        for detail in results.to_owned() {
            let mut mut_details = results.get_mut(&detail.0).unwrap();
            self.populate_static_details(&mut mut_details);
        }

        // Add Bogon IP Results
        for result in details_bogon {
            results.insert(result.ip.clone(), result);
        }

        // Update cache
        results.iter().for_each(|x| {
            self.cache.put(cache_key(x.0.as_str()), x.1.clone());
        });

        // Add cache hits to the result
        hits.iter().for_each(|x| {
            results.insert(x.ip.clone(), x.clone());
        });

        Ok(results)
    }

    async fn _lookup_batch(
        self: &Self,
        client: reqwest::Client,
        ips: &[&str],
    ) -> Result<HashMap<String, IpDetails>, IpError> {
        // Lookup cache misses which are not bogon
        let response = client
            .post(&format!("{}/batch", self.url))
            .headers(Self::construct_headers())
            .bearer_auth(&self.token.as_ref().unwrap_or(&"".to_string()))
            .json(&json!(ips))
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

        // Parse the results
        let result: HashMap<String, IpDetails> =
            serde_json::from_str(&raw_resp)?;
        return Ok(result);
    }

    /// looks up IPDetails for a single IP Address
    ///
    /// # Example
    ///
    /// ```no_run
    /// use ipinfo::IpInfo;
    ///
    ///  #[tokio::main]
    /// async fn main() {
    ///     let mut ipinfo = IpInfo::new(Default::default()).expect("should construct");
    ///     let res = ipinfo.lookup("8.8.8.8").await.expect("should run");
    /// }
    /// ```
    pub async fn lookup(&mut self, ip: &str) -> Result<IpDetails, IpError> {
        if is_bogon(&ip.to_string()) {
            return Ok(IpDetails {
                ip: ip.to_string(),
                bogon: Some(true),
                ..Default::default() // fill remaining with default values
            });
        }

        // Check for cache hit
        let cached_detail = self.cache.get(&cache_key(ip));

        if !cached_detail.is_none() {
            return Ok(cached_detail.unwrap().clone());
        }

        // lookup in case of a cache miss
        let response = self
            .client
            .get(&format!("{}/{}", self.url, ip))
            .headers(Self::construct_headers())
            .bearer_auth(&self.token.as_ref().unwrap_or(&"".to_string()))
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
        let mut details: IpDetails = serde_json::from_str(&raw_resp)?;
        self.populate_static_details(&mut details);

        // update cache
        self.cache.put(cache_key(ip), details.clone());
        Ok(details)
    }

    /// Get a mapping of a list of IPs on a world map
    ///
    /// # Example
    ///
    /// ```no_run
    /// use ipinfo::IpInfo;
    ///
    ///  #[tokio::main]
    /// async fn main() {
    ///     let ipinfo = IpInfo::new(Default::default()).expect("should construct");
    ///     let map_url = ipinfo.get_map(&["8.8.8.8", "4.2.2.4"]).await.expect("should run");
    /// }
    /// ```
    pub async fn get_map(&self, ips: &[&str]) -> Result<String, IpError> {
        if ips.len() > 500_000 {
            return Err(err!(MapLimitError));
        }

        let map_url = &format!("{}/tools/map?cli=1", self.url);
        let client = self.client.clone();
        let json_ips = serde_json::json!(ips);

        let response = client.post(map_url).json(&json_ips).send().await?;
        if !response.status().is_success() {
            return Err(err!(HTTPClientError));
        }

        let response_json: serde_json::Value = response.json().await?;
        let report_url = response_json["reportUrl"]
            .as_str()
            .ok_or("Report URL not found");
        Ok(report_url.unwrap().to_string())
    }

    // Add country details and EU status to response
    fn populate_static_details(&self, details: &mut IpDetails) {
        if !&details.country.is_empty() {
            let country_name = self.countries.get(&details.country).unwrap();
            details.country_name = Some(country_name.to_string());
            details.is_eu = Some(self.eu.contains(&details.country));
            let country_flag =
                self.country_flags.get(&details.country).unwrap();
            details.country_flag = Some(country_flag.to_owned());
            let file_ext = ".svg";
            details.country_flag_url = Some(COUNTRY_FLAG_URL.to_string() + &details.country + file_ext);
            let country_currency =
                self.country_currencies.get(&details.country).unwrap();
            details.country_currency = Some(country_currency.to_owned());
            let continent = self.continents.get(&details.country).unwrap();
            details.continent = Some(continent.to_owned());
        }
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
        return IpInfo::new(IpInfoConfig {
            token: Some(env::var("IPINFO_TOKEN").unwrap().to_string()),
            timeout: Duration::from_secs(3),
            cache_size: 100,
            ..Default::default()
        })
        .expect("should construct");
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

    #[tokio::test]
    async fn request_single_ip() {
        let mut ipinfo = get_ipinfo_client();

        let details =
            ipinfo.lookup("66.87.125.72").await.expect("should lookup");

        assert_eq!(details.ip, "66.87.125.72");
    }

    #[tokio::test]
    async fn request_no_token() {
        let mut ipinfo =
            IpInfo::new(Default::default()).expect("should construct");

        assert_eq!(
            ipinfo
                .lookup_batch(&["8.8.8.8"], BatchReqOpts::default())
                .await
                .err()
                .unwrap()
                .kind(),
            IpErrorKind::IpRequestError
        );
    }

    #[tokio::test]
    async fn request_multiple_ip() {
        let mut ipinfo = get_ipinfo_client();

        let details = ipinfo
            .lookup_batch(&["8.8.8.8", "4.2.2.4"], BatchReqOpts::default())
            .await
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
        assert_eq!(ip8.country_flag_url, Some("https://cdn.ipinfo.io/static/images/countries-flags/US.svg".to_owned()));
        assert_eq!(
            ip8.country_flag,
            Some(CountryFlag {
                emoji: "🇺🇸".to_owned(),
                unicode: "U+1F1FA U+1F1F8".to_owned()
            })
        );
        assert_eq!(
            ip8.country_currency,
            Some(CountryCurrency {
                code: "USD".to_owned(),
                symbol: "$".to_owned()
            })
        );
        assert_eq!(
            ip8.continent,
            Some(Continent {
                code: "NA".to_owned(),
                name: "North America".to_owned()
            })
        );
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

    #[tokio::test]
    async fn request_cache_miss_and_hit() {
        let mut ipinfo = get_ipinfo_client();

        // Populate the cache with 8.8.8.8
        let details = ipinfo
            .lookup_batch(&["8.8.8.8"], BatchReqOpts::default())
            .await
            .expect("should lookup");

        // Assert 1 result
        assert!(details.contains_key("8.8.8.8"));
        assert_eq!(details.len(), 1);

        // Should have a cache hit for 8.8.8.8 and query for 4.2.2.4
        let details = ipinfo
            .lookup_batch(&["4.2.2.4", "8.8.8.8"], BatchReqOpts::default())
            .await
            .expect("should lookup");

        // Assert 2 results
        assert!(details.contains_key("8.8.8.8"));
        assert!(details.contains_key("4.2.2.4"));
        assert_eq!(details.len(), 2);
    }

    #[test]
    fn test_is_bogon() {
        assert_eq!(true, is_bogon("169.254.0.1"));
        assert_eq!(true, is_bogon("192.0.2.1"));
        assert_eq!(false, is_bogon("8.8.8.8"));
        assert_eq!(true, is_bogon("2001:db8::1"));
        assert_eq!(false, is_bogon("2606:4700:4700:1111::2"));
    }
}
