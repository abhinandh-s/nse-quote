use nse_quote::NseClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = NseClient::new()?;
    let response = client.quote_equity("GROWW").await?;

    println!("Symbol        : {}", response.info.symbol);
    println!("Company       : {}", response.info.company_name);
    println!("Last Price    : ₹{}", response.price_info.last_price);
    println!("Change (%)    : {:+.2}%", response.price_info.p_change);
    println!("Day High/Low  : {} / {}",
        response.price_info.intra_day_high_low.max,
        response.price_info.intra_day_high_low.min,
    );

    Ok(())
}
