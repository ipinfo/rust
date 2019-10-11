# [<img src="https://ipinfo.io/static/ipinfo-small.svg" alt="IPinfo" width="24"/>](https://ipinfo.io/) IPinfo Rust Client Library

This is the Rust client library for the [IPinfo.io](https://ipinfo.io) IP address API.
It allows you to lookup your own IP address, or get any of the following details for an IP:

- IP geolocation (city, region, country, postal code, latitude and longitude)
- ASN details (ISP or network operator, associated domain name, and type, such as business, hosting or company)
- Company details (the name and domain of the business that uses the IP address)
- Carrier details (the name of the mobile carrier and MNC and MCC for that carrier if the IP is used exclusively for mobile traffic)

## Usage

To use IPinfo, add the followinig to your `Cargo.toml` file.

```toml
[dependencies]
ipinfo = "0.1"
```

## Getting Started

An access token is required, which can be acquired by signing up for a free account
at [https://ipinfo.io/signup](https://ipinfo.io/signup).

The free plan is limited to 1,000 requests per day, and doesn't include some of the
data fields such as the IP type and company information. To get the complete list of
information on an IP address and make more requests per day see [https://ipinfo.io/pricing](https://ipinfo.io/pricing).

## Example

```rust
use ipinfo::{IpInfo, IpInfoConfig};

fn main() {
  let config = IpInfoConfig { token: Some("my token".to_string()), ..Default::default() };
  let mut ipinfo = IpInfo::new(config).expect("should construct");
  let res = ipinfo.lookup(&["8.8.8.8", "4.2.2.4"]);

  match res {
    Ok(r) => println!("{}: {}", "8.8.8.8", r["8.8.8.8"].hostname),
    Err(e) => println!("error occurred: {}", &e.to_string()),
  }
}
```

## Features

* Smart LRU cache for cost and quota savings.
* Structured and type checked query results.
* Bulk IP address lookup using IPinfo batch API.

## Other Libraries

There are official IPinfo client libraries available for many languages including
PHP, Go, Java, Ruby, and many popular frameworks such as Django, Rails and Laravel.
There are also many third party libraries and integrations available for our API.

## Contributing

Thought of something you'd like to see? You can visit the issue tracker
to check if it was reported or proposed before, and if not please feel free to
create an issue or feature request. Ready to start contributing?
The [contributing guide][contributing] is a good place to start. If you have
questions please feel free to ask.

## About IPinfo

Founded in 2013, IPinfo prides itself on being the most reliable, accurate, and
in-depth source of IP address data available anywhere. We process terabytes of
data to produce our custom IP geolocation, company, carrier and IP type data sets.
Our API handles over 12 billion requests a month for 100,000 businesses and developers.

[![image](https://avatars3.githubusercontent.com/u/15721521?s=128&u=7bb7dde5c4991335fb234e68a30971944abc6bf3&v=4)](https://ipinfo.io/)

[contributing]: https://github.com/amrali/waver/blob/master/CONTRIBUTING.md
