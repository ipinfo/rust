use ipinfo::{IpInfoLite, IpInfoLiteConfig};
use std::env;

#[tokio::main]
async fn main() {
    let token = env::args().nth(1);

    let config = IpInfoLiteConfig {
        token,
        ..Default::default()
    };

    let mut ipinfo = IpInfoLite::new(config).expect("should construct");

    let res = ipinfo.lookup_self_v4().await;
    match res {
        Ok(r) => {
            println!("Current IP lookup result: {:?}", r);
        }
        Err(e) => println!("error occurred: {}", &e.to_string()),
    }
}
