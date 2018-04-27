#[macro_use]
extern crate serde_derive;

#[warn(unused_must_use)]
pub mod address;
pub mod rates;
pub mod transaction;

pub const SHAPESHIFT_URL: &'static str = "https://shapeshift.io";

pub const AUTHOR: &'static str = "Logan Saether";
pub const AUTHOR_GITHUB: &'static str = "@lsaether";
pub const AUTHOR_EMAIL: &'static str = "Lsaether@protonmail.com";
