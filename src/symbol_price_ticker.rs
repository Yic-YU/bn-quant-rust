use reqwest::Error;
use serde::Deserialize;



#[derive(Deserialize, Clone)]
struct SymbolPriceTicker{
    symbol: String,
    price: String,
}


pub async fn get_coin_price() -> Result<(), Error> {

    let api = "https://api.binance.com/api/v3/ticker/price";

    let res = reqwest::get(api).await?;

    let tickers= res.json::<Vec<SymbolPriceTicker>>().await?;
    
    
    Ok(())
}