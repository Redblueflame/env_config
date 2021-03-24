use proc_config_derive::EnvConfig;

#[derive(EnvConfig, Deserialize)]
#[env(deserializer = "deserialize")]
pub struct Config {
    pub name: String,
    pub db_path: Option<String>,
}

fn deserialize<'a, T: serde::Deserialize<'a>>(source: &'a str) -> Result<T, toml::de::Error> {
    return toml::from_str(source.as_ref());
}

fn main() {}
