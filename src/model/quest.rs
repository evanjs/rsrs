use serde_derive::*;
#[derive(Serialize, Deserialize, Debug)]
pub struct Quest {
    pub title: String,
    pub status: Status,
    pub difficulty: i64,
    pub members: bool,
    #[serde(rename = "questPoints")]
    pub quest_points: i64,
    #[serde(rename = "userEligible")]
    pub user_eligible: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Quests {
    pub quests: Vec<Quest>,
    #[serde(rename = "loggedIn")]
    pub logged_in: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Status {
    COMPLETED,
    NOT_STARTED,
    STARTED
}
