//! Basic full response dump.

use nse_quote::NseClient;

const SYMBOLS: [&str; 1] = ["GROWW"];

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = NseClient::new()?;
    for symbol in SYMBOLS {
        let response = client.quote_equity(symbol).await?;

        println!("{:#?}", response);
    }

    Ok(())
}
