use serde_derive::*;
#[derive(Serialize, Deserialize)]
pub struct Quest {
    title: String,
    status: String,
    difficulty: i64,
    members: bool,
    #[serde(rename = "questPoints")]
    quest_points: i64,
    #[serde(rename = "userEligible")]
    user_eligible: bool,
}

#[derive(Serialize, Deserialize)]
pub struct Quests {
    quests: Vec<Quest>,
    #[serde(rename = "loggedIn")]
    logged_in: String,
}
