extern crate reqwest;
extern crate serde;
extern crate serde_json;

#[derive(Serialize, Deserialize)]
pub struct txResponse {
	deposit: String,
	depositType: String,
	withdrawal: String,
	withdrawalType: String,	
}

pub fn shift(withdrawAddr: &str, pair: &str) -> String {
	use std::io::Read;
	use std::collections::HashMap;

	let uri = format!("{}/shift", super::SHAPESHIFT_URL);

	let mut post_request = HashMap::new();
	post_request.insert("withdrawal", &withdrawAddr);
	post_request.insert("pair", &pair);

	let client = reqwest::Client::new().unwrap();
	let mut resp = client.post(&uri).json(&post_request).send().unwrap();
	assert!(resp.status().is_success());

	let mut content = String::new();
	resp.read_to_string(&mut content);

	let tx: txResponse = serde_json::from_str(&content).unwrap();
	let finish = format!("\nSend your {} to Shapeshift address {}\n
Shapeshift will send {} to address {}\n
Type `shapeshift-rs status {}` to check status of your transaction",
		tx.depositType,
		tx.deposit,
		tx.withdrawalType,
		tx.withdrawal,
		tx.deposit);
	finish
}

pub fn shift_with_return_addr(withdrawAddr: &str, pair: &str, returnAddr: &str) -> String {
	use std::io::Read;
	use std::collections::HashMap;

	let uri = format!("{}/shift", super::SHAPESHIFT_URL);

	let mut post_request = HashMap::new();
	post_request.insert("withdrawal", &withdrawAddr);
	post_request.insert("pair", &pair);
	post_request.insert("returnAddress", &returnAddr);

	let client = reqwest::Client::new().unwrap();
	let mut resp = client.post(&uri).json(&post_request).send().unwrap();
	assert!(resp.status().is_success());

	let mut content = String::new();
	resp.read_to_string(&mut content);

	let tx: txResponse = serde_json::from_str(&content).unwrap();
	let finish = format!("\nSend your {} to Shapeshift address {}\n
Shapeshift will send {} to address {}\n
Type `shapeshift-rs status {}` to check status of your transaction",
		tx.depositType,
		tx.deposit,
		tx.withdrawalType,
		tx.withdrawal,
		tx.deposit);
	finish
}


#[derive(Serialize, Deserialize)]
pub struct emailResponse {
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

	let client = reqwest::Client::new().unwrap();
	let mut resp = client.post(&uri).json(&post_request).send().unwrap();
	assert!(resp.status().is_success());

	let mut content = String::new();
	resp.read_to_string(&mut content);

	let e: emailResponse = serde_json::from_str(&content).unwrap();
	let finish = format!("{}! {}.", e.email.status, e.email.message);
	finish
}

#[derive(Serialize, Deserialize)]
pub struct FixedTx {
	pair: String,
	withdrawal: String,
	withdrawalAmount: String,
	deposit: String,
	depositAmount: String,
	expiration: f32,
	quotedRate: String,
}

// {"success":{"orderId":"5dcb1d01-2861-4879-8751-2f757d053a02","pair":"btc_ltc","withdrawal":"LKJocimVE1xjES4364EFkfXKUs4xH1ZS3P","withdrawalAmount":"10","deposit":"1H6DgWw76KNAUni7bNxhNLpQRvnphuzKA8","depositAmount":"0.16199562","expiration":1498451266757,"quotedRate":"61.73623676","maxLimit":0.98873348,"returnAddress":"1Fu5HBe4FpkaF6cJM6M6cQLxjNv48n3Pwd","apiPubKey":"shapeshift","minerFee":"0.001"}}

#[derive(Serialize, Deserialize)]
pub struct FixedTxSuccess {
	success: FixedTx,
}

pub fn shift_fixed_amount(amount: &str, 
						  withdrawAddr: &str,
						  pair: &str) -> String {
	use std::io::Read;
	use std::collections::HashMap;

	let uri = format!("{}/sendamount", super::SHAPESHIFT_URL);

	let mut post_request = HashMap::new();
	post_request.insert("amount", &amount);
	post_request.insert("withdrawal", &withdrawAddr);
	post_request.insert("pair", &pair);

	let client = reqwest::Client::new().unwrap();
	let mut resp = client.post(&uri).json(&post_request).send().unwrap();
	assert!(resp.status().is_success());

	let mut content = String::new();
	resp.read_to_string(&mut content);

	// println!("{}", &content);
	let fTx: FixedTxSuccess = serde_json::from_str(&content).unwrap();
	let f = fTx.success;
	// TODO: convert the linux epoch f32 returned by shapeshift
	// into a time stamp for readability.
	let finish = format!("\nSend {} {} to Shapeshift address {}\n
Shapeshift will send {} {} to address {}\n
Quoted price: {}\n
Type `shapeshift-rs status {}` to check status of your transaction",
		f.depositAmount,
		&f.pair[0..3],
		f.deposit,
		// f.expiration,
		f.withdrawalAmount,
		&f.pair[4..7],
		f.withdrawal,
		f.quotedRate,
		f.deposit);
	finish
}

