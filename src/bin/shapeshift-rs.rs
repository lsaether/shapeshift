extern crate clap;
extern crate term;

use clap::{Arg, App, SubCommand};
use std::io;

extern crate shapeshift;

const APP_NAME: &'static str = "shapeshift-rs";
const AUTHOR:   &'static str = "Logan Saether, @lsaether";
const VERSION:  &'static str = "0.1.0";
const ABOUT:    &'static str = "Command line interface for the Shapeshift API.";

fn main() {
    // Wraps the terminal so you can colorize the output.
    let mut t = term::stdout().unwrap();

    t.fg(term::color::BRIGHT_GREEN).unwrap();

    let matches = App::new(APP_NAME)
                        .version(VERSION)
                        .author(AUTHOR)
                        .about(ABOUT)
                        .subcommand(SubCommand::with_name("validate")
                          		.about("Check address is valid for withdrawal.")
                        			.arg(Arg::with_name("address")
                          			  		.index(1)
                          			  		.required(true)
                          			  		.help("Address you would like to validate."))
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

                        .subcommand(SubCommand::with_name("market-info")
                          		.about("Get market info for pair.")
                          		.arg(Arg::with_name("pair")
                          			  		.index(1)
                          			  		.required(true)
                          			  		.help("Pair of token assets in required format, ie `btc_ltc`.")))

                        .subcommand(SubCommand::with_name("get-coins")
                              .about("Returns list of coins and their availability."))
                        .get_matches();

  // shapeshift-rs validate <address> <coin>
  if let Some(ref matches) = matches.subcommand_matches("validate") {
    let addr = matches.value_of("address").unwrap();
    let coin = matches.value_of("coin").unwrap();
    let response = shapeshift::address::validate_address(addr, coin);
    println!("{}", response);
  }

  // shapeshift-rs market-info <pair>
  if let Some(ref matches) = matches.subcommand_matches("market-info") {
    let pair = matches.value_of("pair").unwrap();
    let response = shapeshift::rates::MarketInfo::get_info(&pair);
    println!("{}", &response);
  }

  // shapeshift-rs shift [-f] <withdrawal_address> <pair> <return_address>
  if let Some(ref matches) = matches.subcommand_matches("shift") {
    let w_addr: String;
    let pair: String;
    let mut amount: String = String::from("0");
    let mut fixed = false;

    if matches.is_present("fixed") {
      fixed = true;
      println!("\nEnter the amount of asset you would like to receive:");
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

        if fixed {
          let response = shapeshift::transaction
                                   ::FxTx
                                   ::shift(&amount, &w_addr, &pair, "");
          println!("{}", &response);
        } else {
          let response = shapeshift::transaction
                                   ::Tx
                                   ::shift(&w_addr, &pair, "");
          println!("{}", &response);
        }

      } else {
        if fixed {
          let response = shapeshift::transaction
                                   ::FxTx
                                   ::shift(&amount, &w_addr, &pair, &r_addr);
          println!("{}", response);
        } else {
          let response = shapeshift::transaction
                                   ::Tx
                                   ::shift(&w_addr, &pair, &r_addr);
          println!("{}", &response);
        }
      }
    }
  }

  // shapeshift-rs status <address>
  if let Some(ref matches) = matches.subcommand_matches("status") {
    let s_addr = matches.value_of("address").unwrap();
    println!("{}", shapeshift::transaction::get_tx_status(&s_addr));
  }

  // shapeshift-rs get-coins
  if let Some(ref _matches) = matches.subcommand_matches("get-coins") {
    println!("NOT YET AVAILABLE");
  }
}

fn read_input() -> String {
    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();

    let user_input = String::from(input.trim());
    user_input
}
