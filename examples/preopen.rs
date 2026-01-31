use nse_quote::NseClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = NseClient::new()?;
    let response = client.quote_equity("GROWW").await?;

    println!("Pre-open price  : {}", response.pre_open_market.final_price);
    println!("Total volume    : {}", response.pre_open_market.total_traded_volume);
    println!("Change (%)      : {}", response.pre_open_market.per_change);

    Ok(())
}
