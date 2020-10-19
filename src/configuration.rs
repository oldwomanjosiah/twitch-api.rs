use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    #[serde(skip_serializing_if = "Option::is_none")]
    note: Option<String>,
    pub client_id: String,
    pub client_secret: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            note: Some(String::from(
                "Get these from https://dev.twitch.tv/console/apps",
            )),
            client_id: String::from("Client ID"),
            client_secret: String::from("Client Secret"),
        }
    }
}
