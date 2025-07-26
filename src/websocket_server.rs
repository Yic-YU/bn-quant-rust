use futures_util::{StreamExt, SinkExt};
use serde::Deserialize;
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use url::Url;
use std::error::Error;
use std::time::Duration;
use tokio::time::sleep;

// 根据文档修正Trade结构体，移除了不存在的 b 和 a 字段
#[derive(Deserialize, Debug)]
#[allow(non_snake_case)]
struct Trade {
    e: String, // 事件类型
    E: u64,    // 事件时间
    s: String, // 交易对
    t: u64,    // 交易ID
    p: String, // 成交价格
    q: String, // 成交数量
    T: u64,    // 成交时间
    m: bool,   // 买方是否是做市方
    M: bool,   // 请忽略该字段
}

pub struct WebSocketServer;

impl WebSocketServer {
    pub fn new() -> Self {
        WebSocketServer
    }

    pub async fn run(&self) -> Result<(), Box<dyn Error + Send + Sync>> {
        println!("WEBSOCKET-BTC-USDT-TRADE 数据服务已启动");

        let url = Url::parse("wss://stream.binance.com:9443/ws/btcusdt@trade").unwrap();

        // 使用循环来实现断线重连
        loop {
            println!("正在尝试连接到: {}...", url);
            
            // 尝试连接
            match connect_async(url.as_str()).await {
                Ok((ws_stream, response)) => {
                    println!("✅ WebSocket 握手成功.");
                    println!("✅ HTTP Response: {:?}", response.status());

                    let (mut write, mut read) = ws_stream.split();

                    // 循环处理接收到的消息
                    while let Some(message) = read.next().await {
                        match message {
                            Ok(msg) => {
                                // **新增：处理Ping/Pong心跳**
                                if let Message::Ping(ping_data) = msg {
                                    println!("收到 Ping 帧, 发送 Pong 作为响应...");
                                    if let Err(e) = write.send(Message::Pong(ping_data)).await {
                                        println!("❌ 发送 Pong 失败: {}", e);
                                        break; // 发送失败，跳出内层循环以重连
                                    }
                                    continue;
                                }

                                if let Message::Text(text) = msg {
                                    println!("收到原始文本: {}", text); // 打印原始数据
                                    
                                    // 尝试反序列化
                                    match serde_json::from_str::<Trade>(&text) {
                                        Ok(trade) => println!("处理后的交易数据: {:?}", trade),
                                        Err(e) => println!("❌ 反序列化交易数据失败: {}", e),
                                    }
                                } else if let Message::Close(c) = msg {
                                     println!("收到 Close 帧: {:?}", c);
                                     break; // 收到关闭帧，跳出以重连
                                }
                            },
                            Err(e) => {
                                println!("❌ 接收消息时发生错误: {}", e);
                                break; // 发生错误，跳出以重连
                            }
                        }
                    }
                },
                Err(e) => {
                    println!("❌ 连接WebSocket失败: {}", e);
                }
            }
            
            // 如果连接断开或失败，等待5秒后重试
            println!("连接已断开，5秒后将尝试重新连接...");
            sleep(Duration::from_secs(5)).await;
        }
    }
}