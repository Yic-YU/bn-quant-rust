use std::error::Error;

use serde::Deserialize;
use tokio::{ sync::broadcast::{self}};




#[derive(Deserialize, Clone, Debug)]
struct SymbolPriceTicker{
    symbol: String,
    price: String,
}




pub struct SymbolPriceServer{
    sender: broadcast::Sender<Vec<SymbolPriceTicker>>
}

impl SymbolPriceServer {
    pub fn new() -> Self{
        let (tx, _rx1) = broadcast::channel(100);
        SymbolPriceServer { sender: tx }
    }

    pub async fn run(&self) -> Result<(), Box<dyn Error + Send + Sync>> {
        println!("SYMBOL-PRICE 数据服务已启动");
        //broadcast确保生产者发送消息时，消费者已经完成订阅
        /**************启动消费者任务**************/ 
        tokio::spawn(handler(self.sender.subscribe()));
        /**************启动生产者任务**************/ 
        tokio::spawn(get_coin_price(self.sender.clone()));

        // tokio::try_join!(handler, producer)?;
        println!("服务终止");
        Ok(())
    }
    
}


//
async fn get_coin_price(sender: broadcast::Sender<Vec<SymbolPriceTicker>>) -> Result<(), Box<dyn Error + Send + Sync>> {

    let api = "https://api.binance.com/api/v3/ticker/price";

    loop {
        
        let res = reqwest::get(api).await?;

        let tickers= res.json::<Vec<SymbolPriceTicker>>().await?;
        

        if let Err(e) = sender.send(tickers) {
            println!("没有活跃的消费者:{}",e);
        }
    }
    
    
}

async fn handler(mut receiver: broadcast::Receiver<Vec<SymbolPriceTicker>>) -> Result<(), Box<dyn Error + Send + Sync>> {
    loop {
        let tickers = receiver.recv().await?;

        for ticker in tickers.iter().filter(|t| t.symbol.starts_with("BTCUSDT")) {
            println!("{},{}",ticker.symbol, ticker.price);
        }
    }
    
    
    
}