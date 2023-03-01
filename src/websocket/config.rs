use ethers::{
	prelude::Provider,
	providers::Http
};

pub fn get_provider() -> Provider<Http> {
	Provider::<Http>::try_from("https://eth-goerli.g.alchemy.com/v2/eqE3zeVND3stKdCjZdLOqU62A2jg6eJc").expect("Can't connect https")
}