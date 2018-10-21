extern crate clap;
extern crate iron;
extern crate jsa;

use clap::{App, Arg};
use iron::prelude::Iron;
use jsa::http::Entry;

fn main() {
    let args = [
        Arg::with_name("conf").short("c")
            .takes_value(true).default_value("./"),
        Arg::with_name("port").short("p")
            .takes_value(true).default_value("8302"),
        Arg::with_name("debug").short("d")
            .takes_value(false),
    ];
    let matches = App::new("jrd").args(&args).get_matches();
    let conf = matches.value_of("conf").unwrap();
    let debug = matches.is_present("debug");
    let _port = matches.value_of("port").unwrap();
    let addr: String = "0.0.0.0:".to_string() + _port;
    let entry = Entry::new(conf.to_string(), debug);
    let _server = Iron::new(entry).http(addr).unwrap();
    println!("[ Jrd][ Serve]: serve on port {}", _port);
}

