use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use anyhow::anyhow;
use axum::http::{HeaderMap, HeaderValue};
use base64::engine::general_purpose;
use base64::Engine;
use reqwest::Client;
use serde::{Deserialize, Serialize};

use crate::domain::transactions::models::transaction::Transaction;
use crate::domain::transactions::ports::TransactionNotifier;

/// An unimplemented example of an adapter to [TransactionNotifier].
#[derive(Debug, Clone)]
pub struct EmailClient(Arc<Mutex<EmailConfig>>);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EmailConfig {
    pub client_id: String,
    pub client_secret: String,
    pub access_token: String,
    pub refresh_token: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct TokenResponse {
    access_token: String,
    refresh_token: Option<String>,
}

impl EmailClient {
    pub fn new(config: EmailConfig) -> Self {
        Self(Arc::new(Mutex::new(config)))
    }

    fn get_config(&self) -> anyhow::Result<EmailConfig> {
        let email_config_clone = Arc::clone(&self.0);

        match email_config_clone.clone().lock() {
            Ok(config) => Ok(EmailConfig {
                client_id: config.client_id.clone(),
                client_secret: config.client_secret.clone(),
                access_token: config.access_token.clone(),
                refresh_token: config.refresh_token.clone(),
            }),
            Err(e) => Err(anyhow!("Could not local email config: {e} ")),
        }
    }

    async fn refresh_token(&self) -> anyhow::Result<()> {
        match self.get_config() {
            Ok(config) => {
                let client = Client::new();
                let mut params = HashMap::new();
                params.insert("client_secret", &config.client_secret);
                params.insert("client_id", &config.client_id);
                params.insert("refresh_token", &config.refresh_token);
                params.insert("access_token", &config.access_token);
                let grant_type = format!("refresh_token");
                params.insert("grant_type", &grant_type);

                let response = client
                    .post("https://oauth2.googleapis.com/token")
                    .form(&params) // This sets the Content-Type to application/x-www-form-urlencoded
                    .send()
                    .await?;

                if response.status().is_success() {
                    let token_response: TokenResponse = response.json().await?;
                    let mut email_config = self.0.lock().unwrap(); // Handle lock errors properly in production
                    email_config.access_token = token_response.access_token;
                    Ok(())
                } else {
                    let error_text = response.text().await?;
                    eprintln!("Error: {}", error_text);
                    Err(anyhow!("Failed to get response for refresh_token"))
                }
            }
            Err(_) => Err(anyhow!("Failed to update refresh token")),
        }
    }

    pub async fn send_email_gmail(&self, reservation: &Transaction) -> Result<(), String> {

        let mail_state = self.get_config().ok();
    
        let username = &reservation.name().0;
        let to = &reservation.email().0;
        let cc = "bookings@fueldao.io";
        let booking_id = format!("{}-{}", reservation.car_id(), &reservation.booking_id());
        let start_date = crate::utils::format_datetime(reservation.start_time());
        let end_date = crate::utils::format_datetime(reservation.end_time());
    
    
        match mail_state {
            Some( state) => {
                let access_token = state.access_token;
        let subject = "Booking Confirmed with FuelDao";
        let body = format!("Hey {username},\n\nThank you for choosing FuelDAO! This is a confirmation email of your booking ID {booking_id} with us from {start_date} IST to {end_date} IST.\n\nWatch this space for more details regarding your vehicle details and other information to make it a smooth experience.\n\nRegards\nTeam FuelDao");
        let url = "https://www.googleapis.com/gmail/v1/users/me/messages/send";
    
        // Create the email message
        let email_raw = format!(
            "To: {}\r\nCc: {}\r\nSubject: {}\r\n\r\n{}",
            to,cc, subject, body
        );
        let encoded_message = general_purpose::STANDARD.encode(email_raw) /* encode(email_raw) */; // Base64 encode the email
    
        let payload = serde_json::json!({
            "raw": encoded_message
        });
    
        let client = Client::new();

        let mut headers = HeaderMap::new();

        headers.insert("Authorization", HeaderValue::from_str(&format!("Bearer {}", access_token)).unwrap());
        
        headers.insert("Content-Type", HeaderValue::from_str("application/json").unwrap());

        
        let response = client.post(url).body(serde_json::to_vec(&payload,).unwrap()).headers(headers).send().await;

        if response.as_ref().is_ok() && response.as_ref().unwrap().status().is_success() {

            Ok(())

        }  else {

            let error_text = response.unwrap().text().await.map_err(|f| f.to_string())?;

            eprintln!("Error: {:?}", error_text);
            
            Err(format!("Failed to send email: {:?}", error_text))
        }
        }
            None => Err("Failed to get mail config".to_string()),
    
    }
}

}

impl TransactionNotifier for EmailClient {
    async fn transaction_created(&self, transaction: &Transaction) {
       let res  = self.send_email_gmail(transaction).await;

       match res {
        Err(e) => {
            let _ =  self.refresh_token().await;
            let _ = self.send_email_gmail(transaction).await;
        }, 
        _ => {}
       }
    }
}
