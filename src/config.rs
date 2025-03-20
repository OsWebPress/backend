use std::env;
use std::fs;
use serde::{Serialize, Deserialize};
use sqlx::Postgres;

#[derive(Serialize, Deserialize, Debug)]
pub struct Vault {
    pub root: String,
    pub database_url: String,
    pub jwt_secret: String,
    pub username: String,
    pub password: String,
}

pub struct PressConfig {
    location: String, // config locaiton
    pub settings: Vault,
    pub pool: Option<sqlx::pool::Pool<Postgres>>,
}

pub fn parse_press_config() -> PressConfig {
    let mut press_config = PressConfig {location: String::from("example.json"), settings: Vault { root: String::new(), database_url: String::new(), jwt_secret: String::new(), username: String::new(), password:String::new()}, pool: None};

    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        press_config.location = args[1].clone();
    }

    println!("using config: {}", press_config.location);
    let content = fs::read_to_string(press_config.location.clone()).expect("error, config not found!");

    let deserialized: Vault = serde_json::from_str(&content).unwrap();
    press_config.settings = deserialized;
    println!("content root: {}", press_config.settings.root);

    press_config
}