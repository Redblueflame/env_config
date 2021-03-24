pub use proc_config_derive::*;
use std::fmt;
use std::fmt::{Display, Formatter};
pub use thiserror::Error;



#[derive(Error, Debug)]
pub enum Error {
    /// There were fields missing from both the configuration file and the environment values.
    /// The attached Vec<String> is an array of all the missing values.
    MissingFields(Vec<String>),
    /// An issue occurred while accessing the config file.
    ReadError(#[from] std::io::Error),
    /// An error occurred while parsing the configuration file.
    ParsingError()
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:#?}", self)
    }
}

pub trait OptionStruct {
    fn set_elem<T>(name: &str, value: T);
    fn finalize() -> Result<Box<Self>, Error>;
}


pub trait EnvConfig {
    fn load<P: AsRef<std::path::Path>>(path: P) -> Result<Box<Self>, Error>;
}

pub trait OptionStructParsing {
    fn apply<P>(&mut self) -> &mut Self;
}


pub fn get_env_variable(name: &str) -> Option<String> {
    match std::env::var(name) {
        Ok(e) => Some(e),
        Err(_) => None
    }
}
