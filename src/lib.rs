use log::*;
use reqwest;
use serde::*;
use serde_json::*;

mod model;

const BASE_URI: &str = "https://apps.runescape.com";
const PLAYER_INFORMATION_URL: &str = "/runemetrics/profile/profile";
const QUEST_INFORMATION_URL: &str = "/runemetrics/quests";

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
        let query = &[("user", username), ("count", &count.to_string())];
        trace!("Query: {:#?}", query);
        let mut res = self
            .client
            .get(&url)
            .query(query)
            .send()
            .expect("Failed to get profile request");
        debug!("Request: {:#?}", &res);

        let text = res.text().expect("Failed to get text from profile request");
        trace!("Profile text: {:#?}", &text);
        serde_json::from_str(&text).expect("Failed to deserialize JSON for profile")
    }

    fn get_quests(&self, username: &'static str) -> model::quest::Quests {
        let url = format!("{}{}", BASE_URI, QUEST_INFORMATION_URL);
        let query = &[("user", username)];
        trace!("Query: {:#?}", query);
        let mut res = self
            .client
            .get(&url)
            .query(query)
            .send()
            .expect("Failed to get quest details for user");
        debug!("Request: {:#?}", &res);

        let text = res.text().expect("Failed to get text from quests request");
        trace!("Quests text: {:#?}", &text);
        serde_json::from_str(&text).expect("Failed to deserialize")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use env_logger;

    fn main() {
    }

    #[test]
    fn test_profile() {
        let username = "evanjs";
        let client = RsClient::new();
        let profile = client.get_profile(username, 20);
        println!("Profile: {:#?}", &profile);
        assert_eq!(profile.name.to_lowercase(), username.to_lowercase());
    }

    #[test]
    fn test_quests() {
        let username = "evanjs";
        let client = RsClient::new();
        let quests = client.get_quests(username);
        println!("Quests: {:#?}", &quests);
    }
}
