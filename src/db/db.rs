use rocksdb::DB;
use std::sync::Arc;

pub trait KVStore {
	fn init(file_path: &str) -> Self;
	fn save(&self, k: &str, v: Vec<u8>) -> bool;
	fn find(&self, k: &str) -> Option<Vec<u8>>;
	fn delete(&self, k: &str) -> bool;
}

#[derive(Clone)]
pub struct RocksDB {
	db: Arc<DB>,
}

impl KVStore for RocksDB {
	fn init(file_path: &str) -> Self {
		RocksDB {
			db: Arc::new(DB::open_default(file_path).unwrap())
		}
	}

	fn save(&self, k: &str, v: Vec<u8>) -> bool {
		self.db.put(k.as_bytes(), v).is_ok()
	}

	fn find(&self, k: &str) -> Option<Vec<u8>> {
		match self.db.get(k.as_bytes()) {
			Ok(Some(v)) => {
				// let result = String::from_utf8(v).unwrap();
				Some(v)
			},
			Ok(None) => {
				None
			},
			Err(_) => {
				None
			}
		}
	}

	fn delete(&self, k: &str) -> bool {
		self.db.delete(k.as_bytes()).is_ok()
	}
}

pub fn setup_db() -> RocksDB {
	KVStore::init("./tmp")
}