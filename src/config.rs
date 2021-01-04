extern crate toml;

use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf};
use toml::de::Error;

#[derive(Clone, Deserialize, Debug, Serialize)]
pub struct Nats {
    /// The nats server URLs (separated by comma) (default "nats://127.0.0.1:4222")
    pub url: String,
    /// User Credentials File
    pub creds: Option<String>,
    /// Use TLS Secure Connection
    pub tls: bool,
    /// The subject to default to
    pub subject: String,
}

impl Default for Nats {
    fn default() -> Self {
        Nats {
            url: "nats://127.0.0.1:4222".to_string(),
            tls: false,
            subject: "channels.all".to_string(),
            creds: None,
        }
    }
}

#[derive(Clone, Deserialize, Debug, Serialize)]
pub struct Server {
    /// Private Endpoint for consuming data from
    pub private: String,
    /// Public Endpoint for publishing data to
    pub public: String,
}
impl Default for Server {
    fn default() -> Self {
        Server {
            public: "0.0.0.0:9002".to_string(),
            private: "127.0.0.1:9001".to_string(),
        }
    }
}

#[derive(Clone, Deserialize, Debug, Serialize)]
pub struct Config {
    #[serde(default)]
    pub server: Server,
    #[serde(default)]
    pub nats: Nats,
}

impl Config {
    pub fn from_file(filename: &PathBuf) -> Result<Self, Error> {
        let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
        toml::from_str(&*contents)
    }

    pub fn print_default() {
        let conf: Config = toml::from_str("").expect("failed to set default");
        let toml = toml::to_string_pretty(&conf).unwrap();
        println!("{}", toml)
    }
}
