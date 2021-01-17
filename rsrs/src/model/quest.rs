use serde_derive::*;
use std::fmt;
#[derive(Serialize, Deserialize, Clone, Debug)]
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
    
//impl fmt::Debug for Quest {
    //fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        //let thing = format!("{}: awisoegn{}", self.title, self.status);
        ////f.write_str(f, "{}: {}", self.title.replace("\\'", "aaa"), self.status)
        //f.write_str(thing.as_str())
    //}
//}



#[derive(Serialize, Deserialize, Debug)]
pub struct Quests {
    pub quests: Vec<Quest>,
    #[serde(rename = "loggedIn")]
    pub logged_in: String,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub enum Status {
    #[serde(rename = "COMPLETED")]
    Completed,
    #[serde(rename = "NOT_STARTED")]
    NotStarted,
    #[serde(rename = "STARTED")]
    Started
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}
