# [<img src="https://ipinfo.io/static/ipinfo-small.svg" alt="IPinfo" width="24"/>](https://ipinfo.io/) IPinfo Rust Client Library

This is the Rust client library for the [IPinfo.io](https://ipinfo.io) IP address API.
It allows you to lookup your own IP address, or get any of the following details for an IP:

- [IP Geolocation](https://ipinfo.io/ip-geolocation-api) (city, region, country, postal code, latitude and longitude)
- [ASN](https://ipinfo.io/asn-api) (ISP or network operator, associated domain name, and type, such as business, hosting or company)
- [Company data](https://ipinfo.io/ip-company-api) (the name and domain of the business that uses the IP address)
- [Carrier details](https://ipinfo.io/ip-carrier-api) (the name of the mobile carrier and MNC and MCC for that carrier if the IP is used exclusively for mobile traffic)

Check all the data we have for your IP address [here](https://ipinfo.io/what-is-my-ip).

## Usage

To use IPinfo, add the followinig to your `Cargo.toml` file.

```toml
[dependencies]
ipinfo = "1.0.0"
```

## Getting Started

An access token is required, which can be acquired by signing up for a free account
at [https://ipinfo.io/signup](https://ipinfo.io/signup).

The free plan is limited to 50,000 requests per month, and doesn't include some of the
data fields such as the IP type and company information. To get the complete list of
information on an IP address and make more requests per day see [https://ipinfo.io/pricing](https://ipinfo.io/pricing).

## Example
This library provides several ready to run examples located in the `/examples` directory. These can be run directly, replacing `<token>` with your access token

```bash
cargo run --example examples/lookup -- <token> 
```
```bash
cargo run --example examples/lookup_batch -- <token> 
```

The `lookup` example above looks more or less like
```rust
use ipinfo::{IpInfo, IpInfoConfig};

fn main() {
    let config = IpInfoConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };

    let mut ipinfo = IpInfo::new(config)
        .expect("should construct");

    let res = ipinfo.lookup("8.8.8.8");
    match res {
        Ok(r) => {
            println!("{} lookup result: {:?}", "8.8.8.8", r);
        },
        Err(e) => println!("error occurred: {}", &e.to_string()),
    }
}
```

## Features

* Smart LRU cache for cost and quota savings.
* Structured and type checked query results.
* Bulk IP address lookup using IPinfo [batch API](https://ipinfo.io/developers/batch).

#### Internationalization

When looking up an IP address, the `response` includes `country_name` which is the country name based on American English, `is_eu` which returns `true` if the country is a member of the European Union (EU), `country_flag` which includes emoji and unicode of a country's flag, `country_currency` 
which includes code and symbol of a country's currency and `continent` which includes code and name of the continent. 

```rust 
let r = ipinfo.lookup("8.8.8.8");
println!("{}: {}", "8.8.8.8", r.country_name) // United States
println!("{}: {:?}", "8.8.8.8", r.is_eu) // Some(false)
println!("{}: {:?}", "8.8.8.8", r.country_flag) // Some(CountryFlag { emoji: "🇺🇸", unicode: "U+1F1FA U+1F1F8" })
println!("{}: {:?}", "8.8.8.8", r.country_currency) // Some(CountryCurrency { code: "USD", symbol: "$" })
println!("{}: {:?}", "8.8.8.8", r.continent) // Some(Continent { code: "NA", name: "North America" })
```

It is possible to return the country name in other languages, change the EU countries and change the flag emoji or unicode by setting the paths of `countries_file_path`, `eu_file_path`, `country_flags_file_path`, `country_currencies_file_path` and `continents_file_path` when creating the `IPinfo` client.

The files must be `.json` files with structures matching the following:

- [countries.json](./assets/countries.json)
- [eu.json](./assets/eu.json)
- [flags.json](./assets/flags.json)
- [currency.json](./assets/currency.json)
- [continent.json](./assets/continent.json)

```rust
let config = IpInfoConfig {
    countries_file_path: Some("custom.json".to_string()),
    eu_file_path: Some("custom_eu.json".to_string()),
    ..Default::default()
};
```

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

Founded in 2013, IPinfo prides itself on being the most reliable, accurate, and in-depth source of IP address data available anywhere. We process terabytes of data to produce our custom IP geolocation, company, carrier, VPN detection, Reverse IP, hosted domains, and IP type data sets. Our API handles over 40 billion requests a month for 100,000 businesses and developers.

[![image](https://avatars3.githubusercontent.com/u/15721521?s=128&u=7bb7dde5c4991335fb234e68a30971944abc6bf3&v=4)](https://ipinfo.io/)

[contributing]: https://github.com/ipinfo/rust/blob/master/CONTRIBUTING.md
