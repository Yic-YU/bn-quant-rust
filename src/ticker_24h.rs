use reqwest::Error;



struct Ticker24hr{
    symbol: String,
    quotevolume: String
}

pub async fn get_24hr_ticker_json() -> Result<(), Error> {
    let api = "https://api.binance.com/api/v3/ticker/24hr";

    let response = reqwest::get(api).await?;

    let json = response.text().await?;

    let _ = tokio::fs::write("24h.txt", json).await;

    Ok(())
}





pub async fn get_24hr_ticker() -> Result<(), Error> {
    
    let api = "https://api.binance.com/api/v3/ticker/24hr";

    let response = reqwest::get(api).await?;

    let volume = response.json::<Vec<String>>().await?;

    for qv in volume {
        println!("{qv}");
    }

    Ok(())
}