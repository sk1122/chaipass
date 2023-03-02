use serde_json::{self, Value};
use tungstenite::{connect, Message};
use url::Url;

use super::block_receipts::{get_block_receipts, process_receipts};

pub async fn get_event() {
    // Connect to the WS server locally
    let (mut socket, _response) = connect(
        Url::parse("wss://eth-mainnet.g.alchemy.com/v2/eqE3zeVND3stKdCjZdLOqU62A2jg6eJc").unwrap(),
    )
    .expect("Can't connect");

    socket.write_message(Message::Text(serde_json::json!({"jsonrpc":"2.0","id": 2, "method": "eth_subscribe", "params": ["newHeads"]}).to_string())).unwrap();

    loop {
        let msg = socket.read_message().expect("Error reading message");
        
        let msg = match msg{
            tungstenite::Message::Text(s) => s,
            tungstenite::Message::Ping(_) => continue,
            _ => {
                panic!()
            }
        }; 
        let parsed: serde_json::Value = serde_json::from_str(&msg).expect("Can't parse to JSON");

        if parsed["params"]["result"]["number"] == Value::Null {
            continue;
        }

        println!("{:?}", parsed["params"]["result"]["number"]);
        
        let block_number = &parsed["params"]["result"]["number"].as_str().unwrap();

        println!("{}", block_number);
        let without_prefix = block_number.trim_start_matches("0x");
        let receipts = get_block_receipts(i64::from_str_radix(&without_prefix, 16).unwrap()).await;

        process_receipts(receipts).await;
    }
}