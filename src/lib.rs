use log::*;
use reqwest;
use serde::*;
use serde_json::*;

mod model;

const BASE_URI: &str = "https://apps.runescape.com";
const PLAYER_INFORMATION_URL: &str = "/runemetrics/profile/profile";

struct RsClient {
    client: reqwest::Client,
}

impl RsClient {
    fn new() -> RsClient {
        RsClient {
            client: reqwest::Client::new(),
        }
    }

    fn get_profile(&self, username: &'static str, count: u64) -> model::profile::Profile {
        let url = format!("{}{}", BASE_URI, PLAYER_INFORMATION_URL);
        let stuff = &[("user", username), ("count", &count.to_string())];
        trace!("Stuff: {:#?}", stuff);
        let mut res = self
            .client
            .get(&url)
            .query(stuff)
            .send()
            .expect("Failed to get profile request");
        debug!("Request: {:#?}", &res);

        let text = res.text().expect("Failed to get text from profile request");
        trace!("Profile text: {:#?}", &text);
        serde_json::from_str(&text).expect("Failed to deserialize JSON for profile")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use env_logger;

    #[test]
    fn test_profile() {
        env_logger::init();
        let username = "evanjs";
        let client = RsClient::new();
        let profile = client.get_profile(username, 20);
        assert_eq!(profile.name.to_lowercase(), username.to_lowercase());
    }
}
