use ipinfo::{IpInfo, IpInfoConfig};
use std::env;

#[tokio::main]
async fn main() {
    let token = env::args().nth(1);

    let config = IpInfoConfig {
        token,
        ..Default::default()
    };

    let mut ipinfo = IpInfo::new(config).expect("should construct");

    let res = ipinfo.lookup("8.8.8.8").await;
    match res {
        Ok(r) => {
            println!("8.8.8.8 lookup result: {:?}", r);
        }
        Err(e) => println!("error occurred: {}", &e.to_string()),
    }
}
