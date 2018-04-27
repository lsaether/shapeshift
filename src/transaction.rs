extern crate reqwest;
extern crate serde;
extern crate serde_json;

use std::fmt;

#[derive(Serialize, Deserialize)]
pub struct Tx {
	deposit: String,
	#[serde(rename = "depositType")]
	deposit_type: String,
	withdrawal: String,
	#[serde(rename = "withdrawalType")]
	withdrawal_type: String,
}

impl fmt::Display for Tx {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "\nSend your {} to shapeshift address {}\n
Shapeshift will send {} to address {}\n
Type `shapeshift-rs status {}` to check status of transaction.",
			self.deposit_type,
			self.deposit,
			self.withdrawal_type,
			self.withdrawal,
			self.deposit)
	}
}

impl Tx {
	pub fn shift(waddr: &str, pair: &str, raddr: &str) -> Tx {
		use std::io::Read;
		use std::collections::HashMap;

		let uri = format!("{}/shift",
			super::SHAPESHIFT_URL);

		let mut post = HashMap::new();
		post.insert("withdrawal", &waddr);
		post.insert("pair", &pair);
		if !raddr.is_empty(){
			post.insert("returnAddress", &raddr);
		}

		// Some client magic to do a post request.
		let client = reqwest::Client::new();
		let mut res = client.post(&uri)
							.json(&post)
							.send()
							.unwrap();
		// No failures getting through here.
		assert!(res.status().is_success());

		// Make an empty string.
		let mut content = String::new();
		// Fill it with our data!
		res.read_to_string(&mut content).unwrap();

		let t: Tx = serde_json::from_str(&content).unwrap();

		t
	}
}

// Unfortunately I see no other way to implement the fixed
// transaction than doing the whole process over again, since
// it takes another arg in the post request and returns
// a json response with more data fields.

// {"success":{"orderId":"5dcb1d01-2861-4879-8751-2f757d053a02","pair":"btc_ltc","withdrawal":"LKJocimVE1xjES4364EFkfXKUs4xH1ZS3P","withdrawalAmount":"10","deposit":"1H6DgWw76KNAUni7bNxhNLpQRvnphuzKA8","depositAmount":"0.16199562","expiration":1498451266757,"quotedRate":"61.73623676","maxLimit":0.98873348,"returnAddress":"1Fu5HBe4FpkaF6cJM6M6cQLxjNv48n3Pwd","apiPubKey":"shapeshift","minerFee":"0.001"}}


// Fixed Transaction
#[derive(Serialize, Deserialize)]
pub struct FxTx {
	pair: String,
	deposit: String,
	#[serde(rename = "depositAmount")]
	deposit_amount: String,
	withdrawal: String,
	#[serde(rename = "withdrawalAmount")]
	withdrawal_amount: String,
	expiration: f32,
	#[serde(rename = "quotedRate")]
	quoted_rate: String,
}

// Internal struct needed for nested JSON
#[derive(Serialize, Deserialize)]
struct FxTxS {
	success: FxTx,
}

impl fmt::Display for FxTx {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "\nSend {} {} to Shapeshift address {}\n
Shapeshift will send {} {} to address {}\n
Quoted price: {}\n
Type `shapeshift-rs status {}` to check status of your transaction",
			self.deposit_amount,
			&self.pair[0..3],
			self.deposit,
			self.withdrawal_amount,
			&self.pair[4..7],
			self.withdrawal,
			self.quoted_rate,
			self.deposit)
	}
}

impl FxTx {
	pub fn shift(amt: &str,
				waddr: &str,
				pair: &str,
				raddr: &str) -> FxTx {

		use std::io::Read;
		use std::collections::HashMap;

		let uri = format!("{}/sendamount",
			super::SHAPESHIFT_URL);

		let mut post = HashMap::new();
		post.insert("amount", &amt);
		post.insert("withdrawal", &waddr);
		post.insert("pair", &pair);
		if !raddr.is_empty(){
			post.insert("returnAddress", &raddr);
		}

		// Some client magic to do a post request.
		let client = reqwest::Client::new();
		let mut res = client.post(&uri)
							.json(&post)
							.send()
							.unwrap();
		// No failures getting through here.
		assert!(res.status().is_success());

		// Make an empty string.
		let mut content = String::new();
		// Fill it with our data!
		res.read_to_string(&mut content).unwrap();

		// A stupid step because API returns a nested JSON
		// and I don't know how to work with nested JSON
		// in Rust
		let fxtxs: FxTxS = serde_json::from_str(&content).unwrap();
		// Now unwrap it how we should!
		let f: FxTx = fxtxs.success;
		// And return it
		f
	}
}

#[derive(Serialize, Deserialize)]
#[serde(rename = "emailResponse")]
pub struct EmailResponse {
	email: Email,
}

#[derive(Serialize, Deserialize)]
pub struct Email {
	status: String,
	message: String,
}

pub fn request_email_receipt(email: &str, withdraw_txid: &str) -> String {
	use std::io::Read;
	use std::collections::HashMap;

	let uri = format!("{}/mail", super::SHAPESHIFT_URL);

	let mut post_request = HashMap::new();
	post_request.insert("email", &email);
	post_request.insert("txid", &withdraw_txid);

	let client = reqwest::Client::new();
	let mut resp = client.post(&uri).json(&post_request).send().unwrap();
	assert!(resp.status().is_success());

	let mut content = String::new();
	resp.read_to_string(&mut content).unwrap();

	let e: EmailResponse = serde_json::from_str(&content).unwrap();
	let finish = format!("{}! {}.", e.email.status, e.email.message);
	finish
}