pub fn shift_fixed_amount_with_return_addr(amount: &str, 
						  withdrawAddr: &str,
						  pair: &str,
						  returnAddr: &str) -> String {
	use std::io::Read;
	use std::collections::HashMap;

	let uri = format!("{}/sendamount", super::SHAPESHIFT_URL);

	let mut post_request = HashMap::new();
	post_request.insert("amount", &amount);
	post_request.insert("withdrawal", &withdrawAddr);
	post_request.insert("pair", &pair);
	post_request.insert("returnAddress", &returnAddr);

	let client = reqwest::Client::new().unwrap();
	let mut resp = client.post(&uri).json(&post_request).send().unwrap();
	assert!(resp.status().is_success());

	let mut content = String::new();
	resp.read_to_string(&mut content);

	// println!("{}", &content);
	let fTx: FixedTxSuccess = serde_json::from_str(&content).unwrap();
	let f = fTx.success;
	// TODO: convert the linux epoch f32 returned by shapeshift
	// into a time stamp for readability.
	let finish = format!("\nSend {} {} to Shapeshift address {}\n
Shapeshift will send {} {} to address {}\n
Quoted price: {}\n
Type `shapeshift-rs status {}` to check status of your transaction",
		f.depositAmount,
		&f.pair[0..3],
		f.deposit,
		// f.expiration,
		f.withdrawalAmount,
		&f.pair[4..7],
		f.withdrawal,
		f.quotedRate,
		f.deposit);
	finish
}

#[derive(Serialize, Deserialize)]
pub struct PriceQuote {
	pair: String,
	withdrawalAmount: String,
	depositAmount: String,
	expiration: f32,
	quotedRate: String,
	minerFee: String,
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

	let client = reqwest::Client::new().unwrap();
	let mut resp = client.post(&uri).json(&post_request).send().unwrap();
	assert!(resp.status().is_success());

	let mut content = String::new();
	resp.read_to_string(&mut content);

	let q: PriceQuoteSuccess = serde_json::from_str(&content).unwrap();
	let q = q.success;
	let finish = format!("Pair: {}\nAmount you will receive: {}\nAmount to send: {}\nExpires: {}\nQuoted Rate: {}\nMiner Fee: {}",
		q.pair,
		q.withdrawalAmount,
		q.depositAmount,
		q.expiration,
		q.quotedRate,
		q.minerFee);
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

	let client = reqwest::Client::new().unwrap();
	let mut resp = client.post(&uri).json(&post_request).send().unwrap();
	assert!(resp.status().is_success());

	let mut content = String::new();
	resp.read_to_string(&mut content);

	let c: CancelResponse = serde_json::from_str(&content).unwrap();
	let finish = format!("{}", c.success);
	finish
}

#[derive(Serialize, Deserialize)]
pub struct StatusResponseComplete {
	status: String,
	address: String,
	withdraw: String,
	incomingCoin: String,
	incomingType: String,
	outgoingCoin: String,
	outgoingType: String,
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
	resp.read_to_string(&mut content);

	if content.contains("no_deposits") || content.contains("received") {
		let s: StatusResponse = serde_json::from_str(&content).unwrap();
		let finish = format!("\nGot status {} on transaction to address {}.", s.status, s.address);
		return finish
	} else if content.contains("error") {
		let s: StatusResponseError = serde_json::from_str(&content).unwrap();
		let finish = format!("\nError on address {}!! {}", s.address, s.error);
		return finish
	} else if content.contains("complete") {
		let s: StatusResponseComplete = serde_json::from_str(&content).unwrap();
		let finish = format!("\nGot status {} on transaction to address {}. You sent {} of {}. You got back {} of {} to address {}. Your transaction ID is {}.",
			s.status,
			s.address,
			s.incomingCoin,
			s.incomingType,
			s.outgoingCoin,
			s.outgoingType,
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
	resp.read_to_string(&mut content);

	let t: TimeRemaining = serde_json::from_str(&content).unwrap();
	let finish = format!("Received status {} on fixed amount transaction to address {}. You have {} seconds to complete the deposit.",
		t.status,
		&address,
		t.seconds_remaining);
	finish
}