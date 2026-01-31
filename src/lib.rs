//! curl -H "User-Agent: Mozilla/5.0" https://www.nseindia.com/api/quote-equity?symbol=GROWW

mod client;
mod models;

pub mod error;
pub use client::*;
pub use models::*;
