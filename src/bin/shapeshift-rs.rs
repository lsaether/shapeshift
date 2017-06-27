extern crate clap;
extern crate term;

use clap::{Arg, App, SubCommand};
use std::io;
use std::io::prelude::*;

extern crate shapeshift;

const APP_NAME: &'static str = "shapeshift-rs";
const AUTHOR:   &'static str = "Logan Saether, @lsaether";
const VERSION:  &'static str = "0.1.0";
const ABOUT:    &'static str = "Command line interface for the Shapeshift API.";

fn main() {
    /// Wraps the terminal so you can colorize the output.
    let mut t = term::stdout().unwrap();

    t.fg(term::color::BRIGHT_GREEN).unwrap();

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
                              .about("Change one crypto asset into another.")
                              .arg(Arg::with_name("withdrawal_address")
                                        .index(1)
                                      	.required(false)
                                      	.help("Address you would like to withdraw into."))
                              .arg(Arg::with_name("pair")
                                      	.index(2)
                                      	.required(false)
                                      	.help("Pair of token assets in required format, ie `btc_ltc`."))
                              .arg(Arg::with_name("return")
                                      	.index(3)
                                      	.required(false)
                                      	.help("Optional: Return address if transaction fails."))
                              .arg(Arg::with_name("fixed")
                                        .short("f")
                                        .long("fixed")
                                        .required(false)
                                        .help("Flag for fixed amount transaction.")))

                        .subcommand(SubCommand::with_name("status")
                                  .about("Get status on pending shift.")
                                  .arg(Arg::with_name("address")
                                        .index(1)
                                        .required(true)
                                        .help("The address Shapeshift provided to send your funds to.")))

                        // REPLACE THE BELOW WITH:
                        // .subcommand(SubCommand::with_name("market-info"))

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

    // shapeshift _validate
    if let Some(ref matches) = matches.subcommand_matches("validate") {
    	println!("Enter an address you would like to validate.");
    	let addr = read_input();
    	println!("Enter the token asset associated with address {}", &addr);
    	let asset = read_input();
    	println!("Making your request...");
    	let response = shapeshift::address::validate_address(&addr, &asset);
    	println!("{}", &response);
    }

    // shapeshift validate <address> <coin>
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

  // shapeshift shift <withdrawal_address> <pair> <return_address>
  if let Some(ref matches) = matches.subcommand_matches("shift") {
    let w_addr: String;
    let pair: String;
    let mut amount: String = String::from("0");
    let mut fixed = false;

    if matches.is_present("fixed") {
      fixed = true;
      println!("Enter the amount of asset you would like to receive:");
      amount = read_input();
    }

    if !matches.is_present("withdrawal_address") {
      println!("\nEnter a valid withdraw address:");
      w_addr = read_input();
    } else { w_addr = String::from(matches.value_of("withdrawal_address").unwrap()); }

    if !matches.is_present("pair") {
      println!("\nEnter the pair you would like to shift in required format [from_to]:");
      pair = read_input(); 
    } else { pair = String::from(matches.value_of("pair").unwrap()); }

    if !matches.is_present("return") {
      println!("\nEnter a return address: [Press enter to skip.]");
      let r_addr = read_input();
      if r_addr.is_empty() {

        t.reset().unwrap();
        t.fg(term::color::MAGENTA).unwrap();

        if fixed {
          let response = shapeshift::transaction::shift_fixed_amount(&amount, &w_addr, &pair);
          println!("{}", &response);
        } else {
          let response = shapeshift::transaction::shift(&w_addr, &pair);
          println!("{}", &response);
        }
      } else {
        if fixed {
          let response = shapeshift::transaction::shift_fixed_amount_with_return_addr(&amount, &w_addr, &pair, &r_addr);
          println!("{}", response);
        } else {
          let response = shapeshift::transaction::shift_with_return_addr(&w_addr, &pair, &r_addr);
          println!("{}", &response);
        }
      }
    }
  }

  if let Some(ref matches) = matches.subcommand_matches("status") {
    let s_addr = matches.value_of("address").unwrap();
    println!("{}", shapeshift::transaction::get_tx_status(&s_addr));
  }
}

fn read_input() -> String {
    let mut input = String::new();

    io::stdin().read_line(&mut input);

    let user_input = String::from(input.trim());
    user_input
}