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

//! # IPinfo: The rust library to lookup IP address information
//!
//! This is the Rust client library for the [IPinfo.io](https://ipinfo.io) IP address API.
//! It allows you to lookup your own IP address, or get any of the following details for an IP:
//!
//! - IP geolocation (city, region, country, postal code, latitude and longitude)
//! - ASN details (ISP or network operator, associated domain name, and type, such as business, hosting or company)
//! - Company details (the name and domain of the business that uses the IP address)
//! - Carrier details (the name of the mobile carrier and MNC and MCC for that carrier if the IP is used exclusively for mobile traffic)
//!
//! ## Features
//!
//! * Smart LRU cache for cost and quota savings.
//! * Structured and type checked query results.
//! * Bulk IP address lookup using IPinfo batch API.
//! ## Example
//!
//! ```no_run
//! use ipinfo::{IpInfo, IpInfoConfig};
//! #[tokio::main]
//! async fn main() {
//!   // Setup token and other configurations.
//!   let config = IpInfoConfig { token: Some("my token".to_string()), ..Default::default() };
//!
//!   // Setup IpInfo structure and start looking up IP addresses.
//!   let mut ipinfo = IpInfo::new(config).expect("should construct");
//!   let res = ipinfo.lookup("8.8.8.8").await;
//!
//!   match res {
//!     Ok(r) => println!("{}: {}", "8.8.8.8", r.hostname.as_ref().unwrap()),
//!     Err(e) => println!("error occurred: {}", &e.to_string()),
//!   }
//! }
//! ```

/// Get crate version from cargo at build time.
const VERSION: &str = env!("CARGO_PKG_VERSION");

#[macro_use]
mod error;
mod api;
mod ipinfo;
mod util;
mod data;

pub use crate::ipinfo::*;
pub use api::*;
pub use error::*;
pub use util::*;
pub use data::*;
