extern crate reqwest;
extern crate serde;
extern crate serde_json;

#[derive(Serialize, Deserialize)]
pub struct ValidIsTrue {
	isvalid: bool,
}

#[derive(Serialize, Deserialize)]
pub struct ValidIsFalse {
	isvalid: bool,
	error: String,
}

pub fn validate_address(address: &str, coin: &str) -> String {
	use std::io::Read;
	let uri = format!("{}/validateAddress/{}/{}", super::SHAPESHIFT_URL, &address, &coin);
	let mut resp = reqwest::get(&uri).unwrap();
	assert!(resp.status().is_success());

	let mut content = String::new();
	resp.read_to_string(&mut content);

	if content.contains("false") {
		let v: ValidIsFalse = serde_json::from_str(&content).unwrap();
		let finish = format!("\nThis is not a valid address! Error message: {}", v.error);
		return finish	
	} else if content.contains("true") {
		let v: ValidIsTrue = serde_json::from_str(&content).unwrap();
		let finish = format!("\nThis is a valid address to withdraw funds");
		return finish
	} else {
		let finish = String::from("Something went wrong... Send an email to the author at Lsaether@protonmail.com");
		return finish
	}
}

// Returns booleans instead of string response. Could be useful
// for building other libraries. `use shapeshift::address::validate_addr(..)`
pub fn validate_addr(address: &str, coin: &str) -> bool {
	use std::io::Read;
	let uri = format!("{}/validateAddress/{}/{}", super::SHAPESHIFT_URL, &address, &coin);
	let mut resp = reqwest::get(&uri).unwrap();
	assert!(resp.status().is_success());

	let mut content = String::new();
	resp.read_to_string(&mut content);

	if content.contains("false") {
		println!("{}", &content);
		return false	
	} else if content.contains("true") {
		return true
	} else {
		println!("Something went wrong... Send an email to the author at Lsaether@protonmail.com");
		return false
	}
}