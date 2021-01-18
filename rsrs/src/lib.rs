use log::*;
use reqwest;
use anyhow::{Result, anyhow};

pub mod model;

const BASE_URI: &str = "https://apps.runescape.com";
const PLAYER_INFORMATION_URL: &str = "/runemetrics/profile/profile";
const QUEST_INFORMATION_URL: &str = "/runemetrics/quests";

#[derive(Default)]
pub struct RsClient {
    client: reqwest::Client,
    username: String
}

impl RsClient {
    pub fn new(username: Option<String>) -> RsClient {
        RsClient {
            username: username.unwrap_or_else(||String::from("")),
            ..Default::default()
        }
    }

    fn get_user(&self, username: Option<&'static str>) -> Result<String> {
        debug!("Username: {:?}", username);
        debug!("Username2: {:?}", self.username);
        if self.username.is_empty() {
            if username.is_none() {
                Err(anyhow!("Please provide a username to query. 1"))
            } else {
                Ok(username.unwrap().to_string())
            }
        } else {
            Ok(self.username.to_string())
        }
    }

    pub async fn get_profile(&self, username: Option<&'static str>, count: u64) -> Result<model::profile::Profile> {
        let url = format!("{}{}", BASE_URI, PLAYER_INFORMATION_URL);
        let user = self.get_user(username)?;
        let query = &[("user", user), ("count", count.to_string())];
        trace!("Query: {:#?}", query);
        let res = self
            .client
            .get(&url)
            .query(query)
            .send()
            .await?;
        debug!("Request: {:#?}", &res);

        let profile: model::profile::Profile = res.json().await.expect("Failed to get text from profile request");
        trace!("Profile text: {:#?}", &profile);
        Ok(profile)
    }

    pub async fn get_quests(&self, username: Option<&'static str>) -> Result<model::quest::Quests> {
        let url = format!("{}{}", BASE_URI, QUEST_INFORMATION_URL);
        let user = self.get_user(username)?;
        let query = &[("user", user)];
        trace!("Query: {:#?}", query);
        let res = self
            .client
            .get(&url)
            .query(query)
            .send()
            .await?;
        debug!("Request: {:#?}", &res);

        Ok(res.json::<model::quest::Quests>().await.expect("Failed to get text from quests request"))
    }

    pub async fn get_completed_quests(&self, quests: model::quest::Quests) -> Vec<model::quest::Quest> {
        let qs = quests.quests.iter().map(|q| q.clone());
        let fs = qs.filter(|q| q.status == model::quest::Status::Completed).collect(); 
        fs
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[async_std::test]
    async fn test_profile() -> Result<()> {
        let username = String::from("evanjs");
        let client = RsClient::new(Some(username.clone()));
        let profile = client.get_profile(None, 20).await?;
        println!("Profile: {:#?}", &profile);
        assert_eq!(profile.name.to_lowercase(), username.to_lowercase());
        Ok(())
    }

    #[async_std::test]
    async fn test_quests() -> Result<()> {
        let username = String::from("evanjs");
        let client = RsClient::new(Some(username.clone()));
        let quests = client.get_quests(None).await?;
        println!("Quests: {:#?}", &quests);
        Ok(())
    }
}
