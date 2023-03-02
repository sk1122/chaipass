use ethers::{
    core::types::{Address, Filter},
    providers::Middleware, 
	types::{Log, U256},
	prelude::{EthEvent, EthLogDecode}
};


use super::{config::get_provider, execute::execute};

#[derive(Debug, Default, EthEvent, PartialEq, Eq)]
#[ethevent(name = "Transfer", abi = "Transfer(address,address,uint256)")]
pub struct Erc20TransferEvent {
    #[ethevent(indexed)]
    pub from: Address,
    #[ethevent(indexed)]
    pub to: Address,
    pub value: U256,
}

pub async fn get_block_receipts(block_number: i64) -> Vec<Log> {
	let provider = get_provider();

	let contract_address = "0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48";
	// let event_name = "SendMessage(bytes,bytes,string)";
	let event_name = "Transfer(address,address,uint256)";

	let filter = Filter::new().address(contract_address.parse::<Address>().unwrap()).event(event_name).from_block(block_number);

	let logs = provider.get_logs(&filter).await.unwrap();

	return logs;
}

pub async fn process_receipts(receipts: Vec<Log>) {
	for receipt in receipts {
		// let route_id = db.find();

		let parsed_data = <Erc20TransferEvent as EthLogDecode>::decode_log(&ethers::abi::RawLog {
			topics: receipt.topics,
			data: receipt.data.to_vec()
		}).unwrap();

		execute(parsed_data.from.to_string());
	}
}