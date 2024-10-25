use std::{env, time::{SystemTime, UNIX_EPOCH}};

use anyhow::Context;

use crate::outbound::email_client::EmailConfig;

const SERVER_PORT_KEY: &str = "SERVER_PORT";

const BACKEND_LIVE_OR_LOCAL: &str = "BACKEND";

const EMAIL_CLIENT_ID: &str = "EMAIL_CLIENT_ID";

const EMAIL_CLIENT_SECRET: &str = "EMAIL_CLIENT_SECRET";

const EMAIL_ACCESS_TOKEN: &str = "EMAIL_ACCESS_TOKEN";

const EMAIL_REFRESH_TOKEN: &str = "EMAIL_REFRESH_TOKEN";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Config {
    pub server_port: String,
    pub live_or_local: String,
    pub email_config: EmailConfig,
}

impl Config {
    pub fn from_env() -> anyhow::Result<Config> {

        dotenv::dotenv().ok();

        let server_port = load_env(SERVER_PORT_KEY).unwrap_or("50051".to_string());


        let live_or_local = load_env(BACKEND_LIVE_OR_LOCAL).unwrap_or("LIVE".to_string());

        let email_config =   EmailConfig {
                client_id: load_env(EMAIL_CLIENT_ID).ok(),
                client_secret: load_env(EMAIL_CLIENT_SECRET).ok(),
                refresh_token: load_env(EMAIL_REFRESH_TOKEN).ok(),
                access_token: load_env(EMAIL_ACCESS_TOKEN).ok(),
                token_expiry:  SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs(),
        }  ;

        Ok(Config {
            server_port,
            live_or_local, 
            email_config
        })
    }
}

fn load_env(key: &str) -> anyhow::Result<String> {
    env::var(key).with_context(|| format!("failed to load environment variable {}", key))
}