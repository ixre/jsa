#![feature(proc_macro_hygiene, decl_macro)]
#![allow(unused)]
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;

use std::path::PathBuf;

use clap::{App, Arg};
use rocket::Config;
use rocket::config::Environment;
use rocket::http::ContentType;
use rocket::logger::LoggingLevel;
use rocket::Request;
use rocket::Rocket;

use jsa::http;

fn rocket(address: &str, port: u16) -> rocket::Rocket {
    let mut cfg = Config::build(Environment::Production)
        .address(address)
        .port(port)
        .secret_key("JFANSeDrbcxXueohPvvcEal0+Fh6bwtQ++6v1wAQDm8=")
        .finalize()
        .unwrap();
    cfg.set_log_level(LoggingLevel::Off);
    let r = rocket::custom(cfg);
    http::mount_routes(r)
}

fn main() {
    let args = [
        Arg::with_name("data-dir")
            .takes_value(true)
            .default_value("./data"),
        Arg::with_name("port")
            .short("p")
            .takes_value(true)
            .default_value("8302"),
        Arg::with_name("debug").short("d").takes_value(false),
    ];
    let matches = App::new("jsa").args(&args).get_matches();
    let data = matches.value_of("data-dir").unwrap();
    let debug = matches.is_present("debug");
    let _port = matches.value_of("port").unwrap();
    let port: u16 = _port.trim().parse().unwrap();
    let addr: String = "0.0.0.0:".to_string() + _port;
    jsa::init(data, debug);
    println!("[ Jsa][ Serve]: serve on port {}", _port);
    rocket("0.0.0.0", port).launch();
}
