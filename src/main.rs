extern crate clap;
use clap::{Arg, App, SubCommand};
use std::io;

extern crate shapeshift;

const APP_NAME: &'static str = "shapeshift-rs";
const AUTHOR:   &'static str = "Logan Saether, @lsaether";
const VERSION:  &'static str = "0.1.0";
const ABOUT:    &'static str = "Command line interface for the Shapeshift API.";

fn main() {
    let matches = App::new(APP_NAME)
                        .version(VERSION)
                        .author(AUTHOR)
                        .about(ABOUT)
                        .subcommand(SubCommand::with_name("validate")
                          			.about("Validate an address for coin."))

                        .subcommand(SubCommand::with_name("_validate")
                          			.about("Validate an address for coin. (PRO MODE)")
                        			.arg(Arg::with_name("address")
                          			  		.index(1)
                          			  		.required(true)
                          			  		.help("Address you wish to validate."))
                          			.arg(Arg::with_name("coin")
                          			  		.index(2)
                          			  		.required(true)
                          			  		.help("Token asset associated with the address.")))

                        .subcommand(SubCommand::with_name("shift")
                        			.about("Change one crypto asset into another."))

                        // .subcommand(SubCommand::with_name("_shift")
                        //             .about("Change one coin into another.")
                        //             .arg(Arg::with_name("withdrawal_address")
                        //               		.index(1)
                        //               		.required(true)
                        //               		.help("Address you would like to withdraw into."))
                        //             .arg(Arg::with_name("pair")
                        //               		.index(2)
                        //               		.required(true)
                        //               		.help("Pair of token assets in required format, ie `btc_ltc`."))
                        //             .arg(Arg::with_name("return")
                        //             		.short("r")
                        //             		.long("return")
                        //             		.takes_value(true)
                        //             		.multiple(false)
                        //               		.index(3)
                        //               		.required(false)
                        //               		.help("Optional: Return address if transaction fails.")))

                        .subcommand(SubCommand::with_name("rates")
                          			.about("Get market info for pair."))

                        .subcommand(SubCommand::with_name("_rates")
                          			.about("Get market info for pair. (PRO MODE)")
                          			.arg(Arg::with_name("pair")
                          			  		.index(1)
                          			  		.required(true)
                          			  		.help("Pair of token assets in required format, ie `btc_ltc`."))
                          			.arg(Arg::with_name("market_info")
                          			  	    .short("i")
                          			  	    .long("info")
                          			  	    .help("Present full market info for the pair.")))
                        .get_matches();

    // shapeshift validate <address> <coin>
    if let Some(ref matches) = matches.subcommand_matches("validate") {
    	println!("Enter an address you would like to validate.");
    	let addr = read_input();
    	println!("Enter the token asset associated with address {}", &addr);
    	let asset = read_input();
    	println!("Making your request...");
    	let response = shapeshift::address::validate_address(&addr, &asset);
    	println!("{}", &response);
    }

    if let Some(ref matches) = matches.subcommand_matches("_validate") {
    	let addr = matches.value_of("address").unwrap();
    	let coin = matches.value_of("coin").unwrap();
    	let response = shapeshift::address::validate_address(addr, coin);
    	println!("{}", response);
    }

    // shapeshift rates
    if let Some(ref matches) = matches.subcommand_matches("rates") {
     	println!("Enter a valid pair in required format, ie `eth_btc`.");
     	let pair = read_input();
     	let response = shapeshift::rates::get_market_info_for_pair(&pair);
     	println!("{}", &response);
    }

    // shapeshift _rates [-i] <pair>
    // flag `-i` returns full market info
    if let Some(ref matches) = matches.subcommand_matches("_rates") {
     	if matches.is_present("market_info") {
     		println!("{}", shapeshift::rates::get_market_info_for_pair(matches.value_of("pair").unwrap()));
     		return 
     	}
        println!("{}", shapeshift::rates::get_rate(matches.value_of("pair").unwrap()));
    }
    
    // shapeshift shift (MAIN FUNCTION)
    if let Some(ref matches) = matches.subcommand_matches("shift") {
    	println!("Do you need a fixed amount? [Yes/No]");
    	let answer = read_input().to_lowercase();
    	let yes = String::from("yes");
    	let no = String::from("no");

    	if &answer == &yes {
    		command_line_shift_fixed_amount();
    	} else if &answer == &no {
    		command_line_shift();
    	} else {
    		println!("Couldn't read response.");
    	}
    }

    // // shapeshift _shift <withdrawal_address> <pair> -r <return_address>
    // if let Some(ref matches) = matches.subcommand_matches("_shift") {
    // 	let w_addr = matches.value_of("withdrawal_address").unwrap();
    // 	let pair = matches.value_of("pair").unwrap();
    // 	let r_addr = matches.value_of("return").unwrap();
    // 	let response = format!("{}...{}...{}", w_addr, pair, r_addr);
    // 	// let response = shapeshift::transaction::shift(w_addr, pair, "0x7E28e5977Ff55A9785A9AB22DfBcccEACe70B860");
    // 	println!("{}", response);
    // }

}

fn read_input() -> String {
    let mut input = String::new();

    io::stdin().read_line(&mut input);

    let user_input = String::from(input.trim());
    user_input
}

fn command_line_shift_fixed_amount() {
	println!("Nothing.");
}

fn command_line_shift() {
	println!("Enter the address you would like to withdraw into.");
	let w_addr = read_input();
	println!("Enter the return address (in case something goes wrong).");
	let r_addr = read_input();
	println!("Enter the pair you would like to shift in required format, ie `btc_eth`.");
	let pair = read_input();
	println!("Make sure this information is correct and if so type `1` and hit enter.");
	println!("You are shifting pair (from_into) {}. The funds will go into your account at address {}. If something goes wrong, funds will be returned to address {}.", &pair, &w_addr, &r_addr);
	let check = read_input();
	if check != "1" {
		println!("Try again!");
	}
	let response = shapeshift::transaction::shift(&w_addr, &pair, &r_addr);
	println!("{}", &response);
}
// 	println!("
// Welcome to the Shapeshift.io Rust language API.\n
// Authored by: Logan Saether @lsaether\n
// ***********************************************\n
// I am not affliated with Shapeshift.io in any way.\n
// ***********************************************\n\n\n
// 	");