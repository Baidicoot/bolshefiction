#[macro_use]
extern crate lazy_static;
extern crate regex;
extern crate markov_chain;

use std::fs;
use regex::Regex;
use markov_chain::Chain;

pub type MarxovChain = Chain<char>;

lazy_static! {
    static ref SENT: Regex = Regex::new(r"[A-Z].*?\.").unwrap();
}

pub fn train(order: usize) -> MarxovChain {
    let text = String::from_utf8_lossy(&fs::read("manifesto.txt").unwrap())
        .chars()
        .map(|x| {
            match x {
                '“' => '"',
                '”' => '"',
                '’' => '\'',
                '–' => '-',
                u => u,
            }
        })
        .filter(|x| {
            x.is_ascii()
        })
        .collect();

    let mut chain = Chain::new(order);

    chain.train(text);

    chain
}

pub fn get(chain: &MarxovChain) -> Option<String> {
    let data = &chain.generate();
    let string: String = data.into_iter().collect();

    match SENT.captures(&string) {
        Some(x) => Some(x[0].to_owned()),
        None => None,
    }
}