use ethers::types::{Transaction, Block};
use wasmer::{Store, Module, imports, Instance, Value};

use crate::db::db::{setup_db, KVStore};

pub fn execute(route_id: String, transaction: Transaction, block_header: Block<Transaction>) {
    let db = setup_db();

    let wasm_byte_code = db.find(route_id.as_str()).unwrap();

    let mut store = Store::default();
    let module = Module::new(&store, &wasm_byte_code).unwrap();
    let import_object = imports! {};
    let instance = Instance::new(&mut store, &module, &import_object).unwrap();

    let run = instance.exports.get_function("run").unwrap();
    // let result = run.call(&mut store, &[Value::V128(u128::from_ne_bytes(transaction.rlp().as_chunks::<16>().0[0])), block_header]).unwrap();
}