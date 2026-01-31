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
        let symbol = symbol.replace("&", "%26");
        let url = format!("{}/api/quote-equity?symbol={}", NSE_BASE, symbol);
        
        let res = self.client.get(&url).send().await?;
        let body = res.text().await?;
        let parsed: Result<Response, serde_json::Error> = serde_json::from_str(&body);
        
        match parsed {
            Ok(res) => Ok(res),
            Err(err) => {
                let (line_nbr, col) = (err.line(), err.column());
                let mut line_count = 1usize;
                for line in body.lines() {
                    if line_nbr == line_count {
                        let (left, _) = line.split_at_checked(col).unwrap();
                        println!("{symbol}");
                        println!("{left}");
                    }
                    line_count += 1;
                }
                Err(NseError::Serialization(err))
            }
        }
    }
}
