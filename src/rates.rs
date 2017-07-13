extern crate reqwest;
extern crate serde;
extern crate serde_json;

use std::fmt;

// Example usage 
// let r = shapeshift::rates::Rate::get_rate("btc_ltc")
// println!("{}", r);

#[derive(Serialize, Deserialize)]
pub struct Rate {
    pair: String,
    rate: String,
}

impl fmt::Display for Rate {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "\nShapeshift will shift {} at a rate of {}.",
			self.pair,
			self.rate)
	}
}

impl Rate {
	pub fn get_rate(pair: &str) -> Rate {
		use std::io::Read;

		let uri = format!("{}/rate/{}",
			super::SHAPESHIFT_URL,
			pair);

		let mut res = reqwest::get(&uri).unwrap();
		assert!(res.status().is_success());

		let mut content = String::new();
		res.read_to_string(&mut content);

		let r: Rate = serde_json::from_str(&content)
									.unwrap();
		r
	}
}

// Example usage 
// let r = shapeshift::rates::MarketInfo::get_info("btc_ltc")
// println!("{}", r);

#[derive(Serialize, Deserialize)]
pub struct MarketInfo {
	pair: String,
	rate: f32,
	minerFee: f32,
	limit: f32,
	minimum: f32,
	// TODO: maxLimit: f32,... What is this?
}

impl fmt::Display for MarketInfo {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "\nShapeshift market info for {}:\n
Rate: {}\n
Limit: {}\n
Minimum: {}\n
Miner Fee: {}",
			self.pair,
			self.rate,
			self.limit,
			self.minimum,
			self.minerFee)
	}
}

impl MarketInfo {
	pub fn get_info(pair: &str) -> MarketInfo {
		use std::io::Read;

		let uri = format!("{}/marketinfo/{}",
			super::SHAPESHIFT_URL,
			pair);

		let mut res = reqwest::get(&uri).unwrap();

		let mut content = String::new();
		res.read_to_string(&mut content);

		let m: MarketInfo = serde_json::from_str(&content)
											.unwrap();
		m
	}
}