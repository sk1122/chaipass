use crate::db::db::{setup_db, KVStore};
use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;

pub fn register_route(wasm_files: Vec<u8>) -> Option<String> {
    let db = setup_db();
    let route_id: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(30)
        .map(char::from)
        .collect();

    match db.save(route_id.as_str(), wasm_files) {
        true => {
            Some(route_id)
        },
        false => {
            None
        }
    }
}