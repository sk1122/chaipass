use assme2::websocket::{websocket::get_event, block_receipts::{get_block_receipts, process_receipts}};

// 1. get event from websocket
// 2. parse event and get routeId
// 3. if routeId is in rocksDB, spawn a new thread to execute wasm
// 4. after executing wasm, send signal to relayer to relay that transaction

#[tokio::main]
async fn main() {
    // get_event().await;
    let logs = get_block_receipts(8578133).await;
    println!("{:?}", logs);
    process_receipts(logs).await;
}