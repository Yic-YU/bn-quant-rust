use bn_btc_price::{symbol_price_ticker::SymbolPriceServer, websocket_server::WebSocketServer};





#[tokio::main]
async fn main() {
    // let server = SymbolPriceServer::new();

    // // server.run().await;
    // if let Err(e) = server.run().await{
    //     println!("{}",e);
    // }
    let ws_server = WebSocketServer::new();
    if let Err(e) = ws_server.run().await {
        println!("{}", e);
    }

}
