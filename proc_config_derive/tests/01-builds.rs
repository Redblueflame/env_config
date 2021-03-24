use proc_config_derive::EnvConfig;

#[derive(EnvConfig, Deserialize)]
#[env(deserializer = "deserialize")]
pub struct Config {
    pub name: String,
    pub db_path: String,
}
fn deserialize<'a, T: serde::Deserialize<'a>>(_source: &'a str) -> Result<T, toml::de::Error> {
    unimplemented!()
}

fn main() {}
