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
        println!("Usage: save_url <url>");
        ()
    })?;

    let mut entry = NewEntry::new_with_url(url);
    entry.tags = Some(vec!["wat,hing".to_owned(), "console".to_owned()]);

    let res = client.create_entry(&entry);

    match res {
        Err(e) => {
            println!("Failed to add entry: {:?}", e);
            Err(())
        }
        Ok(entry) => {
            println!("Success!");
            println!("{:#?}", entry);
            Ok(())
        }
    }
}
