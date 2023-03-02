use assme2::{websocket::{websocket::get_event, block_receipts::{get_block_receipts, process_receipts}}, api::api::register_route, db::db::{setup_db, KVStore}};

// 1. get event from websocket
// 2. parse event and get routeId
// 3. if routeId is in rocksDB, spawn a new thread to execute wasm
// 4. after executing wasm, send signal to relayer to relay that transaction

#[tokio::main]
async fn main() {
    get_event().await;
}