use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub username: String
}
