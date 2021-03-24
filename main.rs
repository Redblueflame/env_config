use proc_config::EnvConfig;
use serde::{Deserialize, Serialize};
use toml::Value;

#[derive(EnvConfig, Deserialize)]
#[env(deserializer = "deserialize")]
pub struct Configuration {
    pub name: String,
    pub address: String,
    pub port: Option<i32>,
}


fn main() {}
