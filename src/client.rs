use reqwest::header;

use crate::Response;
use crate::error::NseError;

const NSE_BASE: &str = "https://www.nseindia.com";

pub struct NseClient {
    client: reqwest::Client,
}

impl NseClient {
    pub fn new() -> Result<Self, NseError> {
        let mut headers = header::HeaderMap::new();
        headers.insert(
            header::USER_AGENT,
            header::HeaderValue::from_static("Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36"),
        );
        headers.insert(
            header::ACCEPT,
            header::HeaderValue::from_static("application/json"),
        );

        let client = reqwest::Client::builder()
            .default_headers(headers)
            .cookie_store(true)
            .build()?;

        Ok(Self { client })
    }

    pub async fn quote_equity(&self, symbol: &str) -> Result<Response, NseError> {
        let url = format!("{}/api/quote-equity?symbol={}", NSE_BASE, symbol);
        let res = self
            .client
            .get(&url)
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?;
        Ok(res)
    }
}
