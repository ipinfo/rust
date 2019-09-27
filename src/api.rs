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

#[derive(Debug, Deserialize, Clone)]
pub struct IpDetails {
    pub ip: String,
    pub hostname: String,
    pub city: String,
    pub region: String,
    pub country: String,
    pub loc: String,
    pub postal: Option<String>,
    pub timezone: Option<String>,
    pub org: Option<String>,
    pub asn: Option<AsnDetails>,
    pub company: Option<CompanyDetails>,
    pub carrier: Option<CarrierDetails>,

    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct AsnDetails {
    pub asn: String,
    pub name: String,
    pub domain: String,
    pub route: String,

    #[serde(rename = "type")]
    pub asn_type: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct CompanyDetails {
    pub name: String,
    pub domain: String,

    #[serde(rename = "type")]
    pub company_type: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct CarrierDetails {
    pub name: String,
    pub mcc: String,
    pub mnc: String,
}
