//! curl -H "User-Agent: Mozilla/5.0" https://www.nseindia.com/api/quote-equity?symbol=GROWW

mod client;
mod models;

pub mod error;
pub use client::*;
pub use models::*;

use serde::{Deserialize, Deserializer};
use serde_json::Value;

pub(crate) fn nse_f64<'de, D>(deserializer: D) -> Result<Option<f64>, D::Error>
where
    D: Deserializer<'de>,
{
    let v = Value::deserialize(deserializer)?;

    match v {
        Value::Number(n) => Ok(n.as_f64()),
        Value::String(s) => {
            let s = s.trim();
            if s == "NA" || s == "-" || s.is_empty() {
                Ok(None)
            } else {
                // Remove commas which are common in Indian number formatting
                let clean_s = s.replace(",", "");
                match clean_s.parse::<f64>() {
                    Ok(parsed) => Ok(Some(parsed)),
                    Err(_) => Ok(None), // Or handle the error differently if preferred
                }
            }
        }
        _ => Ok(None),
    }
}