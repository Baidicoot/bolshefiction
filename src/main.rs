#[macro_use]
extern crate lazy_static;
extern crate hyper;
extern crate bolshefiction;
extern crate regex;

use hyper::{Body, Request, Response, Server};
use hyper::service::service_fn_ok;
use hyper::rt::{self, Future};
use regex::{Regex, Captures};
use std::fs;

lazy_static! {
    static ref REPLACER: Regex = Regex::new(r"\$phrase\$").unwrap();
}

fn main() {
    let addr = ([127, 0, 0, 1], 8080).into();

    let chain = bolshefiction::train(8);

    let template = fs::read_to_string("template.html").unwrap();

    println!("Initialized Processes.");

    let server = Server::bind(&addr)
        .serve(move || {
            let a = chain.clone();
            let t = template.clone();

            service_fn_ok(move |req: Request<Body>| {
                let phrase = bolshefiction::get(&a).unwrap();
                
                let response = REPLACER.replace_all(&t, |_: &Captures| {
                        phrase.to_string()
                    }).to_string();

                Response::new(Body::from(response))
            })
        })
        .map_err(|e| eprintln!("server error: {}", e));

    println!("Listening on http://{}", addr);

    rt::run(server);
}
