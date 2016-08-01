extern crate regex;

use regex::Regex;

fn main() {
    let regexp = Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap();
    println!("Did our date match? {}", regexp.is_match("2014-01-01"));
}
