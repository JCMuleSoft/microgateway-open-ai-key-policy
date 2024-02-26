use serde::Deserialize;
#[derive(Deserialize, Clone, Debug)]
pub struct Config {
    #[serde(alias = "api_key")]
    pub api_key: Vec<String>,
}
