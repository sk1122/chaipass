pub fn initialize_rocksdb() {
	let mut options = rocksdb::Options::default();
	options.set_error_if_exists(false);
	options.create_if_missing(true);
	options.create_missing_
}