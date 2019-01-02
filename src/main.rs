#![feature(proc_macro_hygiene, decl_macro)]
#![allow(unused)]
#[macro_use]
extern crate rocket;

use std::path::PathBuf;

use clap::{App, Arg};
use iron::prelude::Iron;
use jsa::http::entry;
use jsa::http2::Entry;
use rocket::config::Environment;
use rocket::http::ContentType;
use rocket::Config;
use rocket::Request;
use rocket::Rocket;

fn rocket(address: &str, port: u16) -> rocket::Rocket {
    let cfg = Config::build(Environment::Production)
        .address(address)
        .port(port)
        .finalize()
        .unwrap();
    rocket::custom(cfg).mount("/", routes![entry::index, entry::all_request])
}

fn main() {
    let args = [
        Arg::with_name("conf")
            .short("c")
            .takes_value(true)
            .default_value("./conf"),
        Arg::with_name("port")
            .short("p")
            .takes_value(true)
            .default_value("8302"),
        Arg::with_name("debug").short("d").takes_value(false),
    ];
    let matches = App::new("jsa").args(&args).get_matches();
    let conf = matches.value_of("conf").unwrap();
    let debug = matches.is_present("debug");
    let _port = matches.value_of("port").unwrap();
    let port: u16 = _port.trim().parse().unwrap();
    let addr: String = "0.0.0.0:".to_string() + _port;
    //let entry = Entry::new(conf.to_string(), debug);
    //let _server = Iron::new(entry).http(addr).unwrap();
    jsa::init(conf, debug);
    rocket("0.0.0.0", port).launch();
    println!("[ Jsa][ Serve]: serve on port {}", _port);
}
