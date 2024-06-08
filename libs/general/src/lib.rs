// use crate::socket_addrs::WebUrl;
// use std::net::Ipv4Addr;

use std::sync::OnceLock;

pub mod socket_addrs;

// use serde::{Deserialize, Serialize};

const PORT_STR: &str = "PORT";
const DEFAULT_PORT: &str = "8080";

// const POSTGRES_STR: &str = "POSTGRES_URL";
const POSTGRES_STR: &str = "DATABASE_URL";
const DEFAULT_POSTGRES_URI: &str = "postgres://myuser:mypassword@localhost/mydatabase";

const MONGO_STR: &str = "MONGO_URI";
const DEFAULT_MONGO_URI: &str = "mongodb://localhost:27017";

const REDIS_STR: &str = "REDIS_HOST";
const DEFAULT_REDIS_URI: &str = "redis://localhost:6379";

fn get_env_variable(variable_name: &str, default_value: &str) -> String {
    std::env::var(variable_name).unwrap_or(default_value.to_string())
}

pub fn get_mongo_uri() -> String {
    get_env_variable(MONGO_STR, DEFAULT_MONGO_URI)
}

pub fn get_redis_uri() -> String {
    get_env_variable(REDIS_STR, DEFAULT_REDIS_URI)
}

pub fn get_sql_uri() -> String {
    get_env_variable(POSTGRES_STR, DEFAULT_POSTGRES_URI)
}

pub fn get_port() -> u16 {
    get_env_variable(PORT_STR, DEFAULT_PORT)
        .parse()
        .unwrap_or(8080)
}


// New way

#[derive(Debug)]
pub enum Error {
    ConfigMissingEnv(&'static str),
}

pub fn config() -> &'static Config {
    static INSTANCE: OnceLock<Config> = OnceLock::new();

    INSTANCE.get_or_init(|| {
      Config::load_from_env().unwrap_or_else(|ex| {
        panic!("Fatal - While loading config - Cause: {ex:?}")
      })
    })
}

#[allow(non_snake_case)]
pub struct Config {
  // -- Web server
  pub WEB_FOLDER: String,
}

impl Config {
  fn load_from_env() -> Result<Config, Error> {
    Ok(Self {
      WEB_FOLDER: get_env("WEB_FOLDER")?,
    })
  }
}

fn get_env(name: &'static str) -> Result<String, Error> {
    std::env::var(name).map_err(|_| Error::ConfigMissingEnv(name))
}