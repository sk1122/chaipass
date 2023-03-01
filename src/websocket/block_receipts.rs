use std::str::FromStr;

use ethers::{
    core::types::{Address, Filter, H256},
    providers::{Middleware}, types::Log,
};

use super::config::get_provider;

pub async fn get_block_receipts(block_number: i64) -> Vec<Log> {
	let provider = get_provider();

	let contract_address = "0x8206b1da5ec144da2869f90dfb23e6e82b79bb25";
	// let event_name = "SendMessage(bytes,bytes,string)";
	let event_name = "Transfer(address,address,uint256)";

	let filter = Filter::new().address(contract_address.parse::<Address>().unwrap()).event(event_name).from_block(block_number);

	let logs = provider.get_logs(&filter).await.unwrap();

	return logs;
}

pub async fn process_receipts(receipts: Vec<Log>) {
	// let provider = get_provider();

	for receipt in receipts {
		if receipt.topics[0].eq(&H256::from_str("0xddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef").unwrap()) {
			println!("found")
		}
	}
}