mod client;
mod models;

use crate::Response;

pub mod error::NseError; 

pub use client::*;
pub use models::*;

use reqwest::header;
use serde::{Deserialize, Deserializer};
use serde_json::Value;

const NSE_BASE: &str = "https://www.nseindia.com";

pub(crate) fn nse_f64<'de, D>(deserializer: D) -> Result<Option<f64>, D::Error>
where
    D: Deserializer<'de>,
{
    let v = Value::deserialize(deserializer)?;

    match v {
        Value::Number(n) => Ok(n.as_f64()),
        Value::String(s) if s == "NA" || s == "-" || s.trim().is_empty() => Ok(None),
        Value::Null => Ok(None),
        _ => Ok(None),
    }
}

pub struct NseClient {
    client: reqwest::Client,
}

impl NseClient {
    // Notice this is now async so we can fetch the initial cookies
    pub async fn new() -> Result<Self, NseError> {
        let mut headers = header::HeaderMap::new();
        headers.insert(
            header::USER_AGENT,
            header::HeaderValue::from_static("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36"),
        );
        headers.insert(
            header::ACCEPT,
            header::HeaderValue::from_static("text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,*/*;q=0.8"),
        );
        headers.insert(
            header::ACCEPT_LANGUAGE,
            header::HeaderValue::from_static("en-US,en;q=0.5"),
        );

        let client = reqwest::Client::builder()
            .default_headers(headers)
            .cookie_store(true)
            .build()?;

        // Hit the home page to grab the session cookies
        let _ = client.get(NSE_BASE).send().await?;

        Ok(Self { client })
    }

    pub async fn quote_equity(&self, symbol: &str) -> Result<Response, NseError> {
        let symbol = symbol.replace("&", "%26");
        let url = format!("{}/api/quote-equity?symbol={}", NSE_BASE, symbol);

        let res = self.client.get(&url).send().await?;
        
        // Ensure we actually got a 200 OK before trying to parse
        let res = res.error_for_status()?; 
        
        let body = res.text().await?;
        let parsed: Result<Response, serde_json::Error> = serde_json::from_str(&body);

        match parsed {
            Ok(res) => Ok(res),
            Err(err) => {
                let (line_nbr, col) = (err.line(), err.column());
                
                // Safer way to extract the error string handling UTF-8 safely
                if let Some(line) = body.lines().nth(line_nbr.saturating_sub(1)) {
                    let left: String = line.chars().take(col).collect();
                    println!("Failed symbol: {symbol}");
                    println!("Error line prefix: {left}");
                }
                
                Err(NseError::Serialization(err))
            }
        }
    }
}