#[derive(Serialize, Deserialize)]
pub struct PriceQuote {
	pair: String,
	#[serde(rename = "withdrawalAmount")]
	withdrawal_amount: String,
	#[serde(rename = "depositAmount")]
	deposit_amount: String,
	expiration: f32,
	#[serde(rename = "quotedRate")]
	quoted_rate: String,
	#[serde(rename = "minerFee")]
	miner_fee: String,
}

#[derive(Serialize, Deserialize)]
pub struct PriceQuoteSuccess {
	success: PriceQuote,
}

pub fn get_price_quote(amount: &str, pair: &str) -> String {
	use std::io::Read;
	use std::collections::HashMap;

	let uri = format!("{}/sendamount", super::SHAPESHIFT_URL);

	let mut post_request = HashMap::new();
	post_request.insert("amount", &amount);
	post_request.insert("pair", &pair);

	let client = reqwest::Client::new();
	let mut resp = client.post(&uri).json(&post_request).send().unwrap();
	assert!(resp.status().is_success());

	let mut content = String::new();
	resp.read_to_string(&mut content).unwrap();

	let q: PriceQuoteSuccess = serde_json::from_str(&content).unwrap();
	let q = q.success;
	let finish = format!("Pair: {}\nAmount you will receive: {}\nAmount to send: {}\nExpires: {}\nQuoted Rate: {}\nMiner Fee: {}",
		q.pair,
		q.withdrawal_amount,
		q.deposit_amount,
		q.expiration,
		q.quoted_rate,
		q.miner_fee);
	finish
}

#[derive(Deserialize, Serialize)]
pub struct CancelResponse {
	success: String,
}

pub fn cancel_pending_tx(address: &str) -> String {
	use std::io::Read;
	use std::collections::HashMap;

	let uri = format!("{}/cancelpending", super::SHAPESHIFT_URL);

	let mut post_request = HashMap::new();
	post_request.insert("address", &address);

	let client = reqwest::Client::new();
	let mut resp = client.post(&uri).json(&post_request).send().unwrap();
	assert!(resp.status().is_success());

	let mut content = String::new();
	resp.read_to_string(&mut content).unwrap();

	let c: CancelResponse = serde_json::from_str(&content).unwrap();
	let finish = format!("{}", c.success);
	finish
}

#[derive(Serialize, Deserialize)]
pub struct StatusResponseComplete {
	status: String,
	address: String,
	withdraw: String,
	#[serde(rename = "incomingCoin")]
	incoming_coin: String,
	#[serde(rename = "incomingType")]
	incoming_type: String,
	#[serde(rename = "outgoingCoin")]
	outgoing_coin: String,
	#[serde(rename = "outgoingType")]
	outgoing_type: String,
	transaction: String,
}

#[derive(Serialize, Deserialize)]
pub struct StatusResponseError {
	status: String,
	address: String,
	error: String,
}

#[derive(Serialize, Deserialize)]
pub struct StatusResponse {
	status: String,
	address: String,
}

pub fn get_tx_status(address: &str) -> String {
	use std::io::Read;
	let uri = format!("{}/txStat/{}", super::SHAPESHIFT_URL, &address);
	let mut resp = reqwest::get(&uri).unwrap();
	assert!(resp.status().is_success());

	let mut content = String::new();
	resp.read_to_string(&mut content).unwrap();

	if content.contains("no_deposits") || content.contains("received") {
		let s: StatusResponse = serde_json::from_str(&content).unwrap();
		let finish = format!("\nGot status {} on transaction to address {}", s.status, s.address);
		return finish
	} else if content.contains("error") {
		let s: StatusResponseError = serde_json::from_str(&content).unwrap();
		let finish = format!("\nError on address {} !!! {}", s.address, s.error);
		return finish
	} else if content.contains("complete") {
		let s: StatusResponseComplete = serde_json::from_str(&content).unwrap();
		let finish = format!("\nGot status {} on transaction to address {}. You sent {} of {}. You got back {} of {} to address {}. Your transaction ID is {}.",
			s.status,
			s.address,
			s.incoming_coin,
			s.incoming_type,
			s.outgoing_coin,
			s.outgoing_type,
			s.withdraw,
			s.transaction);
		return finish
	} else {
		let finish = String::from("Something went wrong... Send an email to the author at Lsaether@protonmail.com");
		return finish
	}
}

#[derive(Serialize, Deserialize)]
pub struct TimeRemaining {
	status: String,
	seconds_remaining: String,
}

pub fn get_time_remaining(address: &str) -> String {
	use std::io::Read;
	let uri = format!("{}/timeremaining/{}", super::SHAPESHIFT_URL, &address);
	let mut resp = reqwest::get(&uri).unwrap();
	assert!(resp.status().is_success());

	let mut content = String::new();
	resp.read_to_string(&mut content).unwrap();

	let t: TimeRemaining = serde_json::from_str(&content).unwrap();
	let finish = format!("Received status {} on fixed amount transaction to address {}. You have {} seconds to complete the deposit.",
		t.status,
		&address,
		t.seconds_remaining);
	finish
}
