extern crate shapeshift;

fn main() {
	let t = shapeshift::transaction::shift_fixed_amount("10", "LKJocimVE1xjES4364EFkfXKUs4xH1ZS3P", "btc_ltc", "1Fu5HBe4FpkaF6cJM6M6cQLxjNv48n3Pwd");
	println!("{}", t);
// 	use std::env;

// 	let args: Vec<String> = env::args().collect();

// 	println!("
// Welcome to the Shapeshift.io Rust language API.\n
// Authored by: Logan Saether @lsaether\n
// ***********************************************\n
// I am not affliated with Shapeshift.io in any way.\n
// ***********************************************\n\n\n
// 	");
	
}

fn examples() {
	println!("Usage examples:\n
***********************************************
Get current rates that shapeshift offers on pair of coins.\n
EX. Bitcoin -> Litecoin\n
shapeshift::rates::get_rate(\"btc_ltc\");\n
Returns:
	");
	println!("{}", shapeshift::rates::get_rate("btc_ltc"));
	println!("***********************************************\n
***********************************************
Get total market info on a pair of coins.\n 
EX. Bitcoin -> Ether\n 
shapeshift::rates::get_market_info_for_pair(\"btc_eth\");\n 
Returns:
	");
	println!("{}", shapeshift::rates::get_market_info_for_pair("btc_eth"));
	println!("***********************************************\n");
}