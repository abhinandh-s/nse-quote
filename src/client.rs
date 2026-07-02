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

    pub async fn connect() -> Result<Self, NseError> {
        let client = Self::new()?;
        
        // Ping the base URL to establish a session and grab cookies
        client
            .client
            .get(NSE_BASE)
            .send()
            .await?;
            
        Ok(client)
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
    
    for (i, line) in body.lines().enumerate() {
        if i + 1 == line_nbr {
            println!("Error at {symbol} - Line {line_nbr}, Col {col}");
            
            // Safe unwrap fallback
            if let Some((left, _)) = line.split_at_checked(col) {
                println!("{left}");
            } else {
                println!("{line}"); // Fallback to printing the whole line
            }
        }
    }
    Err(NseError::Serialization(err))
}
        }
    }
}