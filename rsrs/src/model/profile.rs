use serde_derive::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct Activities {
    pub date: String,
    pub details: String,
    pub text: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Profile {
    pub magic: i64,
    pub questsstarted: i64,
    pub totalskill: i64,
    pub questscomplete: i64,
    pub questsnotstarted: i64,
    pub totalxp: i64,
    pub ranged: i64,
    pub activities: Vec<Activities>,
    pub skillvalues: Vec<Skillvalues>,
    pub name: String,
    pub rank: String,
    pub melee: i64,
    pub combatlevel: i64,
    #[serde(rename = "loggedIn")]
    pub logged_in: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Skillvalues {
    pub level: i64,
    pub xp: i64,
    pub rank: i64,
    pub id: i64,
}
