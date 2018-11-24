use std::env;
use std::error::Error;
use std::result::Result;

use wallabag_api::types::{AuthInfo, Config, NewAnnotation, NewEntry, Range};
use wallabag_api::Client;

pub fn main() -> Result<(), ()> {
    let config = Config {
        auth_info: AuthInfo {
            client_id: env::var("WALLABAG_CLIENT_ID").expect("WALLABAG_CLIENT_ID not set"),
            client_secret: env::var("WALLABAG_CLIENT_SECRET")
                .expect("WALLABAG_CLIENT_SECRET not set"),
            username: env::var("WALLABAG_USERNAME").expect("WALLABAG_USERNAME not set"),
            password: env::var("WALLABAG_PASSWORD").expect("WALLABAG_PASSWORD not set"),
        },
        base_url: "https://framabag.org".to_owned(),
    };

    println!("{:#?}", config);

    let mut client = Client::new(config);

    let url = std::env::args().nth(1).ok_or_else(|| {
        println!("Usage: check_exists <url>");
        ()
    })?;

    let res = client.check_exists(&url);

    match res {
        Err(e) => {
            println!("Request failed: {:?}", e);
            Err(())
        }
        Ok(exists) => {
            match exists {
                Some(id) => {
                    println!("Exists. id: {}", id);
                }
                None => {
                    println!("Url does not exist: {}", url);
                }
            }
            Ok(())
        }
    }
}