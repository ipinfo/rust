//   Copyright 2024 IPinfo library developers
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
//! Functions for checking whether an IP address is bogus.
//!
//! Here "bogus" or "bogon" means an IP address that is not valid for use on the
//! public internet. This includes private IP addresses, loopback addresses, and
//! other reserved addresses.
//!
//! This module may return false negatives.
//!
//! # Example
//!
//! ```
//! use ipinfo::is_bogon;
//!
//! assert_eq!(is_bogon("127.0.0.1"), true);
//! assert_eq!(is_bogon("8.8.8.8"), false);
//! ```
use std::net::IpAddr;

use ipnetwork::{Ipv4Network, Ipv6Network};
use lazy_static::lazy_static;

lazy_static! {
    /// IPv4 bogon networks
    static ref BOGON_V4_NETWORKS: Vec<Ipv4Network> = [
        "0.0.0.0/8",
        "10.0.0.0/8",
        "100.64.0.0/10",
        "127.0.0.0/8",
        "169.254.0.0/16",
        "172.16.0.0/12",
        "192.0.0.0/24",
        "192.0.2.0/24",
        "192.168.0.0/16",
        "198.18.0.0/15",
        "198.51.100.0/24",
        "203.0.113.0/24",
        "224.0.0.0/4",
        "240.0.0.0/4",
        "255.255.255.255/32"
    ]
    .iter()
    .map(|s| s.parse().expect("invalid ipv4 network"))
    .collect();

    /// IPv6 bogon networks
    static ref BOGON_V6_NETWORKS: Vec<Ipv6Network> = [
        "::/128",
        "::1/128",
        "::ffff:0:0/96",
        "::/96",
        "100::/64",
        "2001:10::/28",
        "2001:db8::/32",
        "fc00::/7",
        "fe80::/10",
        "fec0::/10",
        "ff00::/8",
        "2002::/24",
        "2002:a00::/24",
        "2002:7f00::/24",
        "2002:a9fe::/32",
        "2002:ac10::/28",
        "2002:c000::/40",
        "2002:c000:200::/40",
        "2002:c0a8::/32",
        "2002:c612::/31",
        "2002:c633:6400::/40",
        "2002:cb00:7100::/40",
        "2002:e000::/20",
        "2002:f000::/20",
        "2002:ffff:ffff::/48",
        "2001::/40",
        "2001:0:a00::/40",
        "2001:0:7f00::/40",
        "2001:0:a9fe::/48",
        "2001:0:ac10::/44",
        "2001:0:c000::/56",
        "2001:0:c000:200::/56",
        "2001:0:c0a8::/48",
        "2001:0:c612::/47",
        "2001:0:c633:6400::/56",
        "2001:0:cb00:7100::/56",
        "2001:0:e000::/36",
        "2001:0:f000::/36",
        "2001:0:ffff:ffff::/64",
    ]
    .iter()
    .map(|s| s.parse().expect("invalid ipv6 network"))
    .collect();
}

/// Returns a boolean indicating whether an IP address is bogus.
///
/// Returns `false` if the IP address is invalid.
///
/// # Examples
///
/// ```
/// use ipinfo::is_bogon;
///
/// assert_eq!(is_bogon("127.0.0.1"), true);
/// assert_eq!(is_bogon("8.8.8.8"), false);
/// assert_eq!(is_bogon("::1"), true);
/// assert_eq!(is_bogon("2606:4700:4700:1111::2"), false);
/// assert_eq!(is_bogon("foo"), false);
/// ```
pub fn is_bogon(ip_address: &str) -> bool {
    ip_address.parse().is_ok_and(is_bogon_addr)
}

/// Returns a boolean indicating whether an IP address is bogus.
///
/// # Examples
///
/// ```
/// use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
/// use ipinfo::is_bogon_addr;
///
/// assert_eq!(is_bogon_addr(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1))), true);
/// assert_eq!(is_bogon_addr(IpAddr::V4(Ipv4Addr::new(8, 8, 8, 8))), false);
/// assert_eq!(is_bogon_addr(IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1))), true);
/// assert_eq!(is_bogon_addr(IpAddr::V6(Ipv6Addr::new(0x2606, 0x4700, 0x4700, 0x1111, 0, 0, 0, 2))), false);
/// ```
pub fn is_bogon_addr(ip_address: IpAddr) -> bool {
    match ip_address {
        IpAddr::V4(ip) => BOGON_V4_NETWORKS
            .iter()
            .any(|&network| network.contains(ip)),
        IpAddr::V6(ip) => BOGON_V6_NETWORKS
            .iter()
            .any(|&network| network.contains(ip)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_bogon() {
        let bogus = ["169.254.0.1", "192.0.2.1", "2001:db8::1"];
        for ip in bogus.iter() {
            assert!(is_bogon(ip));
        }

        let legit = ["8.8.8.8", "1.1.1.1", "192.1.0.0", "2001:470:1f0b:1::1"];
        for ip in legit.iter() {
            assert!(!is_bogon(ip));
        }
    }
}
