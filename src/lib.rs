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
        Value::String(s) if s == "NA" || s == "-" || s.is_empty() => Ok(None),
        Value::Null => Ok(None),
        _ => Ok(None),
    }
}
