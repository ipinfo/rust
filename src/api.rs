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

use serde::Deserialize;
use serde_json::Value;

/// IP address lookup details.
#[derive(Debug, Deserialize, Clone)]
pub struct IpDetails {
    /// The IP address.
    pub ip: String,

    /// The reverse DNS lookup hostname of the IP address.
    pub hostname: String,

    /// The city for the IP address.
    pub city: String,

    /// The region for the IP address.
    pub region: String,

    /// The country for the IP address.
    pub country: String,

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

    /// TODO
    pub privacy: Option<PrivacyDetails>,

    /// TODO
    pub abuse: Option<AbuseDetails>,

    /// TODO
    pub domains: Option<DomainsDetails>,

    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

/// ASN details.
#[derive(Debug, Deserialize, Clone)]
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
#[derive(Debug, Deserialize, Clone)]
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
#[derive(Debug, Deserialize, Clone)]
pub struct CarrierDetails {
    /// The name of the carrier ISP that owns that mobile IP address.
    pub name: String,

    /// MCC GSM network code of this carrier.
    pub mcc: String,

    /// MNC GSM network code of this carrier.
    pub mnc: String,
}

/// Privacy details.
#[derive(Debug, Deserialize, Clone)]
pub struct PrivacyDetails {
    /// TODO
    pub vpn: bool,

    /// TODO
    pub proxy: bool,

    /// TODO
    pub tor: bool,

    /// TODO
    pub hosting: bool,
}

/// Abuse details.
#[derive(Debug, Deserialize, Clone)]
pub struct AbuseDetails {
    /// TODO
    pub address: String,

    /// TODO
    pub country: String,

    /// TODO
    pub email: String,

    /// TODO
    pub name: String,

    /// TODO
    pub network: String,

    /// TODO
    pub phone: String,
}

/// Domains details.
#[derive(Debug, Deserialize, Clone)]
pub struct DomainsDetails {
    /// TODO
    pub ip: Option<String>,

    /// TODO
    pub total: u64,

    /// TODO
    pub domains: Vec<String>,
}
