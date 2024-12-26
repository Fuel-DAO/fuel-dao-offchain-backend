use std::env;

use reqwest::Client;
use serde_json::Value;

use crate::domain::transactions::ports::PaymentService;


#[derive(Clone)]
pub struct PaymentClient {
    config: PaymentConfig,
}

#[derive(Clone)]
pub struct PaymentConfig {
    pub payment_key: String, 
    pub payment_secret: String,
}

impl PaymentClient {
    pub fn new(config: PaymentConfig) -> Self {
        Self { config }
    }
}


impl PaymentService for PaymentClient {
    
    async fn create_payment_link(
        &self,
        payment_amount_in_inr_f32: f64,
        booking_id: u64
    ) -> Result<String, String> {
        // Payment amount and callback URL
        // dotenv::dotenv().ok();
        // let live = env::var("BACKEND").unwrap_or("LIVE".to_string()) == "LIVE" ;
        let live = true;
        let callback_url = if !live {
            "http://localhost:8080/payment"
        } else {
            "https://fuelev.in/payment"
        };
    // API endpoint
    let url = "https://api.razorpay.com/v1/payment_links";

    // Create the JSON payload
    let payload = serde_json::json!({
        "amount": (payment_amount_in_inr_f32 * 100.0) as u64, // Convert to paisa
        "callback_url": callback_url,
        "reference_id": booking_id.to_string(),
    });

    // Create the reqwest client
    let client = Client::new();

    // Send the POST request
    let response = client
        .post(url)
        .basic_auth(&self.config.payment_key, Some(&self.config.payment_secret)) // Add basic auth
        .header("Content-Type", "application/json") // Set content type
        .json(&payload) // Add JSON payload
        .send() // Send the request
        .await.map_err(|f| f.to_string())?;

    // Check if the response was successful
    if response.status().is_success() {
        // Parse the response as JSON
        let response_json: Value = response.json().await.map_err(|f| f.to_string())?;

        // Extract and print the short_url
        if let Some(short_url) = response_json.get("short_url").and_then(|v| v.as_str()) {
            Ok(short_url.to_string())
        } else {
            Err("short_url not found in the response.".to_string())
        }
    } else {
        // Handle error response
        Err(format!("Error: {:?}", response.text().await))
    }

    }
}
