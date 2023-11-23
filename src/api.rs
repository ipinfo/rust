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

//! IPinfo API data structures.

use std::collections::HashMap;
use std::fmt;

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// IP address lookup details.
#[derive(Debug, Default, Deserialize, Serialize, Clone)]
pub struct IpDetails {
    /// The IP address.
    pub ip: String,

    /// The reverse DNS lookup hostname of the IP address.
    pub hostname: Option<String>,

    /// The city for the IP address.
    pub city: String,

    /// The region for the IP address.
    pub region: String,

    /// The country for the IP address.
    pub country: String,

    /// The countryname for the IP address.
    pub country_name: Option<String>,

    /// EU status of the country.
    pub is_eu: Option<bool>,

    /// Flag and unicode of the country.
    pub country_flag: Option<CountryFlag>,

    /// Link of the Flag of country.
    pub country_flag_url: Option<String>,

    /// Code and symbol of the country's currency.
    pub country_currency: Option<CountryCurrency>,

    /// Code and name of the continent.
    pub continent: Option<Continent>,

    /// The geographical location for the IP address.
    pub loc: String,

    /// The organization for the IP address.
    pub org: Option<String>,

    /// The postal code for the IP address.
    pub postal: Option<String>,

    /// The timezone for the IP address.
    pub timezone: Option<String>,

    /// The AS details the IP address is part of.
    pub asn: Option<AsnDetails>,

    /// The company details that owns this IP address.
    pub company: Option<CompanyDetails>,

    /// The carrier details that owns this mobile IP address.
    pub carrier: Option<CarrierDetails>,

    /// The privacy details for the IP address.
    pub privacy: Option<PrivacyDetails>,

    /// The abuse details for the IP address.
    pub abuse: Option<AbuseDetails>,

    /// The hosted domains details for the IP address.
    pub domains: Option<DomainsDetails>,

    /// If the IP Address is Bogon
    pub bogon: Option<bool>,

    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

/// ASN details.
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct AsnDetails {
    /// The AS number.
    pub asn: String,

    /// The name of the entity that owns this AS.
    pub name: String,

    /// The domain for the entity that owns this AS.
    pub domain: String,

    /// The route for this AS.
    pub route: String,

    /// The entity type that owns this AS. (i.e., business, education, hosting, isp)
    #[serde(rename = "type")]
    pub asn_type: String,
}

/// Company details.
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CompanyDetails {
    /// The name of the entity that owns the IP address.
    pub name: String,

    /// The domain for the entity that owns this IP address.
    pub domain: String,

    /// The type of entity that owns this IP address. (i.e., business, education, hosting, isp)
    #[serde(rename = "type")]
    pub company_type: String,
}

/// Mobile carrier details.
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CarrierDetails {
    /// The name of the carrier ISP that owns that mobile IP address.
    pub name: String,

    /// MCC GSM network code of this carrier.
    pub mcc: String,

    /// MNC GSM network code of this carrier.
    pub mnc: String,
}

/// Privacy details.
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct PrivacyDetails {
    /// Whether this IP address belongs to a VPN.
    pub vpn: bool,

    /// Whether this IP address belongs to a proxy.
    pub proxy: bool,

    /// Whether this IP address is using Tor.
    pub tor: bool,

    /// Whether this IP address is a relay.
    pub relay: bool,

    /// Whether this IP address is from a hosting provider.
    pub hosting: bool,

    /// The service offering the privacy service(s) listed here.
    pub service: String,
}

/// Abuse details.
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct AbuseDetails {
    /// The abuse contact's address.
    pub address: String,

    /// The abuse contact's country.
    pub country: String,

    /// The abuse contact's email.
    pub email: String,

    /// The abuse contact's name.
    pub name: String,

    /// The abuse contact's network range.
    pub network: String,

    /// The abuse contact's phone number.
    pub phone: String,
}

/// Domains details.
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct DomainsDetails {
    /// The IP address associated with these hosted domains details.
    pub ip: Option<String>,

    /// The actual total number of domains hosted on this IP address.
    pub total: u64,

    /// A sample list of hosted domains on this IP address.
    pub domains: Vec<String>,
}

/// Country details.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Country {
    pub code: String,
    pub name: String,
}

/// CountryFlag details.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct CountryFlag {
    pub emoji: String,
    pub unicode: String,
}

/// CountryCurrency details.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct CountryCurrency {
    pub code: String,
    pub symbol: String,
}

/// Continent details.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Continent {
    pub code: String,
    pub name: String,
}

impl fmt::Display for Country {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}
