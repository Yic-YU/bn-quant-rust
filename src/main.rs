use bn_btc_price::symbol_price_ticker::SymbolPriceServer;





#[tokio::main]
async fn main() {
    let server = SymbolPriceServer::new();

    // server.run().await;
    if let Err(e) = server.run().await{
        println!("{}",e);
    }
}
