extern crate reqwest;
extern crate serde;
extern crate serde_json;

#[derive(Serialize, Deserialize)]
pub struct Rate {
    pair: String,
    rate: String,
}

pub fn get_rate(pair: &str) -> String {
	use std::io::Read;
	let uri = format!("{}/rate/{}", super::SHAPESHIFT_URL, pair);
	let mut resp = reqwest::get(&uri).unwrap();
	assert!(resp.status().is_success());

	let mut content = String::new();
	resp.read_to_string(&mut content);

	let r: Rate = serde_json::from_str(&content).unwrap();
	let finish = format!("Shapeshift will shift {} at a rate of {}.", r.pair, r.rate);
	finish
}

#[derive(Serialize, Deserialize)]
pub struct Limit {
	pair: String,
	limit: String,
}

pub fn get_limit(pair: &str) -> String {
	use std::io::Read;
	let uri = format!("{}/limit/{}", super::SHAPESHIFT_URL, pair);
	let mut resp = reqwest::get(&uri).unwrap();

	let mut content = String::new();
	resp.read_to_string(&mut content);

	let l: Limit = serde_json::from_str(&content).unwrap();
	let finish = format!("Shapeshift will shift {} with a limit of {}.", l.pair, l.limit);
	finish
}

#[derive(Serialize, Deserialize)]
pub struct MarketInfo {
	pair: String,
	rate: f32,
	minerFee: f32,
	limit: f32,
	minimum: f32,
	// TODO: maxLimit... What is this?
}

pub fn get_market_info_for_pair(pair: &str) -> String {
	use std::io::Read;
	let uri = format!("{}/marketinfo/{}", super::SHAPESHIFT_URL, pair);
	let mut resp = reqwest::get(&uri).unwrap();

	let mut content = String::new();
	resp.read_to_string(&mut content);

	let m: MarketInfo = serde_json::from_str(&content).unwrap();
	let finish = format!("Shapeshift market info for {}:\nRate: {}\nLimit: {}\nMinimum: {}\nMiner Fee: {}",
		m.pair,
		m.rate,
		m.limit,
		m.minimum,
		m.minerFee);
	finish
}

