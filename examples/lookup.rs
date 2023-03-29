use std::env;
use ipinfo::{IpInfo, IpInfoConfig};

#[tokio::main]
async fn main() {
    let token = env::args().skip(1).next();

    let config = IpInfoConfig {
        token,
        ..Default::default()
    };

    let mut ipinfo = IpInfo::new(config)
        .expect("should construct");

    let res = ipinfo.lookup("8.8.8.8").await;
    match res {
        Ok(r) => {
            println!("{} lookup result: {:?}", "8.8.8.8", r);
        },
        Err(e) => println!("error occurred: {}", &e.to_string()),
    }
}
