#![feature(proc_macro_hygiene, decl_macro)]
#![feature(never_type)]
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;

use std::sync::Mutex;

use crate::jsa::ItemManager;

pub use self::user::User;
pub use self::user::UserFlag;

pub mod http;
mod jsa;
mod user;

// App name
const NAME: &str = "JSA";
// App version
const VERSION: &str = "1.0";

lazy_static! {
    static ref MANAGER: Mutex<Option<ItemManager>> = Mutex::new(None);
    static ref DEBUG_MODE: Mutex<bool> = Mutex::new(false);
    static ref CONF_PATH: Mutex<String> = Mutex::new(String::from("./conf"));
}

pub fn init(conf: &str, debug: bool) {
    *MANAGER.lock().unwrap() = Some(ItemManager::new(conf.to_owned()).unwrap());
    *DEBUG_MODE.lock().unwrap() = debug;
    *CONF_PATH.lock().unwrap() = conf.to_string();
}
