use std::{env, time::{SystemTime, UNIX_EPOCH}};

use anyhow::{anyhow, Context};

use crate::outbound::email_client::EmailConfig;

const SERVER_PORT_KEY: &str = "SERVER_PORT";

const BACKEND_LIVE_OR_LOCAL: &str = "BACKEND";

const EMAIL_CLIENT_ID: &str = "EMAIL_CLIENT_ID";

const EMAIL_CLIENT_SECRET: &str = "EMAIL_CLIENT_SECRET";

const EMAIL_ACCESS_TOKEN: &str = "EMAIL_ACCESS_TOKEN";

const EMAIL_REFRESH_TOKEN: &str = "EMAIL_REFRESH_TOKEN";

const ADMIN_PRIVATE_KEY: &str = "FUEL_DAO_CANISTER_CONTROLLER_PRIVATE_KEY";

const RAZORPAY_KEY: &str = "RAZORPAY_KEY";

const RAZORPAY_SECRET: &str = "RAZORPAY_SECRET";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Config {
    pub server_port: String,
    pub live_or_local: String,
    pub email_config: EmailConfig,
    pub admin_private_key: String,
    pub razorpay_key: String, 
    pub razorpay_secret: String,
}

impl Config {
    pub fn from_env() -> anyhow::Result<Config> {

        dotenv::dotenv().ok();

        let server_port = load_env(SERVER_PORT_KEY).unwrap_or("50051".to_string());


        let live_or_local = load_env(BACKEND_LIVE_OR_LOCAL).unwrap_or("LIVE".to_string());

        let razorpay_key = load_env(RAZORPAY_KEY).ok();
        
        let razorpay_secret = load_env(RAZORPAY_SECRET).ok();
        
        let admin_private_key = load_env(ADMIN_PRIVATE_KEY).map_err(|f| anyhow!("Failed to get admin private key {f:?}"))?;

        let email_config =   EmailConfig {
                client_id: load_env(EMAIL_CLIENT_ID).ok(),
                client_secret: load_env(EMAIL_CLIENT_SECRET).ok(),
                refresh_token: load_env(EMAIL_REFRESH_TOKEN).ok(),
                access_token: load_env(EMAIL_ACCESS_TOKEN).ok(),
                token_expiry:  SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs(),
        };

        Ok(Config {
            server_port,
            live_or_local, 
            email_config,
            admin_private_key: admin_private_key,
            razorpay_key: razorpay_key.context("Failed to get razorpay payment ket")?, 
            razorpay_secret: razorpay_secret.context("Failed to get razorpay payment secret")?,
        })
    }
}

fn load_env(key: &str) -> anyhow::Result<String> {
    env::var(key).with_context(|| format!("failed to load environment variable {}", key))
}