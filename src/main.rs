use bn_btc_price::{symbol_price_ticker::get_coin_price, ticker_24h::{get_24hr_ticker, get_24hr_ticker_json}};







#[tokio::main]
async fn main() {
    if let Err(e) = get_coin_price().await  {
        println!("{}",e);
    }
    if let Err(e) = get_24hr_ticker().await  {
        println!("get_24hr_ticker_json{}",e);
    }

    if let Err(e) = get_24hr_ticker_json().await  {
        println!("get_24hr_ticker_json{}",e);
    }
}
