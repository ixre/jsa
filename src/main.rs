#![feature(proc_macro_hygiene, decl_macro)]
#![allow(unused)]
#[macro_use]
extern crate rocket;

use std::path::PathBuf;

use clap::{App, Arg};
use rocket::Config;
use rocket::config::Environment;
use rocket::http::ContentType;
use rocket::logger::LoggingLevel;
use rocket::Request;
use rocket::Rocket;
use rocket_contrib::serve::StaticFiles;

use jsa::http::index;

fn rocket(address: &str, port: u16) -> rocket::Rocket {
    let mut cfg = Config::build(Environment::Production)
        .address(address)
        .port(port)
        .secret_key("JFANSeDrbcxXueohPvvcEal0+Fh6bwtQ++6v1wAQDm8=")
        .finalize()
        .unwrap();
    cfg.set_log_level(LoggingLevel::Off);
    rocket::custom(cfg)
        .mount(
            "/",
            routes![index::index,
            index::all,
             index::favicon,
              index::login],
        )
        .mount("/static", StaticFiles::from("./static"))
        //.mount("/login",routes![console::index,console::login,console::index2])
        .mount("/console", StaticFiles::from("./static/app"))
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
    jsa::init(conf, debug);
    println!("[ Jsa][ Serve]: serve on port {}", _port);
    rocket("0.0.0.0", port).launch();
}
