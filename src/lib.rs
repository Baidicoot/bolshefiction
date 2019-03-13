#[macro_use]
extern crate lazy_static;
extern crate regex;
extern crate markov_chain;

use std::fs;
use regex::Regex;
use markov_chain::Chain;
use std::boxed::Box;

pub type MarxovChain = Chain<u8>;

lazy_static! {
    static ref SENT: Regex = Regex::new(r"[A-Z].*?\.").unwrap();
}

pub fn train(order: usize) -> MarxovChain {
    let text = fs::read("manifesto.txt").unwrap();
    let mut chain = Chain::new(order);

    chain.train(text);

    chain
}

pub fn get(chain: &MarxovChain) -> Option<String> {
    let data = &chain.generate();
    let string = String::from_utf8_lossy(data);

    match SENT.captures(&string) {
        Some(x) => Some(x[0].to_owned()),
        None => None,
    }
}